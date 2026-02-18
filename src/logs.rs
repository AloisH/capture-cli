use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::thread;
use std::time::Duration;

use crate::meta::capture_dir;

pub fn run(
    name: &str,
    lines: Option<usize>,
    head: Option<usize>,
    grep: Option<&str>,
    follow: bool,
    stderr: bool,
) {
    let dir = capture_dir(name);
    if !dir.exists() {
        eprintln!("error: no capture '{name}'");
        std::process::exit(1);
    }

    let log_file = if stderr { "stderr.log" } else { "stdout.log" };
    let path = dir.join(log_file);

    if follow {
        tail_follow(&path, grep);
        return;
    }

    let file = File::open(&path).expect("open log file");
    let reader = BufReader::new(file);
    let mut all: Vec<String> = reader.lines().map(|l| l.expect("read line")).collect();

    // Filter
    if let Some(pat) = grep {
        all.retain(|l| l.contains(pat));
    }

    // Slice
    let slice: &[String] = if let Some(n) = head {
        &all[..n.min(all.len())]
    } else if let Some(n) = lines {
        let start = all.len().saturating_sub(n);
        &all[start..]
    } else {
        &all
    };

    for line in slice {
        println!("{line}");
    }
}

fn tail_follow(path: &std::path::Path, grep: Option<&str>) {
    let mut file = File::open(path).expect("open log file");
    // Start from end
    file.seek(SeekFrom::End(0)).expect("seek");
    let mut reader = BufReader::new(file);

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => {
                thread::sleep(Duration::from_millis(100));
            }
            Ok(_) => {
                let trimmed = line.trim_end_matches('\n');
                if let Some(pat) = grep {
                    if !trimmed.contains(pat) {
                        continue;
                    }
                }
                println!("{trimmed}");
            }
            Err(e) => {
                eprintln!("error reading log: {e}");
                break;
            }
        }
    }
}
