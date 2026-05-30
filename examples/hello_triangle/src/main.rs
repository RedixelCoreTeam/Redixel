use redixel::run;

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("Falha ao iniciar logger WASM");
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }

    log::info!("Starting Hello Triangle using Redixel Engine...");

    if let Err(e) = run() {
        log::error!("The engine crashed with a fatal error: {e}");
    }

    log::info!("Engine exited successfully.");
}
