mod app;
mod manual;
mod utils;

use crate::utils::assets_location;
use app::SatApp;
use chrono::Utc;
use eframe::epi::{App, IconData};
use eframe::{run_native, NativeOptions};
use egui::style::Style;
use egui::Response;
use egui::{Button, CentralPanel, Color32, RichText, TextEdit, Vec2};
use image::ImageError;
use native_dialog::MessageType;
use std::num::ParseFloatError;
use std::path::PathBuf;
use std::sync::Arc;
use std::{env, fs, io};
use utils::{RadioOption, Step};

// TODO:
//  - crear app multiventana

#[cfg(target_os = "linux")]
fn image_path() -> PathBuf {
    let mut path = assets_location();
    path.push("icon.png");

    path
}

fn icon_data() -> Result<IconData, ImageError> {
    let icon = image::open(image_path())?.to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    Ok(IconData {
        rgba: icon.into_raw(),
        width: icon_width,
        height: icon_height,
    })
}

fn win_options() -> NativeOptions {
    let mut win_option = NativeOptions::default();
    let icon = icon_data().expect("Failed to read icon app");
    win_option.icon_data = Some(icon);

    win_option
}

fn main() {
    let app = SatApp::default();

    run_native(Box::new(app), win_options());
}
