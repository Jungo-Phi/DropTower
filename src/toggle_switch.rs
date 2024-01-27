use eframe::egui::{Ui, Response, vec2, Sense, WidgetInfo, WidgetType, lerp, pos2, Widget};

pub struct Toggle;

impl Toggle {
	pub fn new(on: &mut bool) -> impl Widget + '_ {
		move |ui: &mut Ui| {
			let desired_size = ui.spacing().interact_size.y * vec2(2.0, 1.0);
			let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());
			if response.clicked() {
				*on = !*on;
				response.mark_changed();
			}
			response.widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, *on, ""));
			
			if ui.is_rect_visible(rect) {
				let how_on = ui.ctx().animate_bool(response.id, *on);
				let visuals = ui.style().interact_selectable(&response, *on);
				let rect = rect.expand(visuals.expansion);
				let radius = 0.5 * rect.height();
				ui.painter()
					.rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
				let circle_x = lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
				let center = pos2(circle_x, rect.center().y);
				ui.painter()
					.circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
			}
			
			response
		}
	}
}
