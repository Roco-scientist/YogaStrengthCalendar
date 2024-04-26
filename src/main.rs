#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// #[allow(non_snake_case)]
use yoga_strength_calendar::app::StrengthYogaApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([370.0, 800.0])
            .with_min_inner_size([370.0, 220.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../bike.png")[..]).unwrap(),
            ),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Yoga and strength for cycling",
        native_options,
        Box::new(|_cc| Box::<StrengthYogaApp>::default()),
    );
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|_cc| Box::<StrengthYogaApp>::default()),
            )
            .await
            .expect("failed to start eframe");
    });
}
