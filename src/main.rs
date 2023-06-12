extern crate chrono;
use std::fs;
use std::io::stdin;
use std::path::Path;
use notify::{Watcher, RecursiveMode, Result};
use regex::Regex;
use lazy_static::lazy_static;
use colored::{Colorize};
use chrono::offset::Local;
use chrono::DateTime;
use std::time::SystemTime;
use crate::config::Config;

mod config;

lazy_static! {
    static ref DATA_POINT_KEY_REGEX: Regex = Regex::new(r"_(\d+)_(\d+)_").unwrap();
}

fn main() -> Result<()> {
    let config = config::load_config();

    let watcher_config = config.clone();
    let debounced_process = fns::debounce(
        move |_| process_file(&watcher_config),
        std::time::Duration::from_secs(1)
    );

    let mut watcher = notify::recommended_watcher(move |res| {
        match res {
            Ok(_) => debounced_process.call(()),
            Err(e) => println!("file watch error: {:?}", e),
        }
    })?;

    watcher.watch(Path::new(&config.file_path), RecursiveMode::NonRecursive)
        .expect("Error watching file");

    process_file(&config);

    stdin().read_line(&mut "".to_string()).expect("Error receiving input");

    Ok(())
}

fn process_file(config: &Config) {
    let system_time: DateTime<Local> = SystemTime::now().into();
    print!("\x1B[2J\x1B[1;1H");
    println!("Hunt MMR Tracker");
    println!("Last Update @ {}", system_time.format("%r"));
    println!("Press 'Enter' to close.");
    println!();

    let xml_string = fs::read_to_string(&config.file_path).unwrap();
    let doc = roxmltree::Document::parse(&*xml_string).unwrap();

    for player_name in &config.player_names {
        let mut keys: Vec<DataPointKey> = doc.descendants()
            .filter(|x| {
                match x.attribute("value") {
                    Some(y) => y.to_string().to_lowercase() == player_name.to_lowercase(),
                    None => false
                }
            })
            .map(|x| DataPointKey::parse(x.attribute("name").unwrap()))
            .collect();
        keys.sort_by_key(|x| x.i1);
        keys.reverse();

        let mmrs: Vec<String> = keys.iter().map(|key| {
            let mmr_name = generate_mmr_name(&key);
            let mmr_node = doc.descendants().find(|x| {
                match x.attribute("name") {
                    Some(y) => y == mmr_name,
                    None => false
                }
            });
            match mmr_node {
                Some(x) => x.attribute("value").unwrap().to_string(),
                None => format!("Could not find mmr ({:?})", key)
            }
        }).collect();
        let colored_mmrs: Vec<String> = mmrs
            .iter()
            .enumerate()
            .map(|(i, mmr)| {
                if i == mmrs.len() - 1 { mmr.to_string() }
                else { mmr.truecolor(100, 100, 100).to_string() }
            })
            .collect();
        let mmr_string = colored_mmrs.join(", ");

        println!("{}: {}", player_name.bold(), mmr_string);
    }
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