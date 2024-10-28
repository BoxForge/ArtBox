use eframe::egui;
use crate::ui::views as views;
use crate::ui::views::sprite_library_view::SpriteLibraryView;

pub struct ArtBoxApp {
    active_view: Box<dyn views::tool_view::ToolView>
}

impl ArtBoxApp {
    pub fn new() -> Self {
        ArtBoxApp {
            // TODO Add dynamic change to active view
            active_view: Box::new(SpriteLibraryView::new()),
        }
    }
}

impl eframe::App for ArtBoxApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.active_view.show_title_bar(ui);
            ui.separator();
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.active_view.show_main_view(ui);
            ui.separator();
        });

        egui::SidePanel::right("right_panel").resizable(false).show(ctx, |ui| {
            self.active_view.show_info_panel(ui);
            ui.separator();
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            self.active_view.show_footer(ui);
        });
    }
}
