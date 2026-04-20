use std::fs;

pub fn save(path: &str, content: &str) {
    fs::write(path, content).ok();
}

pub fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

pub fn delete(path: &str) {
    fs::remove_file(path).ok();
}

pub fn list_all(dir: &str) -> String {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "txt") {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    files.push(filename.to_string());
                }
            }
        }
    }
    files.join("|")
}
