use std::fs::{self, read_dir};
use std::io::BufReader;

use hashbrown::hash_map::HashMap;
use quartz_nbt::io::Flavor;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum File {
    Nbt(quartz_nbt::NbtCompound),
    Json(serde_json::Value),
}

#[derive(Debug)]
pub struct Folder {
    folders: HashMap<String, Folder>,
    files: HashMap<String, File>,
}

impl Folder {
    pub fn deserialize_folder(path: &str) -> Result<Folder, String> {
        Folder::deserialize_folder_(path)
    }

    fn deserialize_folder_(path: &str) -> Result<Folder, String> {
        let mut folder = Folder {
            folders: HashMap::new(),
            files: HashMap::new(),
        };

        for file in read_dir(path).unwrap() {
            if let Ok(file) = file {
                if file.file_type().unwrap().is_dir() {
                    folder.folders.insert(
                        file.path()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                        Folder::deserialize_folder_(
                            file.path().into_os_string().as_os_str().to_str().unwrap(),
                        )?,
                    );
                } else {
                    let file_name = file.file_name();
                    let file_name = file_name.to_str().unwrap();
                    let file = fs::File::open(file.path()).map_err(|e| e.to_string())?;
                    let mut reader = BufReader::new(file);

                    if file_name.ends_with(".json") {
                        folder.files.insert(
                            file_name[..file_name.len() - 5].to_string(),
                            File::Json(serde_json::from_reader(reader).map_err(|e| e.to_string())?),
                        );
                    } else if file_name.ends_with(".nbt") {
                        folder.files.insert(
                            file_name[..file_name.len() - 4].to_string(),
                            File::Nbt(
                                quartz_nbt::io::read_nbt(&mut reader, Flavor::GzCompressed)
                                    .map_err(|e| e.to_string())?
                                    .0,
                            ),
                        );
                    }
                }
            }
        }

        Ok(folder)
    }
}

#[derive(Debug)]
pub struct Datapack {
    data: Folder,
    assets: Folder,
}

impl Datapack {
    pub fn new(path: &str) -> Result<Self, String> {
        Ok(Self {
            data: Folder::deserialize_folder((path.to_string() + "data/").as_str())?,
            assets: Folder::deserialize_folder((path.to_string() + "assets/").as_str())?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        println!("{:#?}", Datapack::new("./").unwrap());
    }
}
