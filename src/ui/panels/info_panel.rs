use eframe::egui;

pub trait InfoPanelTrait {
    fn get_item(&self) -> String;

    fn show_info_panel(&self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.group(|ui| {
                ui.add_space(10.0);
                let (response, painter) = ui.allocate_painter(egui::vec2(200.0, 300.0), egui::Sense::hover());
                let rect = response.rect;
                painter.rect_filled(rect, egui::Rounding::same(5.0), egui::Color32::from_rgb(200, 200, 200));
                ui.label("Info Panel"); 
            });
        });
        
    }
}

