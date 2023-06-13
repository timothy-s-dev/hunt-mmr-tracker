use std::fs;
use std::sync::Mutex;
use lazy_static::lazy_static;
use regex::Regex;
use crate::config::Config;
use crate::player_data::PlayerData;

lazy_static! {
    static ref DATA_POINT_KEY_REGEX: Regex = Regex::new(r"_(\d+)_(\d+)_").unwrap();
}

pub fn process_file(config_mutex: &Mutex<Config>) -> Result<Vec<PlayerData>, String> {
    let config = config_mutex.lock().unwrap();

    let xml_string_result = fs::read_to_string(&config.file_path);
    if let Err(msg) = xml_string_result {
        return Err(format!("Error loading attributes.xml: {}", msg));
    }
    let xml_string = xml_string_result.unwrap();

    let doc_result = roxmltree::Document::parse(&*xml_string);
    if let Err(msg) = doc_result {
        return Err(format!("Error parsing attributes.xml: {}", msg));
    }
    let doc = doc_result.unwrap();

    let mut player_data: Vec<PlayerData> = Vec::new();
    for player_name in &config.player_names {
        let mut keys: Vec<DataPointKey> = doc.descendants()
            .filter_map(|x| {
                match x.attribute("value") {
                    Some(y) => {
                        if y.to_string().to_lowercase() == player_name.to_lowercase() {
                            if let Some(name) = x.attribute("name") {
                                Some(DataPointKey::parse(name))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    },
                    None => None
                }
            })
            .collect();
        keys.sort_by_key(|x| x.i1);
        keys.reverse();

        let mmrs: Vec<String> = keys.iter().map(|key| {
            let mmr_name = generate_mmr_name(&key);
            let mmr_node = doc.descendants().find(|x|
                match x.attribute("name") {
                    Some(y) => y == mmr_name,
                    None => false
                } &&
                    match x.attribute("value") {
                        Some(_) => true,
                        None => false
                    }
            );
            match mmr_node {
                Some(x) => x.attribute("value").unwrap().to_string(),
                None => format!("Could not find mmr ({:?})", key)
            }
        }).collect();

        player_data.push(PlayerData {
            name: player_name.to_string(),
            mmr_history: mmrs.iter().map(|x| x.parse::<u32>().unwrap()).collect()
        })
    }
    Ok(player_data)
}

#[derive(Debug)]
struct DataPointKey {
    i1: i32,
    i2: i32,
}

impl DataPointKey {
    fn parse(label: &str) -> DataPointKey {
        let caps = DATA_POINT_KEY_REGEX.captures(label).unwrap();
        DataPointKey {
            i1: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            i2: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        }
    }
}

fn generate_mmr_name(key: &DataPointKey) -> String {
    format!("MissionBagPlayer_{}_{}_mmr", key.i1, key.i2)
}