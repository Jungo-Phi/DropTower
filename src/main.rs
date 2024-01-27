mod tower_control;
mod toggle_switch;
mod utils;
mod crash_data;
mod custom_widgets;

use std::default;
use std::sync::Arc;
use eframe::egui::{IconData, Vec2};
use eframe::Theme;
use tower_control::TowerControl;
use image::load_from_memory;



fn main() -> Result<(), eframe::Error> {
	/*let icon = Some(Arc::new(IconData {
		rgba: Vec::from(include_bytes!("../assets/drop_tower_icon.png")),
		width: 256,
		height: 256,
	}));*/
	let icon = {
		let image = load_from_memory(include_bytes!("../assets/drop_tower_icon.png"))
		.expect("Failed to open icon path")
		.into_rgba8();
		let (width, height) = image.dimensions();
		let rgba = image.into_raw();
		IconData { rgba, width, height }
	}; // an example
	
	let vb = eframe::egui::ViewportBuilder {
		inner_size: Some(Vec2::new(1000., 600.)),
		min_inner_size: Some(Vec2::new(750., 400.)),
		icon: Some(Arc::new(icon)),
		fullsize_content_view: None,
		titlebar_buttons_shown: Some(true),
		titlebar_shown: Some(false),
		..Default::default()
	};
	
	let options = eframe::NativeOptions {
		centered: true,
		viewport: vb,
		default_theme: Theme::Light,
		..Default::default()
	};
	eframe::run_native(
		"DropTower Control",
		options,
		Box::new(|cc| Box::new(TowerControl::new(cc)))
	)
}
