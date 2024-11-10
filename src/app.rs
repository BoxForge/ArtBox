use eframe::egui;
use crate::tools::ToolBox;

pub struct ArtBoxApp {
    toolbox: ToolBox
}

impl ArtBoxApp {
    pub fn new(toolbox: ToolBox) -> Self {
        ArtBoxApp { toolbox }
    }
}

impl eframe::App for ArtBoxApp {
     
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
         
        if let Some(active_tool) = self.toolbox.get_active_tool() {
            active_tool.show_ui(ctx);
        }
        
    }
}
