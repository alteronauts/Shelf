mod app;
mod export;
mod model;
mod persist;
mod scanner;
mod ui;

fn load_icon() -> egui::IconData {
    let png_bytes = include_bytes!("../assets/icon.png");
    let img = image::load_from_memory(png_bytes)
        .expect("Failed to decode icon")
        .into_rgba8();
    let (w, h) = img.dimensions();
    egui::IconData {
        rgba: img.into_raw(),
        width: w,
        height: h,
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 650.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title("Shelf")
            .with_icon(std::sync::Arc::new(load_icon())),
        ..Default::default()
    };

    eframe::run_native(
        "Shelf",
        options,
        Box::new(|_cc| Ok(Box::new(app::App::default()))),
    )
    .expect("Failed to run Shelf");
}
