#![no_std]

#[no_mangle]
pub fn c(a: f32, b: f32, op: i32) -> f32 {
    match op {
        1 => a + b,
        2 => a - b,
        3 => a * b,
        4 => a / b,
        _ => 0.0,
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }

