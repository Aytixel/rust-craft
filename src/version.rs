use anyhow::{anyhow, Result};
use async_std::{fs::File, io::ReadExt};
use serde_json::Value;

#[derive(Debug)]
pub struct Version {
    pub id: String,
    pub name: String,
    pub protocol: u32,
    pub world_version: u32,
    pub resource_pack_version: u32,
    pub datapack_version: u32,
}

impl Version {
    pub async fn new() -> Result<Self> {
        let mut file = File::open("./version.json")
            .await
            .map_err(|e| anyhow!("Can't open the version file. {e}"))?;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer).await?;

        let version_file: Value = serde_json::from_slice(&buffer)
            .map_err(|e| anyhow!("Can't parse the version file. {e}"))?;

        fn to_err<T>(option: Option<T>) -> Result<T> {
            option.ok_or_else(|| anyhow!("Wrong version file format"))
        }

        Ok(Self {
            id: to_err(version_file["id"].as_str())?.to_string(),
            name: to_err(version_file["name"].as_str())?.to_string(),
            protocol: to_err(version_file["protocol_version"].as_u64())? as u32,
            world_version: to_err(version_file["world_version"].as_u64())? as u32,
            resource_pack_version: to_err(version_file["pack_version"]["resource"].as_u64())?
                as u32,
            datapack_version: to_err(version_file["pack_version"]["data"].as_u64())? as u32,
        })
    }
}
