use eframe::{App, CreationContext, egui, Frame};
use eframe::egui::{Context, Image, Slider, Ui};
use egui_extras::install_image_loaders;
use crate::crash_data::{CrashData, CrashResults};
use crate::custom_widgets::{ui_lamp};


enum ScreenState {
	First,
	CrashData(CrashData),
	AreYouReadyToCrash(CrashData),
	ReadyToCrash(CrashData),
	Results(CrashData, CrashResults)
}


pub struct TowerControl {
	a: bool,
	b: u16,
	c: f32,
	camera_in_place: bool,
	object_in_place: bool,
}

impl TowerControl {
	pub fn new(_cc: &CreationContext) -> Self {
		Self { a: false, b: 1, c: 0., camera_in_place: false, object_in_place: false }
	}
}


impl TowerControl {
	fn menus(&mut self, _ctx: &Context, frame: &mut Frame, ui: &mut Ui) {
		ui.menu_button("File", |ui| {
			ui.menu_button("Tests", |ui| {
				ui.selectable_value(&mut self.b, 1, "1");
				ui.selectable_value(&mut self.b, 2, "2");
				ui.selectable_value(&mut self.b, 3, "3");
			});
			if ui.button("Exit").clicked() {
				todo!("close") //frame.close();
				//ui.close_menu();
			}
		});
	}
	
	fn left_panel(&mut self, _ctx: &Context, _frame: &mut Frame, ui: &mut Ui) {
		ui.vertical_centered(|ui| {
			ui.heading("Parameters");
			ui.add_space(5.);
		});
		
		ui.label(format!("Selected: {}", self.b));
		
		ui.checkbox(&mut self.a, "A");
	}
	
	fn right_panel(&mut self, _ctx: &Context, _frame: &mut Frame, ui: &mut Ui) {
		ui.vertical_centered(|ui| {
			ui.heading("Info");
			ui.add_space(5.);
		});
		
		let _ = ui.radio(true, "Impacteur chargé");
		let _ = ui.radio(false, "Clamps fermés");
		let _ = ui.radio(true, "Porte fermée");
		ui_lamp(ui, &self.a, "Percuteur chargé");
		ui_lamp(ui, &self.a, "Clamps fermés");
		ui_lamp(ui, &self.a, "Porte fermée");
		
		ui.separator();
		
		ui.checkbox(&mut self.camera_in_place, "Caméra installée");
		ui.checkbox(&mut self.object_in_place, "Objet en place");
	}
	
	fn central_panel(&mut self, ctx: &Context, _frame: &mut Frame, ui: &mut Ui) {
		ui.vertical_centered(|ui| {
			ui.horizontal(|ui| {
				if ui.button("⟳").on_hover_text("reset A").clicked() { self.c = 0. };
				ui.add(Slider::new(&mut self.c, -5.0..=5.)); // .show_value(false)
				
				ui.group(|ui| {
					ui.selectable_value(&mut self.c, -5., "-0.5");
					ui.selectable_value(&mut self.c, 0., "0");
					ui.selectable_value(&mut self.c, 5., "0.5");
				});
			});
		});
		
		ui.add(
			Image::new(egui::include_image!("../assets/gear_icon.png"))
			//.rounding(5.0)
		);
	}
}


impl App for TowerControl {
	fn update(&mut self, ctx: &Context, frame: &mut Frame) {
		install_image_loaders(ctx);
		// Basically the layout of the panels
		egui::CentralPanel::default().show(ctx, |ui| {
			egui::TopBottomPanel::top("top_panel")
				.show_inside(ui, |ui| self.menus(ctx, frame, ui));
			
			egui::SidePanel::left("left_panel")
				.resizable(false)
				.default_width(200.0)
				.show_inside(ui, |ui| self.left_panel(ctx, frame, ui));
			
			egui::SidePanel::right("right_panel")
				.resizable(false)
				.default_width(200.0)
				//.width_range(150.0..=300.0)
				.show_inside(ui, |ui| self.right_panel(ctx, frame, ui));
			
			egui::CentralPanel::default().show_inside(ui, |ui| {
				self.central_panel(ctx, frame, ui);
			});
		});
		ctx.request_repaint();
	}
}
