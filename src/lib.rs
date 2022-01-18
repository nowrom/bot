pub mod discord;
pub mod matrix;

use std::{collections::HashSet, sync::Mutex};

use fuse_rust::{Fuse, FuseProperty, Fuseable};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
// pub mod telegram;
#[derive(Deserialize, Serialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct RomDevice {
    id: String,
}

fn default_resource() -> String {
    "Unknown".to_string()
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Device {
    #[serde(default = "default_resource")]
    name: String,
    #[serde(default = "default_resource")]
    codename: String,
    #[serde(default = "default_resource")]
    brand: String,
    roms: HashSet<RomDevice>,
}

impl Device {
    pub fn new(name: String, codename: String, brand: String, roms: HashSet<RomDevice>) -> Self {
        Self {
            name,
            codename,
            brand,
            roms,
        }
    }

    /// Get a reference to the device's codename.
    pub fn codename(&self) -> &str {
        self.codename.as_ref()
    }

    /// Get a mutable reference to the device's codename.
    pub fn codename_mut(&mut self) -> &mut String {
        &mut self.codename
    }
}

impl Fuseable for Device {
    fn properties(&self) -> Vec<FuseProperty> {
        return vec![
            FuseProperty {
                value: String::from("name"),
                weight: 2.0,
            },
            FuseProperty {
                value: String::from("codename"),
                weight: 2.0,
            },
            FuseProperty {
                value: String::from("brand"),
                weight: 1.0,
            },
        ];
    }

    fn lookup(&self, key: &str) -> Option<&str> {
        return match key {
            "name" => Some(&self.name),
            "codename" => Some(&self.codename),
            "brand" => Some(&self.brand),
            _ => None,
        };
    }
}

impl Device {}

lazy_static! {
    static ref DATA: Mutex<Vec<Device>> = Mutex::new(vec![]);
}

pub async fn update_devices() {
    let text = reqwest::Client::new()
        .get("https://nowrom.deno.dev")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let mut data: Vec<Device> = serde_json::from_str(&text).unwrap();
    let mut devices = DATA.lock().unwrap();
    devices.clear();
    devices.append(&mut data);
}

pub fn get_data() -> Vec<Device> {
    (*&DATA.lock().unwrap()).to_vec()
}

pub fn search(text: String) -> Option<(Device, Vec<Device>)> {
    let data = get_data();

    let fuse = Fuse::default();
    let results = fuse.search_text_in_fuse_list(&text, &data);
    return if results.is_empty() {
        None
    } else {
        let val = data[results[0].index].clone();
        let mut alternatives = vec![];
        for index in 2..=10 {
            if index > results.len() {
                continue;
            };
            alternatives.push(data[results[index].index].clone());
        }
        Some((val, alternatives))
    };
}

pub fn codename(i: String) -> Option<Device> {
    let data = get_data();
    let mut iter = data.iter();
    let search = i.to_lowercase();
    let t = iter.find(|x| *x.codename().to_lowercase() == search);
    t.map(|x| x.clone())
}

pub fn format_device(d: Device, other: Vec<Device>) -> String {
    format!(
        "{} {}\ncodename: `{}`\nroms: {}{}",
        d.brand,
        d.name,
        d.codename,
        d.roms
            .iter()
            .map(|x| { format!("`{}`", x.id) })
            .collect::<Vec<String>>()
            .join(", "),
        if !other.is_empty() {
            format!(
                "\nor did you mean: {}",
                other
                    .iter()
                    .map(|x| { format!("`{}`", x.codename) })
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        } else {
            format!("")
        }
    )
}
