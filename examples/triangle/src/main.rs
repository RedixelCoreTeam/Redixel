use redixel::Color;
use redixel::Game;
use redixel::GameContext;
use redixel::Vec2;

struct Triangle;

impl Game for Triangle {
    fn on_start(&mut self, _ctx: &mut dyn GameContext) {
        log::info!("triangle::on_start");
    }

    fn on_update(&mut self, _ctx: &mut dyn GameContext) {}

    fn on_render(&mut self, ctx: &mut dyn GameContext) {
        let (w, h): (f32, f32) = ctx.screen_size();
        let center_x: f32 = w / 2.0;
        let center_y: f32 = h / 2.0;

        let min_dimension: f32 = w.min(h);

        let scale_factor: f32 = 0.20;
        let triangle_size: f32 = min_dimension * scale_factor;

        let half_size: f32 = triangle_size / 2.0;

        let top: Vec2 = Vec2::new(center_x, center_y - half_size);
        let left: Vec2 = Vec2::new(center_x - half_size, center_y + half_size);
        let right: Vec2 = Vec2::new(center_x + half_size, center_y + half_size);

        ctx.draw_triangle(top, left, right, Color::rgb(1.0, 0.5, 0.0));
    }
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
        eprintln!("Engine error: {e}");
        std::process::exit(1);
    }
}
