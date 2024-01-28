use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CrashData {
    author: String,
    comment: String,
    //date: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CrashResults {
    speed: f64,
}
