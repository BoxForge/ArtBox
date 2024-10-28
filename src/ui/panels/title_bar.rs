use eframe::egui;

pub trait TitleBarTrait {
<<<<<<< HEAD
    fn get_tool_title(&self) -> &String;
    fn on_tool_selected(&mut self);
    fn on_settings_clicked(&mut self);
    fn on_icon_clicked(&mut self);
=======
    fn get_tool_title(&self) -> String;
    fn set_tool_title(&mut self, title: String);
    fn on_tool_selected(&mut self);
    fn on_settings_clicked(&mut self);
    fn on_icon_clicked(&mut self);

    fn show_title_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("ArtBox");
            if ui.button(self.get_tool_title()).clicked() {
                self.on_tool_selected();
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("★").clicked() {
                    self.on_icon_clicked();
                }
                if ui.button("⚙").clicked() {
                    self.on_settings_clicked();
                }
            });
        });
    }
}

pub struct TitleBar {
    title: String,
    selected_tool: Option<String>,
    settings: Option<String>,
    icon: Option<String>,
}
>>>>>>> f0ece1bd5f62e95ebc0b556f237d01605f49cde3

    fn show_title_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("ArtBox");
            if ui.button(self.get_tool_title()).clicked() {
                self.on_tool_selected();
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("★").clicked() {
                    self.on_icon_clicked();
                }
                if ui.button("⚙").clicked() {
                    self.on_settings_clicked();
                }
            });
        });
    }
}

