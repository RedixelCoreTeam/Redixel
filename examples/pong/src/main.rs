fn main() {
    #[cfg(target_arch = "wasm32")]
    if let Err(e) = pong::wasm_main() {
        eprintln!("{e:?}");
    }

    #[cfg(not(target_arch = "wasm32"))]
    pong::main();
}
