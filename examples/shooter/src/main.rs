fn main() {
    #[cfg(target_arch = "wasm32")]
    if let Err(e) = shooter::wasm_main() {
        eprintln!("{e:?}");
    }

    #[cfg(not(target_arch = "wasm32"))]
    shooter::main();
}
