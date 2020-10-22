use jni_sys::{jobject, JavaVM};
use ndk_sys::ANativeWindow;
use std::ffi::c_void;

extern "C" {
    fn vr_main(java: *mut JavaVM, activity: jobject, native_window: *mut ANativeWindow) -> c_void;
}

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    let native_activity = ndk_glue::native_activity();
    let native_window = ndk_glue::native_window();
    unsafe {
        vr_main(
            native_activity.vm(),
            native_activity.activity(),
            native_window.as_ref().unwrap().ptr().as_ptr(),
        );
    }
}
