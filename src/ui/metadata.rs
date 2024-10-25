use eframe::egui;

pub struct Metadata {
    sprite_name: String,
    sprite_dimensions: String,
    sprite_color: String,
    sprite_date: String,
    sprite_tags: String,
}

impl Metadata {
    pub fn new() -> Self {
        Metadata {
            sprite_name: String::from("Name"),
            sprite_dimensions: String::from("Dimensions"),
            sprite_color: String::from("Color"),
            sprite_date: String::from("Date"),
            sprite_tags: String::from("Tags"),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.label("Sprite Name:");
        ui.text_edit_singleline(&mut self.sprite_name);
        ui.label("Sprite Dimensions:");
        ui.text_edit_singleline(&mut self.sprite_dimensions);
        ui.label("Sprite Color:");
        ui.text_edit_singleline(&mut self.sprite_color);
        ui.label("Sprite Date:");
        ui.text_edit_singleline(&mut self.sprite_date);
        ui.label("Sprite Tags:");
        ui.text_edit_singleline(&mut self.sprite_tags);
    }
}
