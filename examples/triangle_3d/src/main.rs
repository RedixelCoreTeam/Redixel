use redixel::Color;
use redixel::Game;
use redixel::GameContext;
use redixel::Vec2;

struct Triangle3d {
    rotation: f32,
}

impl Game for Triangle3d {
    fn on_start(&mut self, _ctx: &mut dyn GameContext) {
        log::info!("triangle_3d::on_start");
    }

    fn on_update(&mut self, ctx: &mut dyn GameContext) {
        self.rotation += (ctx.delta_time() as f32) * 1.5;
    }

    fn on_render(&mut self, ctx: &mut dyn GameContext) {
        ctx.clear_color(Color::rgb(0.1, 0.1, 0.12));

        let (w, h) = ctx.screen_size();
        let center_x: f32 = w / 2.0;
        let center_y: f32 = h / 2.0;
        let scale: f32 = w.min(h) * 0.40;

        let vertices: [(f32, f32, f32); 4] = [(0.0, -0.8, 0.0), (-0.8, 0.6, 0.5), (0.8, 0.6, 0.5), (0.0, 0.6, -0.8)];

        let faces: [(usize, usize, usize, Color); 4] = [
            (0, 1, 2, Color::rgb(0.8, 0.2, 0.2)),
            (0, 2, 3, Color::rgb(0.2, 0.8, 0.2)),
            (0, 3, 1, Color::rgb(0.2, 0.2, 0.8)),
            (1, 3, 2, Color::rgb(0.8, 0.8, 0.1)),
        ];

        let rotate = |x: f32, y: f32, z: f32, angle: f32| -> (f32, f32, f32) {
            let rx: f32 = x * angle.cos() - z * angle.sin();
            let rz: f32 = x * angle.sin() + z * angle.cos();

            let tilt: f32 = 0.4;
            let ry: f32 = y * tilt.cos() - rz * tilt.sin();
            let rz_final: f32 = y * tilt.sin() + rz * tilt.cos();

            (rx, ry, rz_final)
        };

        let rotated_vertices: Vec<(f32, f32, f32)> = vertices
            .iter()
            .map(|v: &(f32, f32, f32)| rotate(v.0, v.1, v.2, self.rotation))
            .collect();

        let mut faces_to_draw = Vec::new();

        for (i1, i2, i3, color) in faces.iter() {
            let v1: (f32, f32, f32) = rotated_vertices[*i1];
            let v2: (f32, f32, f32) = rotated_vertices[*i2];
            let v3: (f32, f32, f32) = rotated_vertices[*i3];

            let avg_z: f32 = (v1.2 + v2.2 + v3.2) / 3.0;
            faces_to_draw.push((avg_z, v1, v2, v3, *color));
        }

        faces_to_draw.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        for (_, v1, v2, v3, color) in faces_to_draw {
            let project = |x: f32, y: f32, z: f32| -> Vec2 {
                let distance: f32 = 2.5;
                let z_perspective: f32 = distance + z;

                Vec2::new(center_x + (x / z_perspective) * scale, center_y + (y / z_perspective) * scale)
            };

            let p1: Vec2 = project(v1.0, v1.1, v1.2);
            let p2: Vec2 = project(v2.0, v2.1, v2.2);
            let p3: Vec2 = project(v3.0, v3.1, v3.2);

            ctx.draw_triangle(p1, p2, p3, color);
        }
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

    if let Err(e) = redixel::run(Triangle3d { rotation: 0.0 }) {
        eprintln!("Engine error: {e}");
        std::process::exit(1);
    }
}
