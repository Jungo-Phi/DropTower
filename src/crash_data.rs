use chrono::{DateTime, Utc, Local, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct CrashData {
	pub name: String,
	pub author: String,
	pub comment: String,
	pub date: DateTime<Utc>,
}
