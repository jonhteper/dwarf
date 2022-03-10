use std::io;
use std::num::ParseFloatError;
use std::sync::Arc;
use eframe::epi::{App, Frame};
use eframe::{NativeOptions, run_native};
use eframe::epi::backend::FrameData;
use egui::{Button, CentralPanel, Color32, Rgba, RichText, TextEdit, Vec2};
use egui::plot::Text;
use egui::style::Spacing;


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

}

impl App for SatApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        frame.set_window_size(Vec2::from([300.0, 400.0]));

        let mut style = egui::style::Style::default();
        //style.visuals.selection.bg_fill = Color32::from_rgb_additive(255,150,57);
        style.visuals.selection.bg_fill = Color32::from_rgb_additive(234,137,52);
        style.visuals.selection.stroke.color = Color32::BLACK;
        style.visuals.dark_mode = true;

        CentralPanel::default().show(ctx, |ui| {
            ui.set_style(Arc::new(style));

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

            ui.separator();
            ui.label("");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.mode, RadioOption::Billed, "Factura");
                ui.selectable_value(&mut self.mode, RadioOption::Standard, "Estándar");
                ui.selectable_value(&mut self.mode, RadioOption::Revert, "Invertido");
            });

            ui.horizontal(|ui| {
                ui.label(" $");
                ui.add(TextEdit::singleline(&mut self.input).hint_text(&self.mode.message()));
                //ui.text_edit_singleline(&mut self.input);
            });
            ui.label(RichText::new(&self.err_msg).color(Color32::RED));
            ui.horizontal(|ui|{
                let  r_btn_clear = ui.add(Button::new("Limpiar"));
                if r_btn_clear.clicked() {
                    self.clear_all()
                }
                let  r_btn_calc = ui.add(Button::new("Calcular"));
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
                        },
                        _ => self.input = "".to_string(),
                    }
                }
            });

        });
    }

    fn name(&self) -> &str {
        "Mis Facturas"
    }
}

impl SatApp {
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
    }

    fn calc(&mut self) -> Result<(), ParseFloatError> {
        self.n_input = self.input.trim().parse::<f64>()?;
        match self.mode {
            RadioOption::Standard => self.standard(),
            RadioOption::Billed => self.billed(),
            RadioOption::Revert => self.revert(),
        }

        Ok(())
    }

    fn billed(&mut self)  {
        let n = self.n_input * 100.0 / 98.75;
        self.iva = n*0.16;
        self.isr = n*0.0125;
        self.total = n + self.iva - self.isr;
        self.free = self.total - self.iva;
    }

    fn standard(&mut self) {
        let n = self.n_input;
        self.iva = n*0.16;
        self.isr = n*0.0125;
        self.total = n + self.iva - self.isr;
        self.free = self.total - self.iva;
    }
    fn revert(&mut self) {
        let n = self.n_input * 87.1459 / 100.0;
        self.iva = n *0.16;
        self.isr = n *0.0125;
        self.total = n + self.iva - self.isr;
        self.free = n;
    }
}


fn main() {
    let app = SatApp::default();
    let win_option = NativeOptions::default();

    run_native(Box::new(app), win_option);
}
