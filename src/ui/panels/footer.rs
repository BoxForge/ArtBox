use eframe::egui;

pub struct Footer {}

impl Footer {
    pub fn new() -> Self {
        Footer {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.label("Footer");
    }
}
