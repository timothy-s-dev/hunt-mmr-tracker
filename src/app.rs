use std::sync::{Arc, Mutex};
use chrono::{DateTime, Local};
use egui::{Color32, RichText};
use crate::player_data::PlayerData;

const FONT_SIZE: f32 = 32.0;

pub struct AppState<> {
    player_data: Arc<Mutex<Result<Vec<PlayerData>, String>>>,
    last_update: Arc<Mutex<DateTime<Local>>>
}

impl AppState {
    pub fn new(player_data: Arc<Mutex<Result<Vec<PlayerData>, String>>>, last_update: Arc<Mutex<DateTime<Local>>>) -> AppState {
        AppState { player_data, last_update }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let player_data = self.player_data.lock().unwrap();
        let last_update = self.last_update.lock().unwrap();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Ok(player_data) = &*player_data {
                for player in player_data {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(format!("{}: ", player.name)).size(FONT_SIZE).strong());
                        for (i, mmr) in player.mmr_history.iter().enumerate() {
                            if i == player.mmr_history.len() - 1 {
                                ui.label(RichText::new(format!("{}", mmr)).size(FONT_SIZE));
                            } else {
                                ui.label(RichText::new(format!("{}, ", mmr))
                                    .color(Color32::from_rgb(100, 100, 100)).size(FONT_SIZE));
                            }
                        }
                    });
                }
            }
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(RichText::new(
                    format!("Last updated @ {}", last_update.format("%r"))
                ).color(Color32::from_rgb(100, 100, 100)));
            });
        });
    }
}

