use redixel::prelude::*;

const ENTITY_SIZE: f32 = 24.0;
const PLAYER_SPEED: f32 = 300.0;
const BOT_SPEED: f32 = 180.0;
const BULLET_SIZE: f32 = 8.0;
const BULLET_SPEED: f32 = 600.0;
const BASE_COOLDOWN: f32 = 0.5;
const RAPID_FIRE_COOLDOWN: f32 = 0.15;
const POWERUP_DURATION: f32 = 5.0;
const POWERUP_SIZE: f32 = 16.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WeaponType {
    Pistol,
    Shotgun,
    Flamethrower,
    Homing,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Action {
    Up,
    Down,
    Left,
    Right,
    Shoot,
    Exit,
}

struct Bullet {
    pos: Vec2,
    vel: Vec2,
    owner_id: usize,
    weapon: WeaponType,
    life: f32,
    destroyed: bool,
    size: f32,
}

struct PowerUp {
    pos: Vec2,
    active: bool,
    weapon: WeaponType,
}

struct Agent {
    id: usize,
    pos: Vec2,
    health: i32,
    shoot_cooldown: f32,
    rapid_fire_timer: f32,
    is_player: bool,
    color: Color,
    weapon: WeaponType,
}

impl Agent {
    fn new(id: usize, x: f32, y: f32, is_player: bool) -> Self {
        let agent_color: Color = if is_player {
            Color::from_rgba8(50, 150, 255, 255)
        } else {
            Color::from_rgba8(255, 50, 50, 255)
        };

        Self {
            id,
            pos: Vec2::new(x, y),
            health: 100,
            shoot_cooldown: 0.0,
            rapid_fire_timer: 0.0,
            is_player,
            color: agent_color,
            weapon: WeaponType::Pistol,
        }
    }

    fn respawn(&mut self, w: f32, h: f32) {
        self.health = 100;
        self.rapid_fire_timer = 0.0;
        self.weapon = WeaponType::Pistol;
        self.pos.x = (self.pos.x + 400.0) % (w - ENTITY_SIZE);
        self.pos.y = (self.pos.y + 300.0) % (h - ENTITY_SIZE);
    }

    fn current_cooldown_max(&self) -> f32 {
        if self.rapid_fire_timer > 0.0 {
            RAPID_FIRE_COOLDOWN
        } else {
            BASE_COOLDOWN
        }
    }
}

struct Shooter {
    initialized: bool,
    agents: Vec<Agent>,
    bullets: Vec<Bullet>,
    powerup: PowerUp,
    time_since_start: f32,
}

impl Shooter {
    fn new() -> Self {
        let mut agents: Vec<Agent> = Vec::new();

        agents.push(Agent::new(0, 0.0, 0.0, true));

        let mut i: usize = 1;
        while i <= 5 {
            agents.push(Agent::new(i, i as f32 * 200.0, i as f32 * 100.0, false));
            i += 1;
        }

        Self {
            agents,
            initialized: false,
            bullets: Vec::new(),
            time_since_start: 0.0,
            powerup: PowerUp {
                active: true,
                weapon: WeaponType::Shotgun,
                pos: Vec2::new(0.0, 100.0),
            },
        }
    }

    fn check_collision(pos1: Vec2, size1: f32, pos2: Vec2, size2: f32) -> bool {
        pos1.x + size1 > pos2.x && pos1.x < pos2.x + size2 && pos1.y + size1 > pos2.y && pos1.y < pos2.y + size2
    }
}

impl Game for Shooter {
    type Action = Action;

    fn on_start(&mut self, ctx: &mut dyn GameContext<Self::Action>) {
        ctx.input_mut().bind(Action::Up, KeyCode::KeyW.into());
        ctx.input_mut().bind(Action::Down, KeyCode::KeyS.into());
        ctx.input_mut().bind(Action::Left, KeyCode::KeyA.into());
        ctx.input_mut().bind(Action::Right, KeyCode::KeyD.into());
        ctx.input_mut().bind(Action::Exit, KeyCode::Escape.into());
        ctx.input_mut().bind(Action::Shoot, MouseButton::Left.into());
    }

    fn on_update(&mut self, ctx: &mut dyn GameContext<Self::Action>) {
        let dt: f32 = ctx.delta_time() as f32;
        let w: f32 = ctx.surface_width() as f32;
        let h: f32 = ctx.surface_height() as f32;

        if !self.initialized {
            let screen_center: Vec2 = Vec2::new(w, h) / 2.0;

            if let Some(player) = self.agents.iter_mut().find(|a: &&mut Agent| a.is_player) {
                player.pos = screen_center - Vec2::splat(ENTITY_SIZE / 2.0);
            }

            self.powerup.pos = Vec2::new((w - POWERUP_SIZE) / 2.0, 100.0);
            self.initialized = true;
        }

        self.time_since_start += dt;

        if ctx.input().held(Action::Exit) {
            ctx.exit();
        }

        if !self.powerup.active && self.time_since_start % 10.0 < dt {
            self.powerup.active = true;
            self.powerup.pos = Vec2::new((self.time_since_start * 100.0) % w, (self.time_since_start * 50.0) % h);

            let weapon_cycle: usize = ((self.time_since_start / 10.0) as usize) % 4;
            self.powerup.weapon = match weapon_cycle {
                0 => WeaponType::Shotgun,
                1 => WeaponType::Flamethrower,
                2 => WeaponType::Homing,
                _ => WeaponType::Pistol,
            };
        }

        let mut player_pos: Vec2 = Vec2::ZERO;
        for agent in self.agents.iter() {
            if agent.is_player {
                player_pos = agent.pos;
            }
        }

        let agents_len: usize = self.agents.len();
        for i in 0..agents_len {
            let agent: &mut Agent = &mut self.agents[i];

            agent.shoot_cooldown -= dt;
            if agent.rapid_fire_timer > 0.0 {
                agent.rapid_fire_timer -= dt;

                if agent.rapid_fire_timer <= 0.0 {
                    agent.weapon = WeaponType::Pistol;
                }
            }

            if agent.is_player {
                let mut dir: Vec2 = Vec2::ZERO;

                if ctx.input().held(Action::Left) {
                    dir.x -= 1.0;
                }

                if ctx.input().held(Action::Right) {
                    dir.x += 1.0;
                }

                if ctx.input().held(Action::Up) {
                    dir.y -= 1.0;
                }

                if ctx.input().held(Action::Down) {
                    dir.y += 1.0;
                }

                if dir.x != 0.0 || dir.y != 0.0 {
                    dir = dir.normalise();
                    agent.pos += dir * (PLAYER_SPEED * dt);
                }

                if ctx.input().held(Action::Shoot)
                    && agent.shoot_cooldown <= 0.0
                    && let Some(mouse_pos) = ctx.input().mouse_position()
                {
                    let center: Vec2 = agent.pos + Vec2::splat(ENTITY_SIZE / 2.0);
                    let dir_raw: Vec2 = mouse_pos - center;
                    let mag: f32 = dir_raw.length();

                    if mag > 0.0 {
                        agent.shoot_cooldown = agent.current_cooldown_max();
                        let dir_vec: Vec2 = dir_raw / mag;

                        match agent.weapon {
                            WeaponType::Pistol => {
                                self.bullets.push(Bullet {
                                    pos: center,
                                    vel: dir_vec * BULLET_SPEED,
                                    owner_id: agent.id,
                                    weapon: WeaponType::Pistol,
                                    life: 3.0,
                                    destroyed: false,
                                    size: BULLET_SIZE,
                                });
                            }
                            WeaponType::Shotgun => {
                                let angles: [f32; 5] = [-0.3, -0.15, 0.0, 0.15, 0.3];

                                for &angle in &angles {
                                    let cos_a: f32 = angle.cos();
                                    let sin_a: f32 = angle.sin();
                                    let rot_vel: Vec2 = Vec2::new(
                                        dir_vec.x * cos_a - dir_vec.y * sin_a,
                                        dir_vec.x * sin_a + dir_vec.y * cos_a,
                                    );

                                    self.bullets.push(Bullet {
                                        pos: center,
                                        vel: rot_vel * BULLET_SPEED,
                                        owner_id: agent.id,
                                        weapon: WeaponType::Shotgun,
                                        life: 0.6,
                                        destroyed: false,
                                        size: BULLET_SIZE,
                                    });
                                }
                            }
                            WeaponType::Flamethrower => {
                                let offsets: [f32; 6] = [-0.4, -0.2, -0.05, 0.05, 0.2, 0.4];
                                let speeds: [f32; 6] = [0.35, 0.5, 0.6, 0.45, 0.55, 0.4];
                                let sizes: [f32; 6] = [10.0, 16.0, 20.0, 18.0, 14.0, 12.0];

                                for i in 0..6 {
                                    let wobble: f32 = (self.time_since_start * 25.0 + (i as f32)).sin() * 0.15;
                                    let angle: f32 = offsets[i] + wobble;

                                    let cos_a: f32 = angle.cos();
                                    let sin_a: f32 = angle.sin();
                                    let rot_vel: Vec2 = Vec2::new(
                                        dir_vec.x * cos_a - dir_vec.y * sin_a,
                                        dir_vec.x * sin_a + dir_vec.y * cos_a,
                                    );

                                    self.bullets.push(Bullet {
                                        pos: center - Vec2::splat(sizes[i] / 2.0),
                                        vel: rot_vel * (BULLET_SPEED * speeds[i]),
                                        owner_id: agent.id,
                                        weapon: WeaponType::Flamethrower,
                                        life: 0.35,
                                        destroyed: false,
                                        size: sizes[i],
                                    });
                                }
                            }
                            WeaponType::Homing => {
                                self.bullets.push(Bullet {
                                    pos: center,
                                    vel: dir_vec * (BULLET_SPEED * 0.6),
                                    owner_id: agent.id,
                                    weapon: WeaponType::Homing,
                                    life: 4.0,
                                    destroyed: false,
                                    size: BULLET_SIZE,
                                });
                            }
                        }
                    }
                }
            } else {
                let center: Vec2 = agent.pos + Vec2::splat(ENTITY_SIZE / 2.0);
                let target_center: Vec2 = player_pos + Vec2::splat(ENTITY_SIZE / 2.0);
                let dir_raw: Vec2 = target_center - center;
                let mag: f32 = dir_raw.length();

                if mag > 0.0 {
                    let dir_vec: Vec2 = dir_raw / mag;
                    let min_dist: f32 = match agent.weapon {
                        WeaponType::Flamethrower => 50.0,
                        WeaponType::Shotgun => 100.0,
                        _ => 150.0,
                    };

                    if mag > min_dist {
                        agent.pos += dir_vec * (BOT_SPEED * dt);
                    }

                    if agent.shoot_cooldown <= 0.0 {
                        agent.shoot_cooldown = agent.current_cooldown_max();

                        self.bullets.push(Bullet {
                            pos: center,
                            vel: dir_vec * BULLET_SPEED,
                            owner_id: agent.id,
                            weapon: agent.weapon,
                            life: 2.0,
                            destroyed: false,
                            size: BULLET_SIZE,
                        });
                    }
                }
            }

            agent.pos.x = agent.pos.x.clamp(0.0, w - ENTITY_SIZE);
            agent.pos.y = agent.pos.y.clamp(0.0, h - ENTITY_SIZE);

            if self.powerup.active && Shooter::check_collision(agent.pos, ENTITY_SIZE, self.powerup.pos, POWERUP_SIZE) {
                agent.weapon = self.powerup.weapon;
                agent.rapid_fire_timer = POWERUP_DURATION;
                self.powerup.active = false;
            }
        }

        let bullets_len: usize = self.bullets.len();
        for i in 0..bullets_len {
            let b: &mut Bullet = &mut self.bullets[i];

            if b.weapon == WeaponType::Homing && !b.destroyed {
                let mut closest_dist: f32 = f32::MAX;
                let mut target_pos: Vec2 = Vec2::ZERO;
                let mut found: bool = false;

                for j in 0..agents_len {
                    let agent: &Agent = &self.agents[j];

                    if agent.id != b.owner_id {
                        let dist_sq: f32 = (agent.pos - b.pos).length_sq();

                        if dist_sq < closest_dist {
                            closest_dist = dist_sq;
                            target_pos = agent.pos;
                            found = true;
                        }
                    }
                }

                if found && closest_dist < 60000.0 {
                    let target_dir: Vec2 = (target_pos - b.pos).normalise();
                    b.vel = (b.vel + target_dir * 15.0).normalise() * (BULLET_SPEED * 0.6);
                }
            }

            b.pos += b.vel * dt;
            b.life -= dt;

            if b.life <= 0.0 || b.pos.x < -b.size || b.pos.x > w || b.pos.y < -b.size || b.pos.y > h {
                b.destroyed = true;
            }
        }

        for i in 0..bullets_len {
            if self.bullets[i].destroyed {
                continue;
            }

            let pos_i: Vec2 = self.bullets[i].pos;
            let size_i: f32 = self.bullets[i].size;
            let owner_i: usize = self.bullets[i].owner_id;

            for agent in self.agents.iter_mut() {
                if agent.id != owner_i && Shooter::check_collision(pos_i, size_i, agent.pos, ENTITY_SIZE) {
                    agent.health -= 25;

                    if agent.health <= 0 {
                        agent.respawn(w, h);
                    }

                    self.bullets[i].destroyed = true;
                    break;
                }
            }
        }

        for i in 0..bullets_len {
            if self.bullets[i].destroyed {
                continue;
            }

            let pos_i: Vec2 = self.bullets[i].pos;
            let size_i: f32 = self.bullets[i].size;
            let owner_i: usize = self.bullets[i].owner_id;

            for j in (i + 1)..bullets_len {
                if self.bullets[j].destroyed {
                    continue;
                }

                if owner_i != self.bullets[j].owner_id {
                    let pos_j: Vec2 = self.bullets[j].pos;

                    if Shooter::check_collision(pos_i, size_i, pos_j, ENTITY_SIZE) {
                        self.bullets[i].destroyed = true;
                        self.bullets[j].destroyed = true;
                        break;
                    }
                }
            }
        }

        self.bullets.retain(|b: &Bullet| !b.destroyed);
    }

    fn on_render(&mut self, ctx: &mut dyn GameContext<Self::Action>) {
        ctx.clear_color(Color::rgb(0.1, 0.1, 0.15));

        if self.powerup.active {
            let pulse: f32 = (self.time_since_start * 5.0).sin().abs();

            let c: Color = match self.powerup.weapon {
                WeaponType::Shotgun => Color::from_rgba8(255, 165, 0, (150.0 + pulse * 100.0) as u8),
                WeaponType::Flamethrower => Color::from_rgba8(255, 50, 0, (150.0 + pulse * 100.0) as u8),
                WeaponType::Homing => Color::from_rgba8(50, 255, 255, (150.0 + pulse * 100.0) as u8),
                WeaponType::Pistol => Color::from_rgba8(200, 50, 255, (150.0 + pulse * 100.0) as u8),
            };

            ctx.draw_rect(self.powerup.pos, Vec2::splat(POWERUP_SIZE), c);
        }

        let agents_len: usize = self.agents.len();
        for i in 0..agents_len {
            let agent: &Agent = &self.agents[i];

            let color: Color = if agent.rapid_fire_timer > 0.0 {
                Color::from_rgba8(255, 200, 50, 255)
            } else {
                agent.color
            };

            ctx.draw_rect(agent.pos, Vec2::splat(ENTITY_SIZE), color);
        }

        let bullets_len: usize = self.bullets.len();
        for i in 0..bullets_len {
            let b: &Bullet = &self.bullets[i];

            let b_color: Color = match b.weapon {
                WeaponType::Pistol => Color::YELLOW,
                WeaponType::Shotgun => Color::from_rgba8(255, 165, 0, 255),
                WeaponType::Flamethrower => Color::from_rgba8(255, 50, 0, 255),
                WeaponType::Homing => Color::from_rgba8(50, 255, 255, 255),
            };

            ctx.draw_rect(b.pos, Vec2::splat(b.size), b_color);
        }

        if let Some(mouse_pos) = ctx.input().mouse_position() {
            ctx.draw_rect(mouse_pos - Vec2::new(2.0, 10.0), Vec2::new(4.0, 20.0), Color::WHITE);
            ctx.draw_rect(mouse_pos - Vec2::new(10.0, 2.0), Vec2::new(20.0, 4.0), Color::WHITE);
        }
    }
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("Failed to initialize WASM logger");
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    }

    if let Err(e) = redixel::run(Shooter::new()) {
        eprintln!("Engine error: {e}");
        std::process::exit(1);
    }
}
