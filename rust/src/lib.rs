mod info;
mod notes;

use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_vuzt_MainActivity_getSystemInfoNative(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let data = info::get_ram_info();
    env.new_string(data).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_vuzt_MainActivity_saveNoteNative(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
    content: JString,
) {
    let p: String = env.get_string(&path).unwrap().into();
    let c: String = env.get_string(&content).unwrap().into();
    notes::save(&p, &c);
}

#[no_mangle]
pub extern "system" fn Java_com_vuzt_MainActivity_readNoteNative(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
) -> jstring {
    let p: String = env.get_string(&path).unwrap().into();
    let data = notes::read(&p);
    env.new_string(data).unwrap().into_raw()
}
