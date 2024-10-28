use crate::ui::panels::{title_bar::TitleBarTrait, main_view::MainViewTrait, info_panel::InfoPanelTrait, footer::FooterTrait};

pub trait ToolView: TitleBarTrait + MainViewTrait + InfoPanelTrait + FooterTrait {}
