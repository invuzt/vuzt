use std::fs;
use jni::objects::JClass;
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_vuzt_MainActivity_getSystemInfoNative(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let mut out = String::new();

    // 1. Uptime (Lama HP nyala)
    if let Ok(uptime) = fs::read_to_string("/proc/uptime") {
        let secs = uptime.split_whitespace().next().unwrap_or("0");
        let hours = secs.parse::<f32>().unwrap_or(0.0) / 3600.0;
        out.push_str(&format!("⏱️ UPTIME: {:.2} Hours\n", hours));
    }

    // 2. Kernel Version
    if let Ok(ver) = fs::read_to_string("/proc/version") {
        let short_ver = ver.split_whitespace().take(3).collect::<Vec<&str>>().join(" ");
        out.push_str(&format!("🐧 KERNEL: {}\n", short_ver));
    }

    // 3. RAM Info (Yang lama tetap ada)
    out.push_str("\n--- MEMORY ---\n");
    if let Ok(data) = fs::read_to_string("/proc/meminfo") {
        for line in data.lines().take(3) {
            out.push_str(line);
            out.push('\n');
        }
    }

    let response = env.new_string(out).expect("Error");
    response.into_raw()
}
