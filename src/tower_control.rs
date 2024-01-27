use std::path::Path;
use eframe::{App, CreationContext, egui, Frame};
use eframe::egui::{Context, Ui, Slider};
use crate::toggle_switch;
use crate::utils::load_image_from_path;


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
				frame.close();
			}
		});
	}
	
	fn left_panel(&mut self, _ctx: &Context, _frame: &mut Frame, ui: &mut Ui) {
		ui.vertical_centered(|ui| {
			ui.heading("Parameters");
			ui.add_space(5.);
		});
		
		ui.checkbox(&mut self.a, "A");
		ui.label(format!("Selected: {}", self.b));
		ui.add(toggle_switch::toggle(&mut self.a));
	}
	
	fn right_panel(&mut self, _ctx: &Context, _frame: &mut Frame, ui: &mut Ui) {
		ui.vertical_centered(|ui| {
			ui.heading("Info");
			ui.add_space(5.);
		});
		
		let _ = ui.radio(true, "Impacteur chargé");
		let _ = ui.radio(false, "Clamps fermés");
		let _ = ui.radio(true, "Porte fermée");
		
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
		// let gear_image = load_image_from_path(Path::new("assets/gear_icon.png"));
		// let gear_texture = ctx.load_texture("gear-image", gear_image, Default::default());
		// ui.image(&gear_texture, gear_texture.size_vec2());
	}
}


impl App for TowerControl {
	fn update(&mut self, ctx: &Context, frame: &mut Frame) {
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
