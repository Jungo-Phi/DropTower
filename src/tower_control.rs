use std::fs::read_to_string;
use std::path::Path;
use chrono::Local;
use eframe::{App, CreationContext, egui, Frame};
use eframe::egui::{Align, Button, Context, Direction, Image, Layout, ProgressBar, Ui, Vec2};
use egui_extras::install_image_loaders;
use rfd::FileDialog;
use crate::crash_data::CrashData;
use crate::custom_widgets::{add_lamp};
use crate::tower_simulation::{CrashResults, TowerSimulation};


#[derive(Default)]
enum ScreenState {
	#[default]
	First,
	CrashDataEntry,
	Confirmation,
	ReadyToCrash,
	DataAcquisition,
	Results(CrashResults),
}

#[derive(Default)]
pub struct TowerControl {
	camera_in_place: bool,
	object_in_place: bool,
	error_window: Option<String>,
	crash_data: CrashData,
	screen_state: ScreenState,
	sim: TowerSimulation,
	save_path: Option<String>,
}


impl TowerControl {
	pub fn new(_cc: &CreationContext) -> Self { Self::default() }
	
	fn load(&self) {
		println!("LOAD");
		let contents = read_to_string(self.save_path.clone().unwrap()).expect("Couldn't find or load that file.");
		println!("{}", contents);
	}
	
	fn save(&self) {
		if let ScreenState::Results(crash_results) = &self.screen_state {
			println!("SAVE");
			// TODO Save
		}
	}
	
	fn get_time(&self, ui: &mut Ui) {
		ui.separator();
		ui.label("Time");
		if ui.button("get time").clicked() {
			let now = Local::now();
			println!("{}", now);
		}
	}
	
	fn menus(&mut self, _ctx: &Context, ui: &mut Ui) {
		ui.menu_button("File", |ui| {
			if ui.button("Nouveau").clicked() {
				todo!("Nouveau");
			}
			if ui.button("Ouvrir un fichier…").clicked() {
				if let Some(path) = FileDialog::new()
					.add_filter("JSON", &["json"])
					.set_directory(Path::new(r"C:\Users"))
					.pick_file() {
					self.save_path = Some(path.display().to_string());
					self.load();
				}
				ui.close_menu();
			}
			if ui.button("Enregistrer").clicked() {
				todo!("Enregistrer");
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
		ui.horizontal(|ui| {
			ui.label("Nom du test:");
			ui.strong(&self.crash_data.name);
		});
		ui.horizontal(|ui| {
			ui.label("Auteur:");
			ui.strong(&self.crash_data.author);
		});
		ui.horizontal(|ui| {
			ui.label("Commentaire:");
			ui.strong(&self.crash_data.comment);
		});
		
		self.get_time(ui);
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
				ui.horizontal(|ui| {
					ui.label("Nom du test:");
					ui.text_edit_singleline(&mut self.crash_data.name);
				});
				ui.horizontal(|ui| {
					ui.label("Auteur:");
					ui.text_edit_singleline(&mut self.crash_data.author);
				});
				ui.horizontal(|ui| {
					ui.label("Commentaire:");
					ui.text_edit_multiline(&mut self.crash_data.comment);
				});
				if ui.button("Ok").clicked() {
					self.screen_state = ScreenState::Confirmation;
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
								2 => "Aimant !",
								3 => "Clamps !",
								4 => "Impacteur non chargé",
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
						self.screen_state = ScreenState::DataAcquisition;
						self.sim.launch_crash();
					}
				});
			}
			ScreenState::DataAcquisition => {
				ui.vertical_centered(|ui| {
					ui.spinner();
					
					let crash_results = self.sim.get_crash_results();  // delay
					self.save();
					self.screen_state = ScreenState::Results(crash_results);
				});
			}
			ScreenState::Results(crash_results) => {
				// ui.add(Image::new(egui::include_image!("../assets/gear_icon.png")).rounding(5.0));
				ui.horizontal(|ui| {
					ui.label("Speed:");
					ui.strong(format!("{:?}", crash_results.get_speed()));
				});
			}
		}
	}
}


impl App for TowerControl {
	fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
		install_image_loaders(ctx);
		
		if let Some(error_text) = &self.error_window {
			let mut open = true;
			egui::Window::new("ERROR")
				.open(&mut open)
				.show(ctx, |ui| {
					ui.label(error_text);
				});
			if !open { self.error_window = None; }
		}
		
		egui::CentralPanel::default().show(ctx, |ui| {
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
}
