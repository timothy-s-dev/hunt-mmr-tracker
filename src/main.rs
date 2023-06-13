#![windows_subsystem = "windows"]

use std::io::stdin;
use std::path::Path;
use std::sync::{Arc, Mutex};
use chrono::{Local};
use notify::{Watcher, RecursiveMode};

mod config;
mod app;
mod player_data;
mod file_scanner;

fn wait_for_input() {
    stdin().read_line(&mut "".to_string()).unwrap();
}

fn main() -> eframe::Result<()> {
    let config_load_result = config::load_config();
    if let Err(msg) = config_load_result {
        println!("{}", msg);
        wait_for_input();
        return Ok(());
    }
    let config = config_load_result.unwrap();
    let file_path = config.file_path.clone();
    let config_mutex = Mutex::new(config);
    let last_update = Arc::new(Mutex::new(Local::now()));

    let player_data = Arc::new(Mutex::new(
        file_scanner::process_file(&config_mutex)
    ));

    let watcher_player_data = player_data.clone();
    let watcher_last_update = last_update.clone();
    let debounced_process = fns::debounce(
        move |_| {
            let mut player_data = watcher_player_data.lock().unwrap();
            *player_data = file_scanner::process_file(&config_mutex);

            let mut last_update = watcher_last_update.lock().unwrap();
            *last_update = Local::now();
        },
        std::time::Duration::from_secs(1)
    );

    let mut watcher = notify::recommended_watcher(move |res| {
        match res {
            Ok(_) => debounced_process.call(()),
            Err(e) => println!("file watch error: {:?}", e),
        }
    }).unwrap();

    watcher.watch(Path::new(&file_path), RecursiveMode::NonRecursive)
        .expect("Error watching file");

    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2{ x: 500.0, y: 200.0 });
    let egui_player_data = player_data.clone();
    let egui_last_update = last_update.clone();
    eframe::run_native(
        "Hunt MMR Tracker",
        native_options,
        Box::new(|_| Box::new(app::AppState::new(egui_player_data, egui_last_update))),
    )
}
