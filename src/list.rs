use std::fs;

use crate::meta::{base_dir, Meta};

pub fn run() {
    let base = base_dir();
    if !base.exists() {
        println!("no active captures");
        return;
    }

    let mut entries: Vec<(String, Meta, bool)> = Vec::new();

    for entry in fs::read_dir(&base).expect("read ~/.capture") {
        let entry = entry.expect("read entry");
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let meta_path = path.join("meta.json");
        if let Ok(data) = fs::read_to_string(&meta_path) {
            if let Ok(meta) = serde_json::from_str::<Meta>(&data) {
                let alive = unsafe { libc::kill(meta.pid as i32, 0) == 0 };
                entries.push((name, meta, alive));
            }
        }
    }

    if entries.is_empty() {
        println!("no active captures");
        return;
    }

    entries.sort_by(|a, b| a.0.cmp(&b.0));

    println!("{:<16} {:<8} {:<10} {:<24} {}", "NAME", "PID", "STATUS", "STARTED", "COMMAND");
    for (name, meta, alive) in &entries {
        let status = if *alive { "running" } else { "dead" };
        let cmd = meta.command.join(" ");
        println!("{:<16} {:<8} {:<10} {:<24} {}", name, meta.pid, status, meta.started_at, cmd);
    }
}
