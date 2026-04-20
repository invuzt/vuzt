pub fn get_ram_info() -> String {
    std::fs::read_to_string("/proc/meminfo").unwrap_or("Error".into())
        .lines().take(3).collect::<Vec<_>>().join("\n")
}
