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
        egui::CentralPanel::default().show(ctx, |ui| {
            self.title_bar.show(ui);
            self.main_view.show(ui);
            self.metadata.show(ui);
            self.footer.show(ui);
        });
    }
}
