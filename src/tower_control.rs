use std::fs;
use std::path::Path;
use chrono::Local;
use eframe::{App, CreationContext, egui, Frame};
use eframe::egui::{Align2, Context, popup_below_widget, ProgressBar, Ui};
use egui_extras::install_image_loaders;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use crate::crashtest_data::{CrashtestData, CrashtestResults};
use crate::custom_widgets::{add_lamp};
use crate::tower_simulation::TowerSimulation;


const APP_DATA_PATH: &str = r"src/app_data.json";
const SPECIAL_CHARS: [char; 12] = ['<', '>', '.', ':', ',', ';', '“', '/', '\\', '|', '?', '*'];

#[derive(Debug, Default, Deserialize, Serialize)]
struct AppData {
	save_path: Option<String>,
}


#[derive(Default)]
enum ScreenState {
	#[default]
	First,
	CrashDataEntry,
	Confirmation,
	ReadyToCrash,
	DataAcquisition,
	Results(CrashtestResults),
}

#[derive(Default)]
pub struct TowerControl {
	camera_in_place: bool,
	object_in_place: bool,
	error_window: Option<String>,
	crashtest_path: Option<String>,
	crashtest_data: CrashtestData,
	screen_state: ScreenState,
	sim: TowerSimulation,
	app_data: AppData,
}


impl TowerControl {
	pub fn new(_cc: &CreationContext) -> Self {
		let app_data = serde_json::from_str(&fs::read_to_string(APP_DATA_PATH).unwrap()).unwrap();
		Self { app_data, ..Default::default() }
	}
	
	fn save_app(&self) {
		fs::write(APP_DATA_PATH, serde_json::to_string(&self.app_data).unwrap())
			.expect("Could not save app");
	}
	fn save_crashtest(&self) {
		if let Some(crashtest_path) = &self.crashtest_path {
			fs::write(crashtest_path, serde_json::to_string(&self.crashtest_data).unwrap())
				.expect("Could not save crashtest");
		}
	}
	
	fn menus(&mut self, _ctx: &Context, ui: &mut Ui) {
		ui.menu_button("File", |ui| {
			if ui.button("Nouveau crashtest").clicked() {
				self.save_crashtest();
				self.crashtest_data.reset();
				self.crashtest_path = None;
				self.screen_state = ScreenState::CrashDataEntry;
			}
			if ui.button("Ouvrir un crashtest…").clicked() {
				let mut file_dialog = FileDialog::new().add_filter("JSON", &["json"]);
				if let Some(folder_path) = &self.app_data.save_path {
					file_dialog = file_dialog.set_directory(Path::new(folder_path));
				}
				if let Some(path) = file_dialog.pick_file() {
					self.app_data.save_path = Some(path.to_str().unwrap().to_string());
				}
				ui.close_menu();
			}
			
			if ui.button("Quitter").clicked() {
				todo!("Quitter")
				//frame.close();
				//ui.close_menu();
			}
		});
	}
	
	fn left_panel(&mut self, _ctx: &Context, ui: &mut Ui) {
		ui.vertical_centered(|ui| {
			ui.heading("Contrôle");
			ui.add_space(5.);
		});
		
		ui.group(|ui| {
			ui.label("AppData:");
			let save_path = self.app_data.save_path.clone().unwrap_or(" aucun fichier sélectionné".to_string());
			ui.label(format!("Save path: {}", save_path));
		});
		
		add_lamp(ui, &self.sim.is_impactor_charged(), "Impacteur chargé");
		add_lamp(ui, &self.sim.is_clamps_closed(), "Clamps fermés");
		add_lamp(ui, &self.sim.is_magnet_closed(), "Aimant fermé");
		add_lamp(ui, &self.sim.is_door_closed(), "Porte fermée");
		
		ui.group(|ui| {
			ui.horizontal(|ui| {
				ui.vertical(|ui| {
					ui.add(ProgressBar::new(self.sim.get_height().clone() / TowerSimulation::MAX_HEIGHT).desired_width(100.));
					ui.label(format!("{:.2}", self.sim.get_height()));
				});
				
				ui.vertical(|ui| {
					if ui.button("⏫").is_pointer_button_down_on() {
						self.sim.change_height(0.05);
					}
					if ui.button("⏶").clicked() {
						self.sim.change_height(0.01);
					}
					if ui.button("⏷").clicked() {
						self.sim.change_height(-0.01);
					}
					if ui.button("⏬").is_pointer_button_down_on() {
						self.sim.change_height(-0.05);
					}
				});
			})
		});
	}
	
	fn right_panel(&mut self, _ctx: &Context, ui: &mut Ui) {
		ui.vertical_centered(|ui| {
			ui.heading("Info");
			ui.add_space(5.);
		});
		self.crashtest_data.show_data(ui);
		
		let r = ui.button("Open popup");
		let popup_id = ui.make_persistent_id("popup id");
		if r.clicked() {
			ui.memory_mut(|mem| mem.toggle_popup(popup_id));
		}
		popup_below_widget(ui, popup_id, &r, |ui| {
			ui.set_min_width(200.0); // if you want to control the size
			ui.label("Some more info, or things you can select:");
			ui.label("…");
		});
	}
	
	fn central_panel(&mut self, _ctx: &Context, ui: &mut Ui) {
		match &self.screen_state {
			ScreenState::First => {
				ui.vertical_centered(|ui| {
					if ui.button("Nouveau").clicked() {
						self.screen_state = ScreenState::CrashDataEntry;
					}
				});
			}
			ScreenState::CrashDataEntry => {
				self.crashtest_data.change_data(ui);
				
				if ui.button("Ok").clicked() {
					if self.crashtest_data.get_name().contains(SPECIAL_CHARS) {
						self.error_window = Some(
							"Nom du test incorrect\nUtilisation de charactères spéciaux\n<>.:,;/\\|?*".to_owned()
						);
					} else if self.crashtest_data.get_name().is_empty() {
						self.error_window = Some("Nom du test vide".to_owned());
					} else {
						// Définit le path du crashtest
						if let Some(folder_path) = self.app_data.save_path.to_owned() {
							self.crashtest_path = Some(folder_path + "\\" + &self.crashtest_data.get_name() + ".json");
							self.screen_state = ScreenState::Confirmation;
						} else if let Some(folder_path) = FileDialog::new().pick_folder() {
							let folder_path = folder_path.to_str().unwrap().to_string();
							self.app_data.save_path = Some(folder_path.clone());
							self.save_app();
							self.crashtest_path = Some(folder_path + "\\" + &self.crashtest_data.get_name() + ".json");
							self.screen_state = ScreenState::Confirmation;
						}
					}
				}
			}
			ScreenState::Confirmation => {
				ui.checkbox(&mut self.camera_in_place, "Caméra installée");
				ui.checkbox(&mut self.object_in_place, "Objet en place");
				if ui.button("Make everything ok").clicked() {
					self.sim.make_everything_ok();
				}
				
				ui.centered_and_justified(|ui| {
					if ui.button("Passer au largage").clicked() {
						let conditions = vec![
							&self.camera_in_place,
							&self.object_in_place,
							self.sim.is_magnet_closed(),
							self.sim.is_clamps_closed(),
							self.sim.is_impactor_charged(),
							self.sim.is_door_closed(),
						];
						if conditions.iter().all(|&&c| c) {
							self.screen_state = ScreenState::ReadyToCrash;
						} else {
							let index = conditions.iter().position(|&&c| !c).unwrap();
							self.error_window = Some(match index {
								0 => "Installer la caméra",
								1 => "Installer l'objet de test",
								2 => "Défaut aimant !",
								3 => "Défaut clamps !",
								4 => "Charger l'impacteur",
								5 => "Fermer la porte",
								_ => "not implemented (code error)",
							}.to_owned());
						}
					}
				});
			}
			ScreenState::ReadyToCrash => {
				ui.vertical_centered(|ui| {
					if ui.button("Confirmer largage").clicked() {
						// Launch Crash-test
						self.sim.launch_crash();
						self.crashtest_data.set_date_time(Local::now());
						self.screen_state = ScreenState::DataAcquisition;
					}
				});
			}
			ScreenState::DataAcquisition => {
				ui.vertical_centered(|ui| {
					ui.spinner();
					
					let crash_results = self.sim.get_crash_results();  // delay
					self.crashtest_data.push_results(crash_results.clone());
					self.save_crashtest();
					self.screen_state = ScreenState::Results(crash_results);
				});
			}
			ScreenState::Results(crashtest_results) => {
				// ui.add(Image::new(egui::include_image!("../assets/gear_icon.png")).rounding(5.0));
				ui.horizontal(|ui| {
					ui.label("Speed:");
					ui.strong(format!("{:?}", crashtest_results.get_speed()));
				});
			}
		}
	}
}


impl App for TowerControl {
	fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
		install_image_loaders(ctx);
		
		egui::CentralPanel::default().show(ctx, |ui| {
			if let Some(error_text) = &self.error_window {
				let mut open = true;
				egui::Window::new("ERROR")
					.resizable(false)
					.collapsible(false)
					.anchor(Align2::CENTER_CENTER, [0., 0.])
					.open(&mut open)
					.show(ctx, |ui| {
						ui.label(error_text);
					});
				if !open { self.error_window = None; }
			}
			ui.set_enabled(self.error_window.is_none());
			
			egui::TopBottomPanel::top("top_panel")
				.show_inside(ui, |ui| self.menus(ctx, ui));
			
			egui::SidePanel::left("left_panel")
				.resizable(false)
				.default_width(200.0)
				.show_inside(ui, |ui| self.left_panel(ctx, ui));
			
			egui::SidePanel::right("right_panel")
				.resizable(false)
				.default_width(200.0)
				//.width_range(150.0..=300.0)
				.show_inside(ui, |ui| self.right_panel(ctx, ui));
			
			egui::CentralPanel::default().show_inside(ui, |ui| {
				self.central_panel(ctx, ui);
			});
		});
		//ctx.request_repaint();
	}
	
	fn on_exit(&mut self, _ctx: Option<&eframe::glow::Context>) { self.save_app(); }
}
