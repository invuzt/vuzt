mod info;
mod notes;

use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

// SAPU JAGAT: Samakan semua 'mut env: JNIEnv'

#[no_mangle]
pub extern "system" fn Java_com_vuzt_MainActivity_getSystemInfoNative(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let data = info::get_vault_stats(&d);
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

#[no_mangle]
pub extern "system" fn Java_com_vuzt_MainActivity_deleteNoteNative(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
) {
    let p: String = env.get_string(&path).unwrap().into();
    notes::delete(&p);
}

#[no_mangle]
pub extern "system" fn Java_com_vuzt_MainActivity_listNotesNative(
    mut env: JNIEnv,
    _class: JClass,
    dir: JString,
) -> jstring {
    let d: String = env.get_string(&dir).unwrap().into();
    let data = notes::list_all(&d);
    env.new_string(data).unwrap().into_raw()
}
