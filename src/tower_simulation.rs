use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CrashResults {
	speed: f32,
}
impl CrashResults {
	pub fn get_speed(&self) -> &f32 {
		&self.speed
	}
}


#[derive(Debug, Default)]
pub struct TowerSimulation {
	height: f32,
	impactor_charged: bool,
	clamps_closed: bool,
	magnet_closed: bool,
	door_closed: bool,
}

impl TowerSimulation {
	pub const MAX_HEIGHT: f32 = 23.;
	
	pub fn change_height(&mut self, delta: f32) {
		self.height += delta;
		self.height = self.height.clamp(0., Self::MAX_HEIGHT);
	}
	pub fn get_height(&self) -> &f32 { &self.height }
	pub fn is_impactor_charged(&self) -> &bool { &self.impactor_charged }
	pub fn is_clamps_closed(&self) -> &bool { &self.clamps_closed }
	pub fn is_magnet_closed(&self) -> &bool { &self.magnet_closed }
	pub fn is_door_closed(&self) -> &bool { &self.door_closed }
}