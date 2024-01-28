use eframe::egui::{Ui, vec2, Sense, Widget, Color32, WidgetText};


fn lamp_ui(on: &bool) -> impl Widget + '_ {
	move |ui: &mut Ui| {
		let desired_size = ui.spacing().interact_size.y * vec2(0.75, 1.);
		let (rect, response) = ui.allocate_exact_size(desired_size, Sense::focusable_noninteractive());
		if ui.is_rect_visible(rect) {
			let visuals = ui.style().interact_selectable(&response, *on);
			let rect = rect.expand(visuals.expansion);
			let fill_color = if *on { Color32::from_rgb(192, 239, 192) } else { Color32::from_rgb(255, 192, 192) };
			ui.painter().circle(rect.center(), 0.4 * rect.height(), fill_color, visuals.fg_stroke);
		}
		response
	}
}

pub fn add_lamp(ui: &mut Ui, on: &bool, text: impl Into<WidgetText>) {
	ui.horizontal(|ui| {
		ui.add(lamp_ui(on));
		ui.add_space(-3.);
		ui.label(text.into());
	});
 }
