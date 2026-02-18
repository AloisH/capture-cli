use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub pid: u32,
    pub command: Vec<String>,
    pub started_at: String,
}

pub fn base_dir() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME not set");
    PathBuf::from(home).join(".capture")
}

pub fn capture_dir(name: &str) -> PathBuf {
    base_dir().join(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_dir_path() {
        unsafe { std::env::set_var("HOME", "/tmp/test-capture-dir") };
        assert_eq!(
            capture_dir("foo"),
            PathBuf::from("/tmp/test-capture-dir/.capture/foo")
        );
    }

    #[test]
    fn meta_serde_roundtrip() {
        let m = Meta {
            pid: 42,
            command: vec!["echo".into(), "hi".into()],
            started_at: "2025-01-01T00:00:00Z".into(),
        };
        let json = serde_json::to_string(&m).unwrap();
        let m2: Meta = serde_json::from_str(&json).unwrap();
        assert_eq!(m2.pid, 42);
        assert_eq!(m2.command, vec!["echo", "hi"]);
        assert_eq!(m2.started_at, "2025-01-01T00:00:00Z");
    }
}
