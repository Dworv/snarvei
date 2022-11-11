use std::{
    env::var,
    fs::{create_dir_all, read_dir, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use lazy_static::lazy_static;

use crate::{Collection, Config};

// const CONFIG_PATH: PathBuf = Pathbuf::from("/.config/snarvei/");
// const COLLECTIONS_PATH: PathBuf = Pathbuf::from("/.config/snarvei/collections/");

lazy_static! {
    pub static ref HOME: PathBuf = PathBuf::from(var("HOME").unwrap());
    pub static ref CONFIG_PATH: PathBuf = HOME.join(".config/snarvei/");
    pub static ref COLLECTIONS_PATH: PathBuf = CONFIG_PATH.join("collections/");
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
        }
        Err(_) => None,
    }
}

pub fn write_config(config: Config) {
    create_dir_all(&*CONFIG_PATH).unwrap();
    let mut fs = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(CONFIG_PATH.join("config.toml"))
        .unwrap();
    fs.write(&toml::to_string(&config).unwrap().as_bytes()).unwrap();
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
        }
        Err(_) => None,
    }
}

pub fn read_collections() -> Vec<Collection> {
    let mut collections: Vec<Collection> = vec![];
    create_dir_all(&*COLLECTIONS_PATH).unwrap();
    let raw = read_dir(&*COLLECTIONS_PATH).unwrap();
    for entry in raw {
        if let Some(name) = entry
            .unwrap()
            .file_name()
            .into_string()
            .unwrap()
            .strip_suffix(".toml")
        {
            read_collection(&String::from(name)).map(|mut c| {
                c.name = name.to_string();
                collections.push(c);
            });
        }
    }
    collections
}

pub fn write_collection(collection: Collection) {
    create_dir_all(&*COLLECTIONS_PATH).unwrap();
    let mut fs = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(COLLECTIONS_PATH.join(collection.name.clone() + ".toml"))
        .unwrap();
    fs.write(&toml::to_string(&collection).unwrap().as_bytes()).unwrap();
}