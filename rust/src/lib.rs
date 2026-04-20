use std::fs;
use jni::objects::JClass;
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_vuzt_MainActivity_getSystemInfoNative(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let mut final_output = String::new();

    // 1. INFO RAM
    final_output.push_str("--- MEMORY ---\n");
    if let Ok(data) = fs::read_to_string("/proc/meminfo") {
        for line in data.lines().take(3) {
            final_output.push_str(line);
            final_output.push('\n');
        }
    }

    // 2. INFO CPU (Ambil Model Name)
    final_output.push_str("\n--- CPU ---\n");
    if let Ok(data) = fs::read_to_string("/proc/cpuinfo") {
        if let Some(model) = data.lines().find(|l| l.contains("Hardware") || l.contains("model name")) {
            final_output.push_str(model);
            final_output.push('\n');
        }
    }

    // 3. INFO BATERAI (Kapasitas & Status)
    final_output.push_str("\n--- BATTERY ---\n");
    let cap = fs::read_to_string("/sys/class/power_supply/battery/capacity")
        .unwrap_or_else(|_| "N/A".to_string());
    let stat = fs::read_to_string("/sys/class/power_supply/battery/status")
        .unwrap_or_else(|_| "Unknown".to_string());
    
    final_output.push_str(&format!("Level: {}%\nStatus: {}", cap.trim(), stat.trim()));

    let response = env.new_string(final_output).expect("Gagal buat string");
    response.into_raw()
}
