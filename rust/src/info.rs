pub fn get_ram_info() -> String {
    let data = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mut total = 0;
    let mut avail = 0;
    
    for line in data.lines() {
        if line.contains("MemTotal") {
            total = parse_kb(line);
        } else if line.contains("MemAvailable") {
            avail = parse_kb(line);
        }
    }
    format!("{}|{}", total, avail) // Kirim data mentah dipisah pipa
}

fn parse_kb(line: &str) -> u64 {
    line.split_whitespace()
        .nth(1)
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap_or(0)
}
