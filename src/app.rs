use eframe::egui;
use crate::ui;

pub struct ArtBoxApp {
    title_bar: ui::title_bar::TitleBar,
    main_view: ui::main_view::MainView,
    metadata: ui::metadata::Metadata,
    footer: ui::footer::Footer
}

impl ArtBoxApp {
    pub fn new() -> Self {
        ArtBoxApp {
            title_bar: ui::title_bar::TitleBar::new(),
            main_view: ui::main_view::MainView::new(),
            metadata: ui::metadata::Metadata::new(),
            footer: ui::footer::Footer::new()
        }
    }
}

impl eframe::App for ArtBoxApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.title_bar.show(ui);
            ui.separator();
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.main_view.show(ui);
            ui.separator();
        });

        egui::SidePanel::right("right_panel").resizable(false).show(ctx, |ui| {
            self.metadata.show(ui);
            ui.separator();
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            self.footer.show(ui);
        });
    }
}
