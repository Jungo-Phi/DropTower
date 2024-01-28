use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct CrashData {
	pub name: String,
	pub author: String,
	pub comment: String,
	//date: u64,
}
