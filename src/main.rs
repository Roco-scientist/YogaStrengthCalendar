#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use anyhow::Result;
use cycling_strength_yoga::gui;
use eframe::egui;

fn main() -> Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 600.0])
            .with_min_inner_size([300.0, 220.0]),
        // .with_icon(
        //     // NOTE: Adding an icon is optional
        //     eframe::icon_data::from_png_bytes(&include_bytes!("")[..])
        //         .unwrap(),
        // ),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Yoga and strength for cycling",
        native_options,
        Box::new(|_cc| Box::new(gui::StrengthYogaApp::default())),
    );
    Ok(())
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
                Box::new(|cc| Box::new(gui::StrengthYogaApp::default())),
            )
            .await
            .expect("failed to start eframe");
    });
}
