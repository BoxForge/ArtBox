use eframe::egui;

pub trait MainViewTrait {
    fn get_item_count(&self) -> usize;
    fn get_item_label(&self, index: usize) -> String;
    fn on_item_selected(&mut self, index: usize);
    fn on_item_clicked(&mut self, index: usize);
    fn show_main_view(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut current_row_count = 0;
            ui.horizontal_wrapped(|ui| {
                for i in 0..self.get_item_count() {
                    let button_label = self.get_item_label(i);
                    let button = ui.button(&button_label);

                    if button.clicked() {
                        self.on_item_clicked(i);
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
}


