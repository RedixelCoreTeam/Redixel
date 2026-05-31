use redixel::Game;
use redixel::GameContext;

struct ClearColor {
    frame: u64,
}

impl Game for ClearColor {
    fn on_start(&mut self, _ctx: &mut dyn GameContext) {
        log::info!("clear_color::on_start");
    }

    fn on_update(&mut self, ctx: &mut dyn GameContext) {
        self.frame += 1;

        if self.frame.is_multiple_of(300) {
            log::info!("Frame {}: delta={:.4}s  fps={:.1}", self.frame, ctx.delta_time(), ctx.fps());
        }
    }

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

    if let Err(e) = redixel::run(ClearColor { frame: 0 }) {
        eprintln!("Engine error: {e}");
        std::process::exit(1);
    }
}
