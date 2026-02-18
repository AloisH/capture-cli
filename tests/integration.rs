#[allow(deprecated)]
use assert_cmd::{cargo::cargo_bin, cargo::cargo_bin_cmd, Command};
use predicates::prelude::*;
use std::thread;
use std::time::Duration;
use tempfile::tempdir;

fn capture() -> Command {
    cargo_bin_cmd!("capture")
}

fn wait_for_meta(dir: &std::path::Path) {
    for _ in 0..50 {
        if dir.join("meta.json").exists() {
            return;
        }
        thread::sleep(Duration::from_millis(100));
    }
    panic!("meta.json not created in time");
}

// --- start ---

#[test]
fn start_creates_dir_meta_logs() {
    let tmp = tempdir().unwrap();
    capture()
        .env("HOME", tmp.path())
        .args(["start", "--name", "foo", "--", "echo", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"));

    let dir = tmp.path().join(".capture/foo");
    assert!(dir.join("meta.json").exists());
    assert!(dir.join("stdout.log").exists());

    let meta: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join("meta.json")).unwrap()).unwrap();
    assert_eq!(meta["command"], serde_json::json!(["echo", "hello"]));
}

#[test]
fn start_duplicate_name_errors() {
    let tmp = tempdir().unwrap();
    capture()
        .env("HOME", tmp.path())
        .args(["start", "--name", "foo", "--", "echo", "hi"])
        .assert()
        .success();
    capture()
        .env("HOME", tmp.path())
        .args(["start", "--name", "foo", "--", "echo", "hi"])
        .assert()
        .code(1);
}

// --- logs ---

#[test]
fn logs_default() {
    let tmp = tempdir().unwrap();
    capture()
        .env("HOME", tmp.path())
        .args(["start", "--name", "foo", "--", "echo", "hello"])
        .assert()
        .success();
    capture()
        .env("HOME", tmp.path())
        .args(["logs", "foo"])
        .assert()
        .success()
        .stdout("hello\n");
}

#[test]
fn logs_lines() {
    let tmp = tempdir().unwrap();
    capture()
        .env("HOME", tmp.path())
        .args(["start", "--name", "foo", "--", "sh", "-c", "echo a; echo b; echo c; echo d"])
        .assert()
        .success();
    capture()
        .env("HOME", tmp.path())
        .args(["logs", "foo", "--lines", "2"])
        .assert()
        .success()
        .stdout("c\nd\n");
}

#[test]
fn logs_head() {
    let tmp = tempdir().unwrap();
    capture()
        .env("HOME", tmp.path())
        .args(["start", "--name", "foo", "--", "sh", "-c", "echo a; echo b; echo c; echo d"])
        .assert()
        .success();
    capture()
        .env("HOME", tmp.path())
        .args(["logs", "foo", "--head", "2"])
        .assert()
        .success()
        .stdout("a\nb\n");
}

#[test]
fn logs_grep() {
    let tmp = tempdir().unwrap();
    capture()
        .env("HOME", tmp.path())
        .args(["start", "--name", "foo", "--", "sh", "-c", "echo apple; echo banana; echo apricot"])
        .assert()
        .success();
    capture()
        .env("HOME", tmp.path())
        .args(["logs", "foo", "--grep", "ap"])
        .assert()
        .success()
        .stdout("apple\napricot\n");
}

#[test]
fn logs_stderr_flag() {
    let tmp = tempdir().unwrap();
    capture()
        .env("HOME", tmp.path())
        .args(["start", "--name", "foo", "--", "sh", "-c", "echo err >&2"])
        .assert()
        .success();
    capture()
        .env("HOME", tmp.path())
        .args(["logs", "foo", "--stderr"])
        .assert()
        .success()
        .stdout("err\n");
}

#[test]
fn logs_missing_errors() {
    let tmp = tempdir().unwrap();
    capture()
        .env("HOME", tmp.path())
        .args(["logs", "nope"])
        .assert()
        .code(1);
}

// --- stop ---

#[test]
fn stop_kills_and_removes_dir() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let mut child = std::process::Command::new(cargo_bin!("capture"))
        .env("HOME", home)
        .args(["start", "--name", "foo", "--", "sleep", "60"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();

    let cap_dir = home.join(".capture/foo");
    wait_for_meta(&cap_dir);

    capture()
        .env("HOME", home)
        .args(["stop", "foo"])
        .assert()
        .success();

    assert!(!cap_dir.exists());
    let _ = child.kill();
    let _ = child.wait();
}

#[test]
fn stop_all_captures() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let mut c1 = std::process::Command::new(cargo_bin!("capture"))
        .env("HOME", home)
        .args(["start", "--name", "a", "--", "sleep", "60"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();

    let mut c2 = std::process::Command::new(cargo_bin!("capture"))
        .env("HOME", home)
        .args(["start", "--name", "b", "--", "sleep", "60"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();

    wait_for_meta(&home.join(".capture/a"));
    wait_for_meta(&home.join(".capture/b"));

    capture()
        .env("HOME", home)
        .args(["stop", "--all"])
        .assert()
        .success();

    assert!(!home.join(".capture/a").exists());
    assert!(!home.join(".capture/b").exists());

    let _ = c1.kill();
    let _ = c1.wait();
    let _ = c2.kill();
    let _ = c2.wait();
}

#[test]
fn stop_missing_errors() {
    let tmp = tempdir().unwrap();
    capture()
        .env("HOME", tmp.path())
        .args(["stop", "nope"])
        .assert()
        .code(1);
}

// --- list ---

#[test]
fn list_empty() {
    let tmp = tempdir().unwrap();
    capture()
        .env("HOME", tmp.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("no active captures"));
}

#[test]
fn list_shows_running_capture() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let mut child = std::process::Command::new(cargo_bin!("capture"))
        .env("HOME", home)
        .args(["start", "--name", "srv", "--", "sleep", "60"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();

    wait_for_meta(&home.join(".capture/srv"));

    capture()
        .env("HOME", home)
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("srv"))
        .stdout(predicate::str::contains("running"))
        .stdout(predicate::str::contains("sleep 60"));

    let _ = child.kill();
    let _ = child.wait();
}

#[test]
fn list_shows_dead_capture() {
    let tmp = tempdir().unwrap();
    capture()
        .env("HOME", tmp.path())
        .args(["start", "--name", "done", "--", "echo", "bye"])
        .assert()
        .success();

    capture()
        .env("HOME", tmp.path())
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("done"))
        .stdout(predicate::str::contains("dead"));
}

#[test]
fn list_shows_multiple_captures() {
    let tmp = tempdir().unwrap();
    let home = tmp.path();
    let mut c1 = std::process::Command::new(cargo_bin!("capture"))
        .env("HOME", home)
        .args(["start", "--name", "alpha", "--", "sleep", "60"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();

    let mut c2 = std::process::Command::new(cargo_bin!("capture"))
        .env("HOME", home)
        .args(["start", "--name", "beta", "--", "sleep", "60"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();

    wait_for_meta(&home.join(".capture/alpha"));
    wait_for_meta(&home.join(".capture/beta"));

    capture()
        .env("HOME", home)
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha"))
        .stdout(predicate::str::contains("beta"));

    let _ = c1.kill();
    let _ = c1.wait();
    let _ = c2.kill();
    let _ = c2.wait();
}
