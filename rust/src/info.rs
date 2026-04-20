use std::fs;

pub fn get_vault_stats(dir: &str) -> String {
    let mut count = 0;
    let mut total_chars = 0;

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "txt") {
                count += 1;
                if let Ok(content) = fs::read_to_string(path) {
                    total_chars += content.len();
                }
            }
        }
    }
    // Return: "JumlahFile|TotalKarakter"
    format!("{}|{}", count, total_chars)
}
