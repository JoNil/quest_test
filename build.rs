use std::{env, error::Error, path::Path, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let ndk_root = PathBuf::from(env::var("ANDROID_NDK_ROOT")?);

    let native_app_glue_path = ndk_root.join(Path::new("sources/android/native_app_glue"));
    let vr_api_path = env::current_dir()?.join("thirdparty/VrApi/Include");

    println!("cargo:rustc-link-search=thirdparty/VrApi/Libs/Android/arm64-v8a/Release");
    println!("cargo:rustc-cdylib-link-arg=-lvrapi");
    println!("cargo:rustc-cdylib-link-arg=-lEGL");
    println!("cargo:rustc-cdylib-link-arg=-lGLESv3");

    cc::Build::new()
        .file("cpp/vr_main.c")
        .include(&native_app_glue_path)
        .include(&vr_api_path)
        .compile("vr_main");

    Ok(())
}
