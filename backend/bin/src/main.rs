use anyhow::Result;
use tracing::*;

use radcam_manager::{logger, web};

pub mod cli;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    cli::init();

    logger::init(cli::log_path(), cli::is_verbose(), cli::is_tracing());

    info!(
        "{}, version: {}-{}, build date: {}",
        option_env!("CARGO_PKG_NAME").unwrap_or("?"),
        option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0"),
        option_env!("VERGEN_GIT_SHA").unwrap_or("?"),
        option_env!("VERGEN_BUILD_DATE").unwrap_or("?"),
    );
    info!(
        "Starting at {}",
        chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
    );
    debug!("Command line call: {}", cli::command_line_string());
    debug!("Command line input struct call: {}", cli::command_line());

    settings::init(cli::settings_file(), cli::is_reset())
        .await
        .unwrap();

    let mcm_client_startup_task = tokio::spawn(mcm_client::init(cli::mcm_address().await));

    let autopilot_startup_task = tokio::spawn(async move {
        loop {
            if let Err(error) = autopilot::init(
                cli::autopilot_scripts_file(),
                cli::mavlink_connection_string().await,
                cli::mavlink_system_id(),
                cli::mavlink_component_id(),
            )
            .await
            {
                error!("Failed initializing autopilot: {error:?}");
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    web::run(cli::web_server().await, cli::default_api_version()).await;

    autopilot_startup_task.abort();
    mcm_client_startup_task.abort();

    Ok(())
}
