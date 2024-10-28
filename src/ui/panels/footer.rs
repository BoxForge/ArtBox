use eframe::egui;

pub trait FooterTrait {

    fn show_footer(&self, ui: &mut egui::Ui) {
        ui.label("Footer");
    }
}
