use chrono::Utc;
use eframe::epi::App;
use eframe::{run_native, NativeOptions};
use egui::{Button, CentralPanel, Color32, RichText, TextEdit, Vec2};
use std::num::ParseFloatError;
use std::sync::Arc;
use std::{fs, io};

use egui::style::Style;
use egui::Response;
use native_dialog::MessageType;

#[derive(Debug, PartialEq)]
enum RadioOption {
    Standard,
    Billed,
    Revert,
}

impl Default for RadioOption {
    fn default() -> Self {
        Self::Standard
    }
}

impl RadioOption {
    fn message(&self) -> String {
        match self {
            RadioOption::Standard => "Escriba la entrada de dinero".to_string(),
            RadioOption::Billed => "¿Cuánto dinero necesitas?".to_string(),
            RadioOption::Revert => "¿De cuánto fue el depósito?".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Step {
    Start,
    Calc,
    Save,
}

impl Default for Step {
    fn default() -> Self {
        Self::Start
    }
}

#[derive(Default)]
struct SatApp {
    input: String,
    mode: RadioOption,
    err_msg: String,
    iva: f64,
    isr: f64,
    free: f64,
    total: f64,
    n_input: f64,
    step: Step,
}

impl App for SatApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        frame.set_window_size(Vec2::from([300.0, 400.0]));

        let style = SatApp::style();

        CentralPanel::default().show(ctx, |ui| {
            ui.set_style(Arc::new(style));

            // Results
            self.ui_grid(ui);

            ui.separator();

            // Mode selector
            self.ui_options(ui);

            // input
            ui.horizontal(|ui| {
                ui.label(" $");
                ui.add(TextEdit::singleline(&mut self.input).hint_text(&self.mode.message()));
            });

            // Error box
            ui.label(RichText::new(&self.err_msg).color(Color32::RED));

            // buttons area
            self.ui_buttons(ui);
        });
    }

    fn name(&self) -> &str {
        "Mis Facturas"
    }
}

impl SatApp {
    fn style() -> Style {
        let mut style = egui::style::Style::default();
        style.visuals.selection.bg_fill = Color32::from_rgb_additive(234, 137, 52);
        style.visuals.selection.stroke.color = Color32::BLACK;
        style.visuals.dark_mode = true;

        style
    }

    fn ui_grid(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("my_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Entrada");
                ui.label(format!("$ {}", &self.n_input));
                ui.end_row();

                ui.label("IVA");
                ui.label(format!("$ {}", &self.iva));
                ui.end_row();

                ui.label("ISR");
                ui.label(format!("$ {}", &self.isr));
                ui.end_row();

                ui.label("Libre Impuestos");
                ui.label(format!("$ {}", &self.free));
                ui.end_row();

                ui.label("Total");
                ui.label(format!("$ {}", &self.total));
                ui.end_row();
            });
    }

    fn ui_options(&mut self, ui: &mut egui::Ui) {
        ui.label("");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, RadioOption::Billed, "Factura");
            ui.selectable_value(&mut self.mode, RadioOption::Standard, "Estándar");
            ui.selectable_value(&mut self.mode, RadioOption::Revert, "Invertido");
        });
    }

    fn __ui_buttons(ui: &mut egui::Ui) -> (Response, Response, Response) {
        (
            ui.add(Button::new("Limpiar")),
            ui.add(Button::new("Calcular")),
            ui.add(Button::new("Guardar")),
        )
    }

    fn ui_buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let (r_btn_clear, r_btn_calc, r_btn_save) = SatApp::__ui_buttons(ui);

            if r_btn_clear.clicked() {
                self.clear_all()
            }

            if r_btn_calc.clicked() {
                if self.input.len() == 0 || self.input == "0".to_string() {
                    return;
                }
                self.clear();
                match self.calc() {
                    Err(err) => {
                        self.clear_all();
                        println!("{:?}", &err);
                        self.err_msg = "Error. Entrada no válida".to_string();
                    }
                    _ => self.input = "".to_string(),
                }
            }

            if r_btn_save.clicked() && self.step <= Step::Calc {
                match self.save() {
                    Err(err) => {
                        println!("error: {:?}", &err);
                        let _ = native_dialog::MessageDialog::new()
                            .set_type(MessageType::Error)
                            .set_title("Error")
                            .set_text(&format!("Ha ocurrido un errror inesperado: {:#?}", err))
                            .show_alert()
                            .unwrap();
                    }
                    _ => {}
                }
            }
        });
    }

    fn clear(&mut self) {
        self.err_msg = "".to_string();
        self.iva = 0.0;
        self.isr = 0.0;
        self.free = 0.0;
        self.total = 0.0;
        self.n_input = 0.0;
    }

    fn clear_all(&mut self) {
        self.input = "".to_string();
        self.mode = RadioOption::default();
        self.clear();
        self.step = Step::default();
    }

    fn calc(&mut self) -> Result<(), ParseFloatError> {
        self.n_input = self.input.trim().parse::<f64>()?;
        match self.mode {
            RadioOption::Standard => self.standard(),
            RadioOption::Billed => self.billed(),
            RadioOption::Revert => self.revert(),
        }

        self.step = Step::Calc;

        Ok(())
    }

    fn billed(&mut self) {
        let n = self.n_input * 100.0 / 98.75;
        self.iva = n * 0.16;
        self.isr = n * 0.0125;
        self.total = n + self.iva - self.isr;
        self.free = self.total - self.iva;
    }

    fn standard(&mut self) {
        let n = self.n_input;
        self.iva = n * 0.16;
        self.isr = n * 0.0125;
        self.total = n + self.iva - self.isr;
        self.free = self.total - self.iva;
    }
    fn revert(&mut self) {
        let n = self.n_input * 87.1459 / 100.0;
        self.iva = n * 0.16;
        self.isr = n * 0.0125;
        self.total = n + self.iva - self.isr;
        self.free = n;
    }

    fn save(&mut self) -> Result<(), native_dialog::Error> {
        let path = native_dialog::FileDialog::new()
            .set_location("~/")
            .show_save_single_file()?
            .ok_or(io::Error::new(
                io::ErrorKind::AddrNotAvailable,
                "Error obtaining file path",
            ))?;

        let file_content = format!(
            "Reporte creado: {}\n Entrada: {}\nIVA: {}\nISR: {}\nLibre Impuestos: {}\nTotal: {}",
            Utc::now().naive_local().format("%Y-%m-%d %H:%M:%S"),
            &self.n_input,
            &self.iva,
            &self.isr,
            &self.free,
            &self.total,
        );

        let _ = fs::write(&path, &file_content)?;

        let _ = native_dialog::MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("Reporte Guardado")
            .set_text(&format!(
                "Guardado en {}",
                path.to_str().unwrap_or("Unknown")
            ))
            .show_alert()?;

        self.step = Step::Save;
        Ok(())
    }
}

fn main() {
    let app = SatApp::default();
    let win_option = NativeOptions::default();

    run_native(Box::new(app), win_option);
}
