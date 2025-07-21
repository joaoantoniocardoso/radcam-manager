use anyhow::{Context, Result, anyhow};
use chrono::Utc;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::{fs, io};
use tokio_stream::{StreamExt, wrappers::ReadDirStream};
use tracing::*;
use uuid::Uuid;

use crate::CameraActuators;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsDataV0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsDataV1 {
    pub actuators: IndexMap<Uuid, CameraActuators>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum RawSettingsData {
    V0(SettingsDataV0),
    V1(SettingsDataV1),
}

pub trait SettingsDataImpl: std::fmt::Debug + Send + Sync {
    fn get_actuators(&self) -> &IndexMap<Uuid, CameraActuators>;
    fn get_actuators_mut(&mut self) -> &mut IndexMap<Uuid, CameraActuators>;

    fn to_raw(&self) -> RawSettingsData;
}

impl SettingsDataImpl for SettingsDataV1 {
    fn get_actuators(&self) -> &IndexMap<Uuid, CameraActuators> {
        &self.actuators
    }

    fn get_actuators_mut(&mut self) -> &mut IndexMap<Uuid, CameraActuators> {
        &mut self.actuators
    }

    fn to_raw(&self) -> RawSettingsData {
        RawSettingsData::V1(self.clone())
    }
}

impl From<SettingsDataV0> for SettingsDataV1 {
    fn from(_v1: SettingsDataV0) -> Self {
        SettingsDataV1 {
            actuators: IndexMap::default(),
        }
    }
}

#[derive(Debug)]
pub struct Settings {
    path: PathBuf,
    inner: Box<dyn SettingsDataImpl>,
}

impl Settings {
    pub(crate) async fn try_new(
        path: PathBuf,
        actuators: IndexMap<Uuid, CameraActuators>,
    ) -> Result<Self> {
        let settings = Self {
            path,
            inner: Box::new(SettingsDataV1 { actuators }),
        };

        settings.save().await?;

        Ok(settings)
    }

    pub async fn from_path(path: &Path) -> Result<Self> {
        async fn read_inner(path: &Path) -> Result<Settings> {
            let contents = fs::read_to_string(path)
                .await
                .with_context(|| format!("Failed to read backup file: {path:?}"))?;
            let raw: RawSettingsData = serde_json::from_str(&contents)
                .with_context(|| format!("Failed to parse JSON from backup: {path:?}"))?;
            let inner = match raw {
                RawSettingsData::V1(v1) => Box::new(v1),
                RawSettingsData::V0(v0) => {
                    warn!("Migrating settings V0 to V1 from {path:?}");
                    Box::new(SettingsDataV1::from(v0))
                }
            };

            debug!("Loaded settings from file: {path:?}");

            let settings = Settings {
                path: path.to_owned(),
                inner,
            };

            settings.save().await?;

            Ok(settings)
        }

        if path.exists() {
            return read_inner(path).await;
        }

        let dir = path.parent().unwrap_or_else(|| Path::new("."));
        let read_dir = fs::read_dir(dir).await?;
        let mut entries = ReadDirStream::new(read_dir);

        let mut backups = vec![];
        while let Some(entry) = entries.next().await {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                if file_name.to_string_lossy().starts_with("settings.json.") {
                    let path = entry.path();
                    if path.extension().map(|e| e == "bak").unwrap_or(false) {
                        backups.push(entry);
                    }
                }
            }
        }

        if let Some(latest_backup) =
            futures::future::try_join_all(backups.iter().map(|e| async move {
                let meta = e.metadata().await.ok();
                Ok::<_, io::Error>((meta.and_then(|m| m.modified().ok()), e.path()))
            }))
            .await?
            .into_iter()
            .max_by_key(|(mod_time, _)| *mod_time)
            .map(|(_, path)| path)
        {
            return read_inner(&latest_backup).await;
        }

        Err(anyhow!("No settings file or backup found"))
    }

    pub async fn save(&self) -> Result<()> {
        let path = self.path.as_path();
        let settings_file = path.to_string_lossy();

        let raw = self.to_raw();
        let new_contents =
            serde_json::to_string_pretty(&raw).context("Failed to serialize config to JSON")?;

        if path.exists() {
            let current_contents = fs::read_to_string(path).await.with_context(|| {
                format!("Failed to read existing settings file: {settings_file:?}")
            })?;

            if current_contents == new_contents {
                trace!("Settings unchanged, not writing or backing up");
                return Ok(());
            }

            let now = Utc::now().timestamp();
            let backup_path = path.with_file_name(format!("settings.json.{now}.bak"));

            fs::copy(path, &backup_path)
                .await
                .with_context(|| format!("Failed to back up file to {backup_path:?}"))?;
            debug!("Created backup: {backup_path:?}");
        }

        fs::write(path, &new_contents)
            .await
            .with_context(|| format!("Failed to write settings file to {settings_file:?}"))?;

        debug!(
            "Wrote new settings to {settings_file:?}:\n{:#?}",
            self.inner
        );

        Ok(())
    }

    pub fn get_actuators(&self) -> &IndexMap<Uuid, CameraActuators> {
        self.inner.get_actuators()
    }

    pub fn get_actuators_mut(&mut self) -> &mut IndexMap<Uuid, CameraActuators> {
        self.inner.get_actuators_mut()
    }

    pub fn to_raw(&self) -> RawSettingsData {
        self.inner.to_raw()
    }
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use crate::{CameraActuatorsConfig, api::ActuatorsState};

    use super::*;

    #[tokio::test]
    async fn test_migrate_v0_insert_and_persist_actuators() -> Result<()> {
        // Create temp file path
        let tmp_file = NamedTempFile::new()?;
        let path = tmp_file.path().to_path_buf();

        // Step 1: Write a SettingsDataV0 JSON to the file
        let v0 = RawSettingsData::V0(SettingsDataV0);
        let json = serde_json::to_string_pretty(&v0)?;
        fs::write(&path, json).await?;

        // Step 2: Read settings (should auto-migrate to V1)
        let mut settings = Settings::from_path(&path).await?;
        let actuators = settings.get_actuators();
        assert!(
            actuators.is_empty(),
            "Expected empty actuator map from V0 migration"
        );

        // Step 3: Insert a new actuator
        let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, b"uuid.example.com");
        let mut actuators = IndexMap::new();
        actuators.insert(
            uuid,
            CameraActuators {
                config: CameraActuatorsConfig::default(),
                state: ActuatorsState {
                    focus: Some(1.0),
                    zoom: Some(2.0),
                    tilt: Some(3.0),
                },
            },
        );
        *settings.get_actuators_mut() = actuators;

        // Step 4: Save updated settings
        settings.save().await?;

        // Step 5: Reload and verify data persisted
        let settings = Settings::from_path(&path).await?;
        let reloaded = settings.get_actuators();

        assert_eq!(reloaded.len(), 1);
        let loaded = reloaded.get(&uuid).unwrap();
        assert_eq!(loaded.state.focus, Some(1.0));
        assert_eq!(loaded.state.zoom, Some(2.0));
        assert_eq!(loaded.state.tilt, Some(3.0));

        Ok(())
    }
}
