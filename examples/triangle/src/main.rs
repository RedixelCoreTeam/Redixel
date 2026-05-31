use redixel::Game;
use redixel::GameContext;

struct Triangle;

impl Game for Triangle {
    fn on_start(&mut self, _ctx: &mut dyn GameContext) {
        log::info!("triangle::on_start");
    }

    fn on_update(&mut self, _ctx: &mut dyn GameContext) {}

    fn on_render(&mut self, _ctx: &mut dyn GameContext) {}
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("Falha ao iniciar logger WASM");
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    }

    if let Err(e) = redixel::run(Triangle) {
        eprintln!("Engine terminated with error: {e}");
        std::process::exit(1);
    }
}
