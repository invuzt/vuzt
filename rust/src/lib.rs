use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use std::fs;
use std::path::Path;

#[no_mangle]
pub extern "system" fn Java_com_vuzt_MainActivity_saveNoteNative(
    env: JNIEnv,
    _class: JClass,
    path: JString,
    content: JString,
) {
    let path_str: String = env.get_string(path).expect("Gagal ambil path").into();
    let content_str: String = env.get_string(content).expect("Gagal ambil konten").into();
    
    fs::write(path_str, content_str).expect("Gagal menulis file");
}

#[no_mangle]
pub extern "system" fn Java_com_vuzt_MainActivity_readNoteNative(
    env: JNIEnv,
    _class: JClass,
    path: JString,
) -> jstring {
    let path_str: String = env.get_string(path).expect("Gagal ambil path").into();
    
    let content = if Path::new(&path_str).exists() {
        fs::read_to_string(path_str).unwrap_or_else(|_| "".to_string())
    } else {
        "Belum ada catatan.".to_string()
    };

    let response = env.new_string(content).expect("Gagal buat string");
    response.into_raw()
}
