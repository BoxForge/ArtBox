use eframe::egui;
use crate::ui::views::sprite_library_view::Item;

pub trait InfoPanelTrait {
<<<<<<< HEAD
    fn get_item(&self) -> Item;
=======
    fn get_item(&self) -> String;
>>>>>>> f0ece1bd5f62e95ebc0b556f237d01605f49cde3

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

