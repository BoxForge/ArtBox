pub mod sprite_library;

use std::collections::HashMap;
use crate::tools::sprite_library::SpriteLibraryUI;


// TOOLBOX

/**************************************************
 * ToolBox Struct (Tool Storage)
 * Manages all tool instances, stores and activates tools.
 **************************************************/

// Stores all tools in a HashMap and keeps track of the active tool.

pub struct ToolBox {
    tools: HashMap<String, Box<Tool>>,
    active_tool: Option<String>
}

impl ToolBox {
    // Creates a new, empty ToolBox. 
    pub fn new() -> Self {
        ToolBox {
            tools: HashMap::new(),
            active_tool: None
        }
    }

    // Adds a tool to the ToolBox with a specific name. 
    pub fn add_tool(&mut self, name: &str, tool: Box<Tool>) {
        self.tools.insert(name.to_string(), tool);
    }

    // Sets a tool as active based on its name. 
    pub fn set_active_tool(&mut self, name: &str) {
        self.active_tool = Some(name.to_string());
    }

    // Retrieves a mutable reference to the currently active tool. 
    pub fn get_active_tool(&mut self) -> Option<&mut Box<Tool>> {
        if let Some(active_tool) = &self.active_tool {
            self.tools.get_mut(active_tool)
        } else {
            None
        }
    }

    // Lists all the tools by their names. 
    pub fn list_tools(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }
}


// TOOL MANAGER

/**************************************************
 * ToolManager Struct (Tool Creation and Management)
 * Handles creation of tools and manages tool initialization.
 **************************************************/

pub struct ToolManager;

impl ToolManager {
    // Creates a new instance of ToolManager (singleton-like behavior). 
    pub fn new() -> Self {
        ToolManager {}
    }

    // Tool Creation Functions
    // Creates a new instance of the SpriteLibrary tool.
    fn create_sprite_library() -> Box<Tool> {
        ToolBuilder::new().with_ui(Box::new(SpriteLibraryUI::new())).build()
    }


    // ToolBox Initialization
    // Initializes the ToolBox with a set of predefined tools. 
    pub fn initialize_tools(&self) -> ToolBox {
        let mut toolbox = ToolBox::new();

        // Add Tools to ToolBox
        toolbox.add_tool("Sprite Library", Self::create_sprite_library());

        // Set Active Tool
        toolbox.set_active_tool("Sprite Library");

        toolbox
    }
}


// TOOL STRUCT

 /**************************************************
 * Tool Struct and Implementation
 * Represents a single tool in the application, composed of multiple components.
 **************************************************/

pub struct Tool {
    ui: Box< dyn ToolUI >,
    logic: Box< dyn ToolLogic >,
    settings: Box< dyn ToolSettings >,
    data: Box< dyn ToolData >,
    api: Box< dyn ToolApi >,
    state: Box< dyn ToolState > 
}

impl Tool {
    // Creates a new instance of a tool with its components. 
    pub fn new (
        ui: Box< dyn ToolUI >,
        logic: Box< dyn ToolLogic >,
        settings: Box< dyn ToolSettings >,
        data: Box< dyn ToolData >,
        api: Box< dyn ToolApi >,
        state: Box< dyn ToolState >,
    ) -> Self {
        Tool { ui, logic, settings, data, api, state }
    }

    // Displays the tool's UI (calls the appropriate `show_*` functions). 
    pub fn show_ui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.ui.show_title_bar(ui);
            ui.separator();
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui.show_main_panel(ui);
            ui.separator();
        });

        egui::SidePanel::right("right_panel").resizable(false).show(ctx, |ui| {
            self.ui.show_info_panel(ui);
            ui.separator();
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            self.ui.show_footer(ui);
        }); 
    } 
}


// TOOL BUILDER

/**************************************************
 * ToolBuilder Struct (Tool Construction)
 * Helps build tools step-by-step, allowing customization.
 **************************************************/ 

pub struct ToolBuilder {
    ui: Option <Box <dyn ToolUI>>,
    logic: Option <Box <dyn ToolLogic>>,
    settings: Option <Box <dyn ToolSettings>>,
    data: Option <Box <dyn ToolData>>,
    api: Option <Box <dyn ToolApi>>,
    state: Option <Box <dyn ToolState>>,
}


// DEFAULT TOOL BUILDER
// Default implementation for ToolBuilder (initializes without components). 

impl ToolBuilder {
    pub fn new() -> Self {
        ToolBuilder {
            ui: None,
            logic: None,
            settings: None,
            data: None,
            api: None,
            state: None
        }
    }

    pub fn with_ui(mut self, ui: Box<dyn ToolUI>) -> Self { 
        self.ui = Some(ui);
        self
    }
     
    pub fn with_logic(mut self, logic: Box<dyn ToolLogic>) -> Self {
        self.logic = Some(logic);
        self
    }

    pub fn with_settings(mut self, settings: Box<dyn ToolSettings>) -> Self {
        self.settings = Some(settings);
        self
    }

    pub fn with_data(mut self, data: Box<dyn ToolData>) -> Self {
        self.data = Some(data);
        self
    }

    pub fn with_api(mut self, api: Box<dyn ToolApi>) -> Self {
        self.api = Some(api);
        self
    }

    pub fn with_state(mut self, state: Box<dyn ToolState>) -> Self {
        self.state = Some(state);
        self
    }

    pub fn build(self) -> Box<Tool> {
        Box::new(Tool::new(
            self.ui.unwrap_or_else(|| Box::new(DefaultToolUI {})),
            self.logic.unwrap_or_else(|| Box::new(DefaultToolLogic {})),
            self.settings.unwrap_or_else(|| Box::new(DefaultToolSettings {})),
            self.data.unwrap_or_else(|| Box::new(DefaultToolData {})),
            self.api.unwrap_or_else(|| Box::new(DefaultToolApi {})),
            self.state.unwrap_or_else(|| Box::new(DefaultToolState {})),
        ))
    }
}


// DEFAULT COMPONENTS

/**************************************************
 * Default Components and Macros (Tool Defaults)
 * Provides default implementations for Tool components.
 **************************************************/

// A macro to define default structs for each component. 

macro_rules! default_component {
    ($name:ident, $trait:ident) => {
        pub struct $name;
        impl $trait for $name {}
        impl Default for $name {
            fn default() -> Self {
                $name {}
            }
        }
    };

    ($name:ident, $trait:ident, $default_impl:block) => {
        pub struct $name;
        impl $trait for $name {
            $default_impl
        }
        impl Default for $name {
            fn default() -> Self {
                $name {}
            }
        }
    };
}

// Defining default components for each tool trait.
default_component!(DefaultToolUI, ToolUI);
default_component!(DefaultToolLogic, ToolLogic);
default_component!(DefaultToolSettings, ToolSettings);
default_component!(DefaultToolData, ToolData);
default_component!(DefaultToolApi, ToolApi);
default_component!(DefaultToolState, ToolState);


// TOOL TRAITS
/**************************************************
 * Tool Traits (Interfaces for Tool Components)
 * Defines the common interface for UI, Logic, Settings, etc.
 **************************************************/ 

pub trait ToolUI {
    
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


pub trait ToolLogic {
    // TODO: Add default tool logic
}

pub trait ToolSettings {
    // TODO: Add default tool settings
}

pub trait ToolData {
    // TODO: Add default tool data
}

pub trait ToolApi {
    // TODO: Add default tool api
}

pub trait ToolState {
    // TODO: Add default tool state
}
 
/**************************************************
 * TODOs for Future Implementation:
 * - Rework `ToolBuilder::build()` to support non-default (custom) types.
 * - Figure out how to acheive more private components.
 *************************************************/
