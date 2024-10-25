mod app;
mod ui;

fn main() -> Result<(), eframe::Error>{
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "ArtBox",
        options,
        Box::new(|_cc| Ok(Box::new(app::ArtBoxApp::new()))),
    )?;
    Ok(())
}

