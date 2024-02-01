use chrono::{DateTime, Local};
use eframe::egui::Ui;
use serde::{Deserialize, Serialize};
use egui_plot::{Plot, Line};


#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct CrashtestResults {
	speed: f32,
	force: Vec<f32>,
}

impl CrashtestResults {
	pub fn new(height: &f32) -> Self {
		let force = (0..100).into_iter().map(|i| (i as f32).to_radians().cos()).collect();
		Self { speed: (2. * height / 9.81).sqrt(), force }
	}
}


#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct CrashtestData {
	name: String,
	author: String,
	comment: String,
	date_time: DateTime<Local>,
	drop_height: f32,
	results: CrashtestResults,
}

impl CrashtestData {
	pub fn reset(&mut self) { *self = Self::default(); }
	pub fn set_results(&mut self, results: CrashtestResults, height: f32) {
		self.results = results;
		self.drop_height = height;
	}
	pub fn get_name(&self) -> &String { &self.name }
	pub fn set_date_time(&mut self, date_time: DateTime<Local>) { self.date_time = date_time; }
	pub fn show_data(&self, ui: &mut Ui) {
		ui.group(|ui| {
			ui.label("Nom du test:");
			ui.strong(&self.name);
			ui.label("Auteur:");
			ui.strong(&self.author);
			ui.label("Date:");
			ui.strong(format!("{}", &self.date_time.date_naive()));
			ui.label("Commentaire:");
			ui.strong(&self.comment);
		});
	}
	pub fn change_data(&mut self, ui: &mut Ui) {
		ui.horizontal(|ui| {
			ui.label("Nom du test:");
			ui.text_edit_singleline(&mut self.name);
		});
		ui.horizontal(|ui| {
			ui.label("Auteur:");
			ui.text_edit_singleline(&mut self.author);
		});
		ui.horizontal(|ui| {
			ui.label("Commentaire:");
			ui.text_edit_multiline(&mut self.comment);
		});
	}
	pub fn show_results(&self, ui: &mut Ui) {
		ui.group(|ui| {
			ui.horizontal(|ui| {
				ui.label("Speed:");
				ui.strong(format!("{:?}", self.results.speed));
			});
			let plot = vec![[0., 0.], [4., 3.], [6., 7.], [10., 10.]];
			Plot::new("Energy plot")
				.height(250.)
				.width(350.)
				.allow_boxed_zoom(false)
				.show(ui, |plot_ui| {
					plot_ui.line(Line::new(plot).name("Force"));
				});
		});
	}
}
