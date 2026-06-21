#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
fn main() {
    if let Err(e) = triangle_3d::desktop_main() {
        eprintln!("Engine error: {e:?}");
        std::process::exit(0);
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    if let Err(e) = triangle_3d::wasm_main() {
        panic!("Engine error: {e:?}");
    }
}

#[cfg(target_os = "android")]
fn main() {
    triangle_3d::android_main();
}
