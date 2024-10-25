use eframe::egui;

pub struct TitleBar {
    title: String,
    selected_tool: Option<String>,
}

impl TitleBar {
    pub fn new() -> Self {
        TitleBar {
            title: "ArtBox".to_string(),
            selected_tool: None,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(&self.title);
            if ui.button("Select Tool").clicked() {
                self.selected_tool = Some("Sprite Library".to_string());
            }
        });
    }
}


