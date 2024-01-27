mod tower_control;
mod toggle_switch;
mod utils;
mod crash_data;
mod custom_widgets;

use eframe::egui::Vec2;
use eframe::{Theme}; //IconData
use tower_control::TowerControl;


fn main() -> Result<(), eframe::Error> {
	//let icon = IconData::try_from_png_bytes(include_bytes!("../assets/drop_tower_icon.png")).unwrap();
	
	let options = eframe::NativeOptions {
		//icon_data: Some(icon),
		//initial_window_size: Some(Vec2::new(1000., 600.)),
		//min_window_size: Some(Vec2::new(750., 400.)),
		centered: true,
		//resizable: true,
		default_theme: Theme::Light,
		..Default::default()
	};
	eframe::run_native(
		"DropTower Control",
		options,
		Box::new(|cc| Box::new(TowerControl::new(cc)))
	)
}
