use eframe::egui;

pub struct TitleBar {
    title: String,
    selected_tool: Option<String>,
    settings: Option<String>,
    icon: Option<String>,
}

impl TitleBar {
    pub fn new() -> Self {
        TitleBar {
            title: "ArtBox".to_string(),
            selected_tool: None,
            settings: None,
            icon: None
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading(&self.title);
            let _tool_button = ui.button("Select Tool");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
               let _star_button = ui.button("★");
               let _settings_button = ui.button("⚙"); 
            });
        });
    }
}


