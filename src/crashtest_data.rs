use chrono::{DateTime, Local};
use eframe::egui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct CrashtestResults {
	speed: f32,
}

impl CrashtestResults {
	pub fn new(height: &f32) -> Self {
		Self { speed: (2. * height / 9.81).sqrt() }
	}
	
	pub fn get_speed(&self) -> &f32 {
		&self.speed
	}
}


#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct CrashtestData {
	name: String,
	author: String,
	comment: String,
	date_time: DateTime<Local>,
	results: CrashtestResults,
}

impl CrashtestData {
	pub fn reset(&mut self) { *self = Self::default(); }
	pub fn push_results(&mut self, results: CrashtestResults) { self.results = results; }
	pub fn get_name(&self) -> &String { &self.name }
	pub fn set_date_time(&mut self, date_time: DateTime<Local>) { self.date_time = date_time; }
	pub fn show_data(&self, ui: &mut Ui) {
		ui.horizontal(|ui| {
			ui.label("Nom du test:");
			ui.strong(&self.name);
		});
		ui.horizontal(|ui| {
			ui.label("Auteur:");
			ui.strong(&self.author);
		});
		ui.horizontal(|ui| {
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
}
