pub mod sprite_library;

use std::collections::HashMap;
// use crate::tools::sprite_library::SpriteLibrary;


// TOOLBOX

/**************************************************
 * ToolBox Struct (Tool Storage)
 * Manages all tool instances, stores and activates tools.
 **************************************************/

// Stores all tools in a HashMap and keeps track of the active tool.

pub struct ToolBox {
    tools: HashMap<String, Tool<DefaultToolUI, DefaultToolLogic, DefaultToolSettings, DefaultToolData, DefaultToolApi, DefaultToolState>>,
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
    pub fn add_tool(&mut self, name: &str, tool: Tool<DefaultToolUI, DefaultToolLogic, DefaultToolSettings, DefaultToolData, DefaultToolApi, DefaultToolState>) {
        self.tools.insert(name.to_string(), tool);
    }

    // Sets a tool as active based on its name. 
    pub fn set_active_tool(&mut self, name: &str) {
        self.active_tool = Some(name.to_string());
    }

    // Retrieves a mutable reference to the currently active tool. 
    pub fn get_active_tool(&mut self) -> Option<&mut Tool<DefaultToolUI, DefaultToolLogic, DefaultToolSettings, DefaultToolData, DefaultToolApi, DefaultToolState>> {
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
    
    // Tool Creation Functions
    // Creates a new instance of the SpriteLibrary tool.
    // TODO: Make this more flexible by allowing different configurations instead of using default. 
    fn create_sprite_library() -> Tool<DefaultToolUI, DefaultToolLogic, DefaultToolSettings, DefaultToolData, DefaultToolApi, DefaultToolState> {
        ToolBuilder::new().with_ui(DefaultToolUI {}).build()
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

pub struct Tool<TUI, TLogic, TSettings, TData, TApi, TState> {
    ui: TUI,
    logic: TLogic,
    settings: TSettings,
    data: TData,
    api: TApi,
    state: TState
}

impl<TUI, TLogic, TSettings, TData, TApi, TState> Tool<TUI, TLogic, TSettings, TData, TApi, TState> 
where
    TUI: ToolUI,
    TLogic: ToolLogic,
    TSettings: ToolSettings,
    TData: ToolData,
    TApi: ToolApi,
    TState: ToolState
{
    // Creates a new instance of a tool with its components. 
    pub fn new (
        ui: TUI,
        logic: TLogic,
        settings: TSettings,
        data: TData,
        api: TApi,
        state: TState,
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

pub struct ToolBuilder<TUI, TLogic, TSettings, TData, TApi, TState> {
    ui: Option<TUI>,
    logic: Option<TLogic>,
    settings: Option<TSettings>,
    data: Option<TData>,
    api: Option<TApi>,
    state: Option<TState>
}


// DEFAULT TOOL BUILDER
// Default implementation for ToolBuilder (initializes without components). 

impl ToolBuilder<DefaultToolUI, DefaultToolLogic, DefaultToolSettings, DefaultToolData, DefaultToolApi, DefaultToolState> {
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
}

// CUSTOM TOOL BUILDER 
// Custom implementation to add specific components to ToolBuilder.

impl<TUI, TLogic, TSettings, TData, TApi, TState> ToolBuilder<TUI, TLogic, TSettings, TData, TApi, TState> { 
    // Functions like `with_ui()`, `with_logic()`, etc., are used to add components step-by-step.
    pub fn with_ui<U>(self, ui: U) -> ToolBuilder<U, TLogic, TSettings, TData, TApi, TState> 
    where U: ToolUI, 
    { 
        ToolBuilder { 
            ui: Some(ui),
            logic: self.logic,
            settings: self.settings,
            data: self.data,
            api: self.api,
            state: self.state
        }
    }
     
    pub fn with_logic<L>(self, logic: L) -> ToolBuilder<TUI, L, TSettings, TData, TApi, TState> 
    where L: ToolLogic 
    {
        ToolBuilder {
            ui: self.ui,
            logic: Some(logic),
            settings: self.settings,
            data: self.data,
            api: self.api,
            state: self.state
        }
    }

    pub fn with_settings<C>(self, settings: C) -> ToolBuilder<TUI, TLogic, C, TData, TApi, TState> 
    where C: ToolSettings 
    {
        ToolBuilder {
            ui: self.ui,
            logic: self.logic,
            settings: Some(settings),
            data: self.data,
            api: self.api,
            state: self.state
        }
    }

    pub fn with_data<D>(self, data: D) -> ToolBuilder<TUI, TLogic, TSettings, D, TApi, TState> 
    where D: ToolData 
    {
        ToolBuilder {
            ui: self.ui,
            logic: self.logic,
            settings: self.settings,
            data: Some(data),
            api: self.api,
            state: self.state
        }
    }

    pub fn with_api<A>(self, api: A) -> ToolBuilder<TUI, TLogic, TSettings, TData, A, TState> 
    where A: ToolApi 
    {
        ToolBuilder {
            ui: self.ui,
            logic: self.logic,
            settings: self.settings,
            data: self.data,
            api: Some(api),
            state: self.state
        }
    }

    pub fn with_state<S>(self, state: S) -> ToolBuilder<TUI, TLogic, TSettings, TData, TApi, S> 
    where S: ToolState 
    {
        ToolBuilder {
            ui: self.ui,
            logic: self.logic,
            settings: self.settings,
            data: self.data,
            api: self.api,
            state: Some(state)
        }
    }
}


// TOOL BUILD
// Builds the tool with all provided components or their default values. 

impl<TUI, TLogic, TSettings, TData, TApi, TState> ToolBuilder<TUI, TLogic, TSettings, TData, TApi, TState> 
where
    TUI: ToolUI + Default,
    TLogic: ToolLogic + Default,
    TSettings: ToolSettings + Default,
    TData: ToolData + Default,
    TApi: ToolApi + Default,
    TState: ToolState + Default
{
    pub fn build(self) -> Tool<TUI, TLogic, TSettings, TData, TApi, TState> {
        Tool::new(
            self.ui.unwrap_or_default(),
            self.logic.unwrap_or_default(),
            self.settings.unwrap_or_default(),
            self.data.unwrap_or_default(),
            self.api.unwrap_or_default(),
            self.state.unwrap_or_default(),
        )
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
 *************************************************/
