use eframe::egui;
use crate::tools::tool_view::ToolView;
use crate::tools::spritelibrary::sprite_library_view::SpriteLibraryView;

pub struct ArtBoxApp {
    active_view: Box<dyn ToolView>
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
        self.active_view.show_ui(ctx);
        
    }
}
