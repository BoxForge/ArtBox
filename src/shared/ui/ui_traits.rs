
use eframe::egui;
use crate::ui::views::sprite_library_view::Item;

pub trait TitleBarTrait {
    fn get_tool_title(&self) -> &String;
    fn on_tool_selected(&mut self);
    fn on_settings_clicked(&mut self);
    fn on_icon_clicked(&mut self);
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

pub trait InfoPanelTrait {
    fn get_item(&self) -> Item;
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

pub trait FooterTrait {

    fn show_footer(&self, ui: &mut egui::Ui) {
        ui.label("Footer");
    }
}
