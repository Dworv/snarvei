use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Config {
    //
}

impl Config {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Collection {
    #[serde(skip)]
    pub name: String,
    pub shortcuts: Vec<Shortcut>,
}

impl Collection {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            shortcuts: vec![]
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Shortcut {
    pub trigger: Keybind,
    pub output: Keybind,
}

impl Shortcut {
    pub fn new(tkey: u16, okey: u16) -> Self {
        Self {
            trigger: Keybind::new(tkey),
            output: Keybind::new(okey)
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Keybind {
    pub key: u16,
    pub includes: Vec<u16>,
    pub excludes: Vec<u16>,
}

impl Keybind {
    pub fn new(key: u16) -> Self {
        Self {
            key: key,
            includes: vec![],
            excludes: vec![]
        }
    }
}