use anyhow::Result;

/// Helper function to deserialize a json string into a variable of type `T`. In general,
/// this is a more robust approach than `serde_json::deserialize`.
/// One the improvements is that it would ignore duplicate keys.
pub fn deserialize<T>(json: &str) -> Result<T>
where
    T: for<'de> serde::Deserialize<'de>,
{
    serde_json::from_str::<serde_json::Value>(json)
        .and_then(serde_json::from_value::<T>)
        .map_err(anyhow::Error::msg)
}
