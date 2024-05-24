#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use csv::ReaderBuilder;
use serde::Deserialize;
use eframe::{egui, App, Frame};
use std::error::Error;
use std::time::{Duration, Instant};
use rand::Rng;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
struct LedCoordinate {
    x: f64,
    y: f64,
}

#[derive(Debug, Deserialize)]
struct RunRace {
    timestamp: DateTime<Utc>,
    x_led: f64,
    y_led: f64,
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

fn read_run_race(file_path: &str) -> Result<Vec<RunRace>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let mut run_race_data = Vec::new();
    for result in rdr.deserialize() {
        let record: RunRace = result?;
        run_race_data.push(record);
    }
    Ok(run_race_data)
}

struct PlotApp {
    coordinates: Vec<LedCoordinate>,
    run_race_data: Vec<RunRace>,
    start_time: Instant,
    start_datetime: DateTime<Utc>,
    current_index: usize,
}

impl PlotApp {
    fn new(coordinates: Vec<LedCoordinate>, run_race_data: Vec<RunRace>) -> Self {
        Self {
            coordinates,
            run_race_data,
            start_time: Instant::now(),
            start_datetime: Utc::now(),
            current_index: 0,
        }
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

        let current_time = self.start_datetime + Duration::from_secs(self.start_time.elapsed().as_secs());

        if self.current_index < self.run_race_data.len() {
            if current_time >= self.run_race_data[self.current_index].timestamp {
                self.current_index += 1;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            for coord in &self.coordinates {
                let norm_x = ((coord.x - min_x) / width) as f32 * ui.available_width();
                let norm_y = ui.available_height() - (((coord.y - min_y) / height) as f32 * ui.available_height());

                let mut color = egui::Color32::BLACK;

                for i in 0..self.current_index {
                    if self.run_race_data[i].x_led == coord.x && self.run_race_data[i].y_led == coord.y {
                        let mut rng = rand::thread_rng();
                        color = egui::Color32::from_rgb(
                            rng.gen_range(0..=255),
                            rng.gen_range(0..=255),
                            rng.gen_range(0..=255),
                        );
                        break;
                    }
                }

                painter.rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(norm_x, norm_y),
                        egui::vec2(20.0, 20.0),
                    ),
                    egui::Rounding::same(0.0),
                    color,
                );
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let coordinates = read_coordinates("led_coords.csv").expect("Error reading CSV");
    let run_race_data = read_run_race("run-race.csv").expect("Error reading CSV");

    let app = PlotApp::new(coordinates, run_race_data);

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "LEDS",
        native_options,
        Box::new(|_cc| Box::new(app)),
    )
}
