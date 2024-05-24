#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use csv::ReaderBuilder;
use serde::Deserialize;
use eframe::{egui, App, Frame};
use std::error::Error;

#[derive(Debug, Deserialize)]
struct LedCoordinate {
    x: f64,
    y: f64,
}

fn read_coordinates(file_path: &str) -> Result<Vec<LedCoordinate>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let mut coordinates = Vec::new();
    for result in rdr.deserialize() {
        let record: LedCoordinate = result?;
        coordinates.push(record);
    }
    Ok(coordinates)
}

struct PlotApp {
    coordinates: Vec<LedCoordinate>,
}

impl PlotApp {
    fn new(coordinates: Vec<LedCoordinate>) -> Self {
        Self { coordinates }
    }
}

impl App for PlotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("my_layer")));

        let (min_x, max_x) = self.coordinates.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), coord| {
            (min.min(coord.x), max.max(coord.x))
        });
        let (min_y, max_y) = self.coordinates.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), coord| {
            (min.min(coord.y), max.max(coord.y))
        });

        let width = max_x - min_x;
        let height = max_y - min_y;

        egui::CentralPanel::default().show(ctx, |ui| {
            for coord in &self.coordinates {
                let norm_x = ((coord.x - min_x) / width) as f32 * ui.available_width();
                let norm_y = ((coord.y - min_y) / height) as f32 * ui.available_height();

                painter.rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(norm_x, norm_y),
                        egui::vec2(20.0, 20.0),
                    ),
                    egui::Rounding::same(0.0),
                    egui::Color32::BLACK,
                );
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let coordinates = read_coordinates("led_coords.csv").expect("Error reading CSV");

    let app = PlotApp::new(coordinates);

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "LED Coordinates",
        native_options,
        Box::new(|_cc| Box::new(app)),
    )
}
