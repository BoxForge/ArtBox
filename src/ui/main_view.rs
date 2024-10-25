use eframe::egui;

pub struct MainView {
    sprite_count: usize,
    pub selected_sprite: Option<String>,
}

impl MainView {
    pub fn new() -> Self {
        MainView {
            sprite_count: 10,
            selected_sprite: None
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
       egui::ScrollArea::vertical().show(ui, |ui| {
          let mut current_row_count = 0;
          ui.horizontal_wrapped(|ui| {
             for i in 0..self.sprite_count {
                let button_label = format!("Sprite {}", i + 1);
                let button = ui.button(&button_label);

                if button.clicked() {
                   self.selected_sprite = Some(button_label);
                }
                current_row_count += 1;
                if current_row_count > 5 {
                   ui.end_row();
                   current_row_count = 0;
                }
             }              
          }) 
       }); 
    }
}
