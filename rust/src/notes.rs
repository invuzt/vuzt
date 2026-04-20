use std::fs;
pub fn save(path: &str, content: &str) { fs::write(path, content).ok(); }
pub fn read(path: &str) -> String { fs::read_to_string(path).unwrap_or_default() }
