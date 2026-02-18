use std::fs;

use crate::meta::{capture_dir, Meta};

pub fn run(name: Option<&str>, all: bool) {
    if all {
        stop_all();
    } else if let Some(name) = name {
        stop_one(name);
    } else {
        eprintln!("error: provide a name or --all");
        std::process::exit(1);
    }
}

fn stop_one(name: &str) {
    let dir = capture_dir(name);
    if !dir.exists() {
        eprintln!("error: no capture '{name}'");
        std::process::exit(1);
    }

    kill_from_meta(&dir);
    fs::remove_dir_all(&dir).expect("remove capture dir");
    println!("stopped '{name}'");
}

fn stop_all() {
    let base = capture_dir("").parent().unwrap().to_path_buf();
    if !base.exists() {
        return;
    }
    let mut count = 0;
    for entry in fs::read_dir(&base).expect("read ~/.capture") {
        let entry = entry.expect("read entry");
        if entry.path().is_dir() {
            kill_from_meta(&entry.path());
            fs::remove_dir_all(entry.path()).expect("remove capture dir");
            count += 1;
        }
    }
    println!("stopped {count} capture(s)");
}

fn kill_from_meta(dir: &std::path::Path) {
    let meta_path = dir.join("meta.json");
    if let Ok(data) = fs::read_to_string(&meta_path) {
        if let Ok(meta) = serde_json::from_str::<Meta>(&data) {
            // SIGTERM; ignore error if already dead
            unsafe {
                libc::kill(meta.pid as i32, libc::SIGTERM);
            }
        }
    }
}
