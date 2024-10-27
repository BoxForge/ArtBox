use eframe::egui;

pub struct Metadata {
    name: String,
    created_date: String,
    updated_date: String,
    tags: Vec<String>,
}

impl Metadata {
    pub fn new() -> Self {
        Metadata {
            name: String::from("Lorem Ipsum"),
            created_date: String::from("09-99-9999"),
            updated_date: String::from("99-99-9999"),
            tags: vec![String::from("Dorem"), String::from("Dolem")]
        }
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            let (response, painter) = ui.allocate_painter(egui::vec2(100.0, 100.0), egui::Sense::hover());
            let preview_rect = response.rect;
            painter.rect_filled(preview_rect, egui::Rounding::same(5.0), egui::Color32::from_rgb(200, 200, 200));
            ui.add_space(10.0);
            ui.label("Sprite Preview");

            ui.add_space(10.0);
            let (response, painter) = ui.allocate_painter(egui::vec2(100.0, 100.0), egui::Sense::hover());
            let palette_rect = response.rect;
            let center = palette_rect.center();
            painter.circle_filled(center, 50.0, egui::Color32::from_rgb(150, 100, 200));
            ui.add_space(10.0);
            ui.label("Color Wheel");

            ui.add_space(10.0);
            ui.group(|ui| {
                ui.label(format!("Name: {}", self.name));
                ui.label(format!("Created: {}", self.created_date));
                ui.label(format!("Updated: {}", self.updated_date));
                ui.label(format!("Tags: {}", self.tags.join(", ")));
            });
        });
    }
}
