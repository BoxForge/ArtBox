use eframe::egui;

pub struct MainView {
    pub selected_sprite: Option<String>,
}

impl MainView {
    pub fn new() -> Self {
        MainView {
            selected_sprite: None
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Select Sprite").clicked() {
                self.selected_sprite = Some("Sprite 1".to_string());
            }
        });
    }
}
