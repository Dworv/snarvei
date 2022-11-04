use std::{env::var, fs::{create_dir_all, read_dir, File}, io::Read, path::PathBuf};

use lazy_static::lazy_static;

use crate::{Collection, Config};

// const CONFIG_PATH: PathBuf = Pathbuf::from("/.config/snarvei/");
// const COLLECTIONS_PATH: PathBuf = Pathbuf::from("/.config/snarvei/collections/");

lazy_static!{
    static ref HOME: PathBuf = PathBuf::from(var("HOME").unwrap());
    static ref CONFIG_PATH: PathBuf = HOME.join(".config/snarvei/");
    static ref COLLECTIONS_PATH: PathBuf = CONFIG_PATH.join("collections/");
}

pub fn read_config() -> Option<Config> {
    let mut file = String::new();
    create_dir_all(&*CONFIG_PATH).unwrap();
    match File::open(CONFIG_PATH.join("config.toml")) {
        Ok(mut f) => {
            f.read_to_string(&mut file).unwrap();
            if let Ok(config) = toml::from_str(&file[..]) {
                Some(config)
            } else {
                None
            }
        },
        Err(_) => None
    }
}

pub fn read_collection(name: &String) -> Option<Collection> {
    let mut file = String::new();
    create_dir_all(&*COLLECTIONS_PATH).unwrap();
    match File::open(COLLECTIONS_PATH.join(String::new() + name + ".toml")) {
        Ok(mut f) => {
            f.read_to_string(&mut file).unwrap();
            if let Ok(config) = toml::from_str(&file[..]) {
                Some(config)
            } else {
                None
            }
        },
        Err(_) => None
    }
}

pub fn read_collections() -> Vec<Collection> {
    let mut collections: Vec<Collection> = vec![];
    create_dir_all(&*COLLECTIONS_PATH).unwrap();
    let raw = read_dir(&*COLLECTIONS_PATH).unwrap();
    for entry in raw {
        entry.unwrap()
            .file_name()
            .into_string()
            .unwrap()
            .strip_suffix(".toml")
            .map( |name| 
                read_collection(&String::from(name)).map(|mut c| 
                    {   
                        c.name = name.to_string();
                        collections.push(c);
                    })
            );
    } 
    collections
}
