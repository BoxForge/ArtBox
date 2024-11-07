
use eframe::egui;
// use crate::tools::spritelibrary::sprite_library_view::Item;

pub trait UITrait {
    fn show_ui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.show_title_bar(ui);
            ui.separator();
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_main_panel(ui);
            ui.separator();
        });

        egui::SidePanel::right("right_panel").resizable(false).show(ctx, |ui| {
            self.show_info_panel(ui);
            ui.separator();
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            self.show_footer(ui);
        }); 
    } 
    fn show_title_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("ArtBox");
            if ui.button("CurrentTool").clicked() {
                ui.label("Clicked");
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("★").clicked() {
                    ui.label("Clicked");
                }
                if ui.button("⚙").clicked() {
                    ui.label("Clicked");
                }
            });
        });
    }
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
            ui.group(|ui| {
                ui.add_space(10.0);
                let (response, painter) = ui.allocate_painter(egui::vec2(200.0, 300.0), egui::Sense::hover());
                let rect = response.rect;
                painter.rect_filled(rect, egui::Rounding::same(5.0), egui::Color32::from_rgb(200, 200, 200));
                ui.label("Info Panel"); 
            });
        });
        
    }
    fn show_footer(&self, ui: &mut egui::Ui) {
        ui.label("Footer");
    }
}

