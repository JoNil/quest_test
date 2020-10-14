use std::{env, error::Error, path::Path, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let ndk_root = PathBuf::from(env::var("ANDROID_NDK_ROOT")?);

    let native_app_glue_path = ndk_root.join(Path::new("sources/android/native_app_glue"));
    let vr_api_path = env::current_dir()?.join("thirdparty/VrApi/Include");

    cc::Build::new()
        .file("cpp/VrCubeWorld_NativeActivity.c")
        .include(&native_app_glue_path)
        .include(&vr_api_path)
        .compile("vrcubeworld");

    Ok(())
}
