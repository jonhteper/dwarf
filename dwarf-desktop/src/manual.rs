use crate::{assets_location, SatApp};
use eframe::epi::{App, Frame};
use egui::plot::{Text, Value};
use egui::{CentralPanel, Context, Label, RichText, TextFormat, Vec2, WidgetText};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use std::{fs, io};

#[derive(Default)]
pub struct Manual;

impl Manual {
    pub fn title(&self) -> &str {
        "Manual de uso"
    }

    pub fn manual_text() -> Result<String, io::Error> {
        let mut path = assets_location();
        path.push("manual.txt");
        fs::read_to_string(path)
    }
}
