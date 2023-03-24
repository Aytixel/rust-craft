use std::fs::File;
use std::io::BufReader;

use serde_json::Value;

pub struct VersionInfo {
    pub name: String,
    pub protocol: u32,
    pub world_version: u32,
    pub ressource_pack_version: u32,
    pub datapack_version: u32,
}

impl VersionInfo {
    pub fn new() -> Result<Self, String> {
        let file = File::open("./version.json")
            .map_err(|e| format!("Can't open the version file. {e}"))?;
        let reader = BufReader::new(file);
        let version_file: Value = serde_json::from_reader(reader)
            .map_err(|e| format!("Can't parse the version file. {e}"))?;

        fn to_err<T>(option: Option<T>) -> Result<T, String> {
            option.ok_or_else(|| "Wrong version file format".to_string())
        }

        Ok(Self {
            name: to_err(version_file["name"].as_str())?.to_string(),
            protocol: to_err(version_file["protocol_version"].as_u64())? as u32,
            world_version: to_err(version_file["world_version"].as_u64())? as u32,
            ressource_pack_version: to_err(version_file["pack_version"]["resource"].as_u64())?
                as u32,
            datapack_version: to_err(version_file["pack_version"]["data"].as_u64())? as u32,
        })
    }
}
