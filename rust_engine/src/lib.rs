use std::fs;
use jni::objects::JClass;
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_vuzt_MainActivity_cekRamNative(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let mut output = String::from("--- RAM INFO (RUST JNI NATIVE) ---\n");
    
    // Membaca file sistem secara langsung
    if let Ok(data) = fs::read_to_string("/proc/meminfo") {
        for line in data.lines().take(6) {
            output.push_str(line);
            output.push('\n');
        }
    } else {
        output.push_str("Gagal mengakses /proc/meminfo");
    }

    let response = env.new_string(output).expect("Gagal buat string");
    response.into_raw()
}
