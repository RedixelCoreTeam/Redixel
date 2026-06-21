#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
fn main() {
    if let Err(e) = triangle::desktop_main() {
        eprintln!("Engine error: {e:?}");
        std::process::exit(0);
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    if let Err(e) = triangle::wasm_main() {
        panic!("Engine error: {e:?}");
    }
}

#[cfg(target_os = "android")]
fn main() {
    triangle::android_main();
}
