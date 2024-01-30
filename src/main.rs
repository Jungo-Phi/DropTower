mod tower_control;
mod utils;
mod crashtest_data;
mod custom_widgets;
mod tower_simulation;

use std::sync::Arc;
use eframe::egui::{IconData, Vec2};
use tower_control::TowerControl;
use image::load_from_memory;



fn main() -> Result<(), eframe::Error> {
	// if cfg!(target_os = "macos") { panic!("Does not work on MacOS") }
	
	let icon = {
		let image = load_from_memory(include_bytes!("../assets/drop_tower_icon.png"))
		.expect("Failed to open icon path")
		.into_rgba8();
		let (width, height) = image.dimensions();
		let rgba = image.into_raw();
		IconData { rgba, width, height }
	};
	
	let viewport = eframe::egui::ViewportBuilder {
		inner_size: Some(Vec2::new(1000., 600.)),
		min_inner_size: Some(Vec2::new(600., 300.)),
		icon: Some(Arc::new(icon)),
		fullsize_content_view: None,
		titlebar_buttons_shown: Some(true),
		titlebar_shown: Some(false),
		..Default::default()
	};
	
	let options = eframe::NativeOptions {
		centered: true,
		viewport,
		..Default::default()
	};
	eframe::run_native(
		"DropTower Control",
		options,
		Box::new(|cc| Box::new(TowerControl::new(cc)))
	)
}
