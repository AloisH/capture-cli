use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Instant;

use crate::meta::{capture_dir, Meta};

pub fn run(name: &str, command: &[String]) {
    let dir = capture_dir(name);

    if dir.exists() {
        fs::remove_dir_all(&dir).expect("failed to remove existing capture");
    }

    fs::create_dir_all(&dir).expect("failed to create capture dir");

    let mut child = match Command::new(&command[0])
        .args(&command[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: failed to spawn '{}': {e}", command[0]);
            eprintln!("hint: interactive CLIs (e.g. claude, vim, htop) are not supported");
            std::process::exit(1);
        }
    };

    let started = Instant::now();

    let meta = Meta {
        pid: child.id(),
        command: command.to_vec(),
        started_at: now(),
    };
    let json = serde_json::to_string_pretty(&meta).expect("serialize meta");
    fs::write(dir.join("meta.json"), json).expect("write meta.json");

    // Tee stdout → terminal + file
    let child_stdout = child.stdout.take().unwrap();
    let stdout_path = dir.join("stdout.log");
    let t1 = thread::spawn(move || {
        tee(child_stdout, &stdout_path, io::stdout());
    });

    // Tee stderr → terminal + file
    let child_stderr = child.stderr.take().unwrap();
    let stderr_path = dir.join("stderr.log");
    let t2 = thread::spawn(move || {
        tee(child_stderr, &stderr_path, io::stderr());
    });

    t1.join().unwrap();
    t2.join().unwrap();
    let status = child.wait().expect("wait on child");

    let code = status.code().unwrap_or(1);
    if code != 0 && started.elapsed().as_secs() < 2 {
        eprintln!("hint: interactive CLIs (e.g. claude, vim, htop) are not supported");
    }

    std::process::exit(code);
}

fn tee(source: impl io::Read, log_path: &std::path::Path, mut terminal: impl Write) {
    let mut log = File::create(log_path).expect("create log file");
    let reader = BufReader::new(source);
    for line in reader.split(b'\n') {
        let data = match line {
            Ok(d) => d,
            Err(_) => break,
        };
        let _ = log.write_all(&data);
        let _ = log.write_all(b"\n");
        let _ = log.flush();
        let _ = terminal.write_all(&data);
        let _ = terminal.write_all(b"\n");
        let _ = terminal.flush();
    }
}

fn now() -> String {
    let output = Command::new("date")
        .arg("-u")
        .arg("+%Y-%m-%dT%H:%M:%SZ")
        .output()
        .expect("date command failed");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
