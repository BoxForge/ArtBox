pub mod sprite_library;

// TODO: Add ToolManager up here

#[allow(dead_code)]
struct Tool<TUI, TLogic, TSettings, TData, TApi, TState> {
    ui: TUI,
    logic: TLogic,
    settings: TSettings,
    data: TData,
    api: TApi,
    state: TState
}

#[allow(dead_code)]
impl<TUI, TLogic, TSettings, TData, TApi, TState> Tool<TUI, TLogic, TSettings, TData, TApi, TState> 
where
    TUI: ToolUI,
    TLogic: ToolLogic,
    TSettings: ToolSettings,
    TData: ToolData,
    TApi: ToolApi,
    TState: ToolState
{
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
}

// TOOL BUILDER

pub struct ToolBuilder<TUI, TLogic, TSettings, TData, TApi, TState> {
    ui: Option<TUI>,
    logic: Option<TLogic>,
    settings: Option<TSettings>,
    data: Option<TData>,
    api: Option<TApi>,
    state: Option<TState>
}

// DEFAULT TOOL BUILDER
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
impl<TUI, TLogic, TSettings, TData, TApi, TState> ToolBuilder<TUI, TLogic, TSettings, TData, TApi, TState> { 
    // Function that builds a custom implementation of ToolUI 
    pub fn with_ui<U>(self, ui: TUI) -> ToolBuilder<U, TLogic, TSettings, TData, TApi, TState> 
    where U: ToolUI 
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
     
    // Function that builds a custom implementation of ToolLogic 
    pub fn with_logic<L>(self, logic: TLogic) -> ToolBuilder<TUI, L, TSettings, TData, TApi, TState> 
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

    // Function that builds a custom implementation of ToolSettings 
    pub fn with_settings<S>(self, settings: TSettings) -> ToolBuilder<TUI, TLogic, S, TData, TApi, TState> 
    where S: ToolSettings 
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

    // Function that builds a custom implementation of ToolData 
    pub fn with_data<D>(self, data: TData) -> ToolBuilder<TUI, TLogic, TSettings, D, TApi, TState> 
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

    // Function that builds a custom implementation of ToolApi
    pub fn with_api<A>(self, api: TApi) -> ToolBuilder<TUI, TLogic, TSettings, TData, A, TState> 
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

    // Function that builds a custom implementation of ToolState
    pub fn with_state<S>(self, state: TState) -> ToolBuilder<TUI, TLogic, TSettings, TData, TApi, S> 
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

default_component!(DefaultToolUI, ToolUI);
default_component!(DefaultToolLogic, ToolLogic);
default_component!(DefaultToolSettings, ToolSettings);
default_component!(DefaultToolData, ToolData);
default_component!(DefaultToolApi, ToolApi);
default_component!(DefaultToolState, ToolState);

// TOOL TRAITS

pub trait ToolUI {
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


trait ToolLogic {
    // TODO: Add default tool logic
}

trait ToolSettings {
    // TODO: Add default tool settings
}

trait ToolData {
    // TODO: Add default tool data
}

trait ToolApi {
    // TODO: Add default tool api
}

trait ToolState {
    // TODO: Add default tool state
}
