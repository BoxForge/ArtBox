fn default_setup(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "luckiest_guy".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../../../assets/LuckiestGuy-Regular.ttf"
        )),
    );

    fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "luckiest_guy".to_owned());
    fonts.families.entry(egui::FontFamily::Monospace).or_default().insert(0, "luckiest_guy".to_owned());

    ctx.set_fonts(fonts);
}
