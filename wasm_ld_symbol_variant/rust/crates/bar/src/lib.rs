#[no_mangle]
pub extern "C" fn init() -> i32 {
    0
}

#[cfg(feature = "control")]
#[no_mangle]
pub extern "C" fn control() {}
