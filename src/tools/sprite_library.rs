use crate::tools::ToolUI;

// -----------------------------------
// Sprite Library UI Component
// -----------------------------------
 
#[derive(Default)]
pub struct SpriteLibraryUI {
    sprite_count: usize,
    selected_sprite: Option<String>,
}

impl SpriteLibraryUI {
    pub fn new() -> Self {
        Self {
            sprite_count: 10,
            selected_sprite: None,
        }
    }
}

impl ToolUI for SpriteLibraryUI {
    
    fn show_main_panel(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut current_row_count = 0;
            ui.horizontal_wrapped(|ui| {
                for i in 0..5 {
                    let button_label = format!("Item {}", i);
                    let button = ui.button(&button_label);

                    if button.clicked() {
                        // TODO: Implement item selection
                    }

                    current_row_count += 1;
                    if current_row_count > 5 {
                        ui.end_row();
                        current_row_count = 0;
                    }
                }
            });
        }); 
        
    } 
     
    fn show_info_panel(&self, ui: &mut egui::Ui) {
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
                let item = Item::new();
                ui.label(format!("Name: {}", item.name));
                ui.label(format!("Created: {}", item.created_date));
                ui.label(format!("Updated: {}", item.updated_date));
                ui.label(format!("Tags: {}", item.tags.join(", ")));
            });
        });} 
     
    fn show_footer(&self, ui: &mut egui::Ui) {
        ui.label("Footer");
    } 
     
}


// -----------------------------------
// Shared Data Structures
// -----------------------------------

pub struct Item {
    name: String,
    created_date: String,
    updated_date: String,
    tags: Vec<String>,
}

impl Item {
    pub fn new() -> Self {
        Item {
            name: String::from("Lorem Ipsum"),
            created_date: String::from("09-99-9999"),
            updated_date: String::from("99-99-9999"),
            tags: vec![String::from("Dorem"), String::from("Dolem")]
        }
    }
}
