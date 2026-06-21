fn main() {
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    if let Err(e) = pong::desktop_main() {
        eprintln!("Engine error: {e:?}");
        std::process::exit(0);
    }

    #[cfg(target_arch = "wasm32")]
    if let Err(e) = pong::wasm_main() {
        panic!("Engine error: {e:?}");
    }
}
