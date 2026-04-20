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

    // 1. RAM INFO
    final_output.push_str("--- MEMORY ---\n");
    if let Ok(data) = fs::read_to_string("/proc/meminfo") {
        for line in data.lines().take(3) {
            final_output.push_str(line);
            final_output.push('\n');
        }
    }

    // 2. CPU INFO (Cari di Hardware atau Processor)
    final_output.push_str("\n--- CPU ---\n");
    if let Ok(data) = fs::read_to_string("/proc/cpuinfo") {
        let model = data.lines()
            .find(|l| l.contains("Hardware") || l.contains("model name") || l.contains("Processor"))
            .unwrap_or("CPU Info: Restricted/Not Found");
        final_output.push_str(model);
        final_output.push('\n');
    }

    // 3. BATTERY INFO (Cari di lokasi alternatif)
    final_output.push_str("\n--- BATTERY ---\n");
    let paths = [
        "/sys/class/power_supply/battery/capacity",
        "/sys/class/power_supply/bms/capacity"
    ];
    
    let mut cap = String::from("N/A");
    for p in &paths {
        if let Ok(c) = fs::read_to_string(p) {
            cap = c.trim().to_string();
            break;
        }
    }
    
    final_output.push_str(&format!("Level: {}%\n", cap));
    
    let response = env.new_string(final_output).expect("Error string");
    response.into_raw()
}
