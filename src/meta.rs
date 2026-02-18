use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub pid: u32,
    pub command: Vec<String>,
    pub started_at: String,
}

pub fn capture_dir(name: &str) -> PathBuf {
    let home = std::env::var("HOME").expect("HOME not set");
    PathBuf::from(home).join(".capture").join(name)
}
