use redixel::prelude::*;

const ENTITY_SIZE: f32 = 24.0;
const PLAYER_SPEED: f32 = 400.0;
const BOT_SPEED: f32 = 220.0;
const BULLET_SIZE: f32 = 8.0;
const BULLET_SPEED: f32 = 700.0;
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
    Dash,
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

struct Particle {
    pos: Vec2,
    vel: Vec2,
    life: f32,
    max_life: f32,
    r: u8,
    g: u8,
    b: u8,
    size: f32,
}

struct ParticleProps {
    pos: Vec2,
    color: (u8, u8, u8),
    count: usize,
    speed: f32,
    seed: f32,
    life: f32,
    size: f32,
}

struct Afterimage {
    pos: Vec2,
    life: f32,
    max_life: f32,
    r: u8,
    g: u8,
    b: u8,
}

struct PowerUp {
    pos: Vec2,
    active: bool,
    weapon: WeaponType,
}

struct Agent {
    id: usize,
    pos: Vec2,
    vel: Vec2,
    health: i32,
    shoot_cooldown: f32,
    rapid_fire_timer: f32,
    dash_cooldown: f32,
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
            vel: Vec2::ZERO,
            health: 100,
            shoot_cooldown: 0.0,
            rapid_fire_timer: 0.0,
            dash_cooldown: 0.0,
            is_player,
            color: agent_color,
            weapon: WeaponType::Pistol,
        }
    }

    fn respawn(&mut self, w: f32, h: f32) {
        self.health = 100;
        self.rapid_fire_timer = 0.0;
        self.dash_cooldown = 0.0;
        self.weapon = WeaponType::Pistol;
        self.vel = Vec2::ZERO;
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
    particles: Vec<Particle>,
    afterimages: Vec<Afterimage>,
    powerup: PowerUp,
    time_since_start: f32,
    screen_shake: f32,
}

impl Shooter {
    fn new() -> Self {
        let mut agents: Vec<Agent> = Vec::new();

        agents.push(Agent::new(0, 0.0, 0.0, true));

        let mut i: usize = 1;
        while i <= 6 {
            agents.push(Agent::new(i, i as f32 * 200.0, i as f32 * 100.0, false));
            i += 1;
        }

        Self {
            agents,
            initialized: false,
            bullets: Vec::new(),
            particles: Vec::new(),
            afterimages: Vec::new(),
            time_since_start: 0.0,
            screen_shake: 0.0,
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

    fn get_weapon_color(weapon: WeaponType) -> (u8, u8, u8) {
        match weapon {
            WeaponType::Pistol => (255, 255, 0),
            WeaponType::Shotgun => (255, 165, 0),
            WeaponType::Flamethrower => (255, 70, 0),
            WeaponType::Homing => (50, 255, 255),
        }
    }

    fn spawn_particles(particles: &mut Vec<Particle>, props: ParticleProps) {
        let mut p: usize = 0;

        while p < props.count {
            let pseudo_angle: f32 = (props.seed * 123.45 + p as f32 * 13.0).sin() * std::f32::consts::TAU;
            let pseudo_speed: f32 = props.speed * (0.5 + (props.seed * 50.0 + p as f32 * 7.0).cos().abs() * 0.5);
            let vel: Vec2 = Vec2::new(pseudo_angle.cos(), pseudo_angle.sin()) * pseudo_speed;

            particles.push(Particle {
                pos: props.pos,
                vel,
                life: props.life,
                max_life: props.life,
                r: props.color.0,
                g: props.color.1,
                b: props.color.2,
                size: props.size,
            });

            p += 1;
        }
    }

    fn spawn_bullets(
        bullets: &mut Vec<Bullet>,
        owner_id: usize,
        weapon: WeaponType,
        center: Vec2,
        dir_vec: Vec2,
        time_since_start: f32,
    ) -> f32 {
        match weapon {
            WeaponType::Pistol => {
                bullets.push(Bullet {
                    pos: center,
                    vel: dir_vec * BULLET_SPEED,
                    owner_id,
                    weapon: WeaponType::Pistol,
                    life: 3.0,
                    destroyed: false,
                    size: BULLET_SIZE,
                });

                100.0
            }
            WeaponType::Shotgun => {
                let angles: [f32; 5] = [-0.3, -0.15, 0.0, 0.15, 0.3];

                for &angle in &angles {
                    let cos_a: f32 = angle.cos();
                    let sin_a: f32 = angle.sin();

                    let rot_vel: Vec2 =
                        Vec2::new(dir_vec.x * cos_a - dir_vec.y * sin_a, dir_vec.x * sin_a + dir_vec.y * cos_a);

                    bullets.push(Bullet {
                        pos: center,
                        vel: rot_vel * BULLET_SPEED,
                        owner_id,
                        weapon: WeaponType::Shotgun,
                        life: 0.6,
                        destroyed: false,
                        size: BULLET_SIZE,
                    });
                }

                400.0
            }
            WeaponType::Flamethrower => {
                let offsets: [f32; 6] = [-0.4, -0.2, -0.05, 0.05, 0.2, 0.4];
                let speeds: [f32; 6] = [0.85, 1.0, 1.1, 0.95, 1.05, 0.9];
                let sizes: [f32; 6] = [12.0, 18.0, 24.0, 20.0, 16.0, 14.0];

                for i in 0..6 {
                    let wobble: f32 = (time_since_start * 25.0 + (i as f32)).sin() * 0.15;
                    let angle: f32 = offsets[i] + wobble;

                    let cos_a: f32 = angle.cos();
                    let sin_a: f32 = angle.sin();

                    let rot_vel: Vec2 =
                        Vec2::new(dir_vec.x * cos_a - dir_vec.y * sin_a, dir_vec.x * sin_a + dir_vec.y * cos_a);

                    bullets.push(Bullet {
                        pos: center - Vec2::splat(sizes[i] / 2.0),
                        vel: rot_vel * (BULLET_SPEED * speeds[i]),
                        owner_id,
                        weapon: WeaponType::Flamethrower,
                        life: 0.9,
                        destroyed: false,
                        size: sizes[i],
                    });
                }

                25.0
            }
            WeaponType::Homing => {
                bullets.push(Bullet {
                    pos: center,
                    vel: dir_vec * (BULLET_SPEED * 0.6),
                    owner_id,
                    weapon: WeaponType::Homing,
                    life: 4.0,
                    destroyed: false,
                    size: BULLET_SIZE,
                });

                150.0
            }
        }
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
        ctx.input_mut().bind(Action::Dash, KeyCode::Space.into());
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
        self.screen_shake = (self.screen_shake - dt * 25.0).max(0.0);

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

            Self::spawn_particles(
                &mut self.particles,
                ParticleProps {
                    pos: self.powerup.pos + Vec2::splat(POWERUP_SIZE / 2.0),
                    color: (255, 255, 255),
                    count: 20,
                    speed: 150.0,
                    seed: self.time_since_start,
                    life: 0.5,
                    size: 4.0,
                },
            );
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
            agent.dash_cooldown -= dt;

            if agent.rapid_fire_timer > 0.0 {
                agent.rapid_fire_timer -= dt;

                if agent.rapid_fire_timer <= 0.0 {
                    agent.weapon = WeaponType::Pistol;
                }
            }

            if agent.dash_cooldown > 0.8 && (self.time_since_start * 20.0).fract() < (dt * 20.0) {
                let (r, g, b): (u8, u8, u8) = if agent.rapid_fire_timer > 0.0 {
                    (255, 200, 50)
                } else if agent.is_player {
                    (50, 150, 255)
                } else {
                    (255, 50, 50)
                };

                self.afterimages.push(Afterimage {
                    pos: agent.pos,
                    life: 0.4,
                    max_life: 0.4,
                    r,
                    g,
                    b,
                });
            }

            if agent.is_player {
                let mut dir: Vec2 = Vec2::ZERO;

                if ctx.input().held(Action::Left) {
                    dir -= Vec2::X;
                }

                if ctx.input().held(Action::Right) {
                    dir += Vec2::X;
                }

                if ctx.input().held(Action::Up) {
                    dir -= Vec2::Y;
                }

                if ctx.input().held(Action::Down) {
                    dir += Vec2::Y;
                }

                if dir.length_sq() > 0.0 {
                    dir = dir.normalise();
                }

                let target_vel: Vec2 = dir * PLAYER_SPEED;
                agent.vel = agent.vel.lerp(target_vel, dt * 15.0);

                if ctx.input().held(Action::Dash) && agent.dash_cooldown <= 0.0 && dir.length_sq() > 0.0 {
                    agent.vel += dir * 1500.0;
                    agent.dash_cooldown = 1.2;
                    self.screen_shake = self.screen_shake.max(5.0);

                    let particle_color: (u8, u8, u8) = if agent.rapid_fire_timer > 0.0 {
                        (255, 200, 50)
                    } else {
                        (50, 150, 255)
                    };

                    Self::spawn_particles(
                        &mut self.particles,
                        ParticleProps {
                            pos: agent.pos + Vec2::splat(ENTITY_SIZE / 2.0),
                            color: particle_color,
                            count: 15,
                            speed: 250.0,
                            seed: self.time_since_start,
                            life: 0.4,
                            size: 6.0,
                        },
                    );
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

                        let recoil: f32 = Self::spawn_bullets(
                            &mut self.bullets,
                            agent.id,
                            agent.weapon,
                            center,
                            dir_vec,
                            self.time_since_start,
                        );

                        agent.vel -= dir_vec * recoil;
                        self.screen_shake = self.screen_shake.max(recoil * 0.015);
                    }
                }
            } else {
                let center: Vec2 = agent.pos + Vec2::splat(ENTITY_SIZE / 2.0);
                let target_center: Vec2 = player_pos + Vec2::splat(ENTITY_SIZE / 2.0);
                let dir_raw: Vec2 = target_center - center;
                let mag: f32 = dir_raw.length();

                let mut dir_vec: Vec2 = Vec2::ZERO;

                if mag > 0.0 {
                    let dir_norm: Vec2 = dir_raw / mag;
                    let min_dist: f32 = match agent.weapon {
                        WeaponType::Flamethrower => 60.0,
                        WeaponType::Shotgun => 120.0,
                        WeaponType::Homing => 250.0,
                        WeaponType::Pistol => 180.0,
                    };

                    if mag > min_dist {
                        dir_vec = dir_norm;
                    } else if mag < min_dist - 30.0 {
                        dir_vec = -dir_norm;
                    } else {
                        dir_vec = Vec2::new(-dir_norm.y, dir_norm.x);
                    }

                    if agent.shoot_cooldown <= 0.0 {
                        agent.shoot_cooldown = agent.current_cooldown_max();

                        let recoil: f32 = Self::spawn_bullets(
                            &mut self.bullets,
                            agent.id,
                            agent.weapon,
                            center,
                            dir_norm,
                            self.time_since_start,
                        );

                        agent.vel -= dir_norm * recoil;
                    }
                }

                let target_vel: Vec2 = dir_vec * BOT_SPEED;
                agent.vel = agent.vel.lerp(target_vel, dt * 8.0);
            }

            agent.pos += agent.vel * dt;

            if agent.pos.x < 0.0 {
                agent.pos.x = 0.0;
                agent.vel.x *= -0.5;
            }

            if agent.pos.x > w - ENTITY_SIZE {
                agent.pos.x = w - ENTITY_SIZE;
                agent.vel.x *= -0.5;
            }

            if agent.pos.y < 0.0 {
                agent.pos.y = 0.0;
                agent.vel.y *= -0.5;
            }

            if agent.pos.y > h - ENTITY_SIZE {
                agent.pos.y = h - ENTITY_SIZE;
                agent.vel.y *= -0.5;
            }

            if self.powerup.active && Self::check_collision(agent.pos, ENTITY_SIZE, self.powerup.pos, POWERUP_SIZE) {
                agent.weapon = self.powerup.weapon;
                agent.rapid_fire_timer = POWERUP_DURATION;
                self.powerup.active = false;

                let (r, g, b): (u8, u8, u8) = Self::get_weapon_color(agent.weapon);
                Self::spawn_particles(
                    &mut self.particles,
                    ParticleProps {
                        pos: agent.pos + Vec2::splat(ENTITY_SIZE / 2.0),
                        color: (r, g, b),
                        count: 30,
                        speed: 350.0,
                        seed: self.time_since_start,
                        life: 0.6,
                        size: 8.0,
                    },
                );
            }
        }

        let mut pushes: Vec<Vec2> = vec![Vec2::ZERO; agents_len];
        for i in 0..agents_len {
            for j in (i + 1)..agents_len {
                let p_i: Vec2 = self.agents[i].pos;
                let p_j: Vec2 = self.agents[j].pos;
                let dir_raw: Vec2 = p_i - p_j;
                let dist_sq: f32 = dir_raw.length_sq();
                let min_dist: f32 = ENTITY_SIZE;

                if dist_sq < min_dist * min_dist && dist_sq > 0.001 {
                    let dist: f32 = dist_sq.sqrt();
                    let overlap: f32 = min_dist - dist;
                    let push_dir: Vec2 = dir_raw / dist;
                    let push_vec: Vec2 = push_dir * (overlap * 0.5);

                    pushes[i] += push_vec;
                    pushes[j] -= push_vec;
                }
            }
        }

        for (agent, &push) in self.agents.iter_mut().zip(pushes.iter()) {
            agent.pos += push;
            agent.pos.x = agent.pos.x.clamp(0.0, w - ENTITY_SIZE);
            agent.pos.y = agent.pos.y.clamp(0.0, h - ENTITY_SIZE);
        }

        let mut a_idx: usize = 0;
        while a_idx < self.afterimages.len() {
            self.afterimages[a_idx].life -= dt;

            if self.afterimages[a_idx].life <= 0.0 {
                self.afterimages.remove(a_idx);
            } else {
                a_idx += 1;
            }
        }

        let mut p_idx: usize = 0;
        while p_idx < self.particles.len() {
            let p_vel: Vec2 = self.particles[p_idx].vel;

            self.particles[p_idx].vel *= 1.0 - (dt * 3.0);
            self.particles[p_idx].pos += p_vel * dt;
            self.particles[p_idx].life -= dt;
            self.particles[p_idx].size *= 1.0 - (dt * 2.0);

            if self.particles[p_idx].life <= 0.0 {
                self.particles.remove(p_idx);
            } else {
                p_idx += 1;
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
                    b.vel = (b.vel + target_dir * 25.0).normalise() * (BULLET_SPEED * 0.6);
                }
            }

            b.pos += b.vel * dt;
            b.life -= dt;

            let (r, g, b_c): (u8, u8, u8) = Self::get_weapon_color(b.weapon);

            if self.time_since_start % 0.05 < dt {
                Self::spawn_particles(
                    &mut self.particles,
                    ParticleProps {
                        pos: b.pos + Vec2::splat(b.size / 2.0),
                        color: (r, g, b_c),
                        count: 1,
                        speed: 20.0,
                        seed: self.time_since_start + b.pos.x,
                        life: 0.2,
                        size: b.size * 0.8,
                    },
                );
            }

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
                if agent.id != owner_i && Self::check_collision(pos_i, size_i, agent.pos, ENTITY_SIZE) {
                    agent.health -= 25;
                    agent.vel += self.bullets[i].vel.normalise() * 300.0;

                    Self::spawn_particles(
                        &mut self.particles,
                        ParticleProps {
                            pos: agent.pos + Vec2::splat(ENTITY_SIZE / 2.0),
                            color: (255, 50, 50),
                            count: 12,
                            speed: 250.0,
                            seed: self.time_since_start,
                            life: 0.4,
                            size: 6.0,
                        },
                    );

                    if agent.health <= 0 {
                        Self::spawn_particles(
                            &mut self.particles,
                            ParticleProps {
                                pos: agent.pos + Vec2::splat(ENTITY_SIZE / 2.0),
                                color: (255, 30, 30),
                                count: 40,
                                speed: 400.0,
                                seed: self.time_since_start,
                                life: 0.8,
                                size: 10.0,
                            },
                        );

                        agent.respawn(w, h);
                    }

                    if agent.is_player {
                        self.screen_shake = self.screen_shake.max(12.0);
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
                    let size_j: f32 = self.bullets[j].size;

                    if Self::check_collision(pos_i, size_i, pos_j, size_j) {
                        self.bullets[i].destroyed = true;
                        self.bullets[j].destroyed = true;

                        Self::spawn_particles(
                            &mut self.particles,
                            ParticleProps {
                                pos: pos_i + Vec2::splat(size_i / 2.0),
                                color: (255, 255, 100),
                                count: 6,
                                speed: 150.0,
                                seed: self.time_since_start,
                                life: 0.3,
                                size: 4.0,
                            },
                        );

                        break;
                    }
                }
            }
        }

        self.bullets.retain(|b: &Bullet| !b.destroyed);
    }

    fn on_render(&mut self, ctx: &mut dyn GameContext<Self::Action>) {
        let w: f32 = ctx.surface_width() as f32;
        let h: f32 = ctx.surface_height() as f32;

        let shake_offset: Vec2 = if self.screen_shake > 0.0 {
            Vec2::new(
                (self.time_since_start * 60.0).sin() * self.screen_shake,
                (self.time_since_start * 73.0).cos() * self.screen_shake,
            )
        } else {
            Vec2::ZERO
        };

        ctx.clear_color(Color::rgb(0.08, 0.08, 0.11));

        let mut player_pos: Vec2 = Vec2::ZERO;
        for agent in self.agents.iter() {
            if agent.is_player {
                player_pos = agent.pos;
            }
        }

        let grid_color: Color = Color::rgb(0.13, 0.13, 0.16);
        let offset_x: f32 = (player_pos.x * -0.05) % 60.0;
        let offset_y: f32 = (player_pos.y * -0.05) % 60.0;

        let mut x: f32 = offset_x;
        if x > 0.0 {
            x -= 60.0;
        }

        while x < w {
            ctx.draw_rect(Vec2::new(x, 0.0) + shake_offset, Vec2::new(1.0, h), grid_color);
            x += 60.0;
        }

        let mut y: f32 = offset_y;
        if y > 0.0 {
            y -= 60.0;
        }

        while y < h {
            ctx.draw_rect(Vec2::new(0.0, y) + shake_offset, Vec2::new(w, 1.0), grid_color);
            y += 60.0;
        }

        let a_len: usize = self.afterimages.len();
        for i in 0..a_len {
            let img: &Afterimage = &self.afterimages[i];
            let alpha: f32 = (img.life / img.max_life).clamp(0.0, 1.0);
            let alpha_curve: f32 = alpha * alpha;
            let alpha_u8: u8 = (alpha_curve * 100.0) as u8;
            let c: Color = Color::from_rgba8(img.r, img.g, img.b, alpha_u8);
            ctx.draw_rect(img.pos + shake_offset, Vec2::splat(ENTITY_SIZE), c);
        }

        let p_len: usize = self.particles.len();
        for i in 0..p_len {
            let p: &Particle = &self.particles[i];
            let alpha: f32 = (p.life / p.max_life).clamp(0.0, 1.0);
            let c: Color = Color::from_rgba8(p.r, p.g, p.b, (alpha * 255.0) as u8);
            ctx.draw_rect(p.pos + shake_offset, Vec2::splat(p.size), c);
        }

        if self.powerup.active {
            let pulse: f32 = (self.time_since_start * 5.0).sin().abs();
            let p_size: f32 = POWERUP_SIZE + (pulse * 6.0);
            let p_offset: Vec2 = Vec2::splat((p_size - POWERUP_SIZE) / 2.0);
            let p_draw_pos: Vec2 = self.powerup.pos - p_offset;

            let (r, g, b): (u8, u8, u8) = Self::get_weapon_color(self.powerup.weapon);
            let c: Color = Color::from_rgba8(r, g, b, (150.0 + pulse * 100.0) as u8);

            ctx.draw_rect(
                p_draw_pos + Vec2::splat(4.0) + shake_offset,
                Vec2::splat(p_size),
                Color::from_rgba8(0, 0, 0, 150),
            );

            ctx.draw_rect(p_draw_pos + shake_offset, Vec2::splat(p_size), c);
        }

        let agents_len: usize = self.agents.len();
        for i in 0..agents_len {
            let agent: &Agent = &self.agents[i];

            let color: Color = if agent.rapid_fire_timer > 0.0 {
                Color::from_rgba8(255, 200, 50, 255)
            } else {
                agent.color
            };

            ctx.draw_rect(
                agent.pos + Vec2::splat(4.0) + shake_offset,
                Vec2::splat(ENTITY_SIZE),
                Color::from_rgba8(0, 0, 0, 150),
            );

            ctx.draw_rect(agent.pos + shake_offset, Vec2::splat(ENTITY_SIZE), color);

            let inner_size: f32 = ENTITY_SIZE * 0.4;
            let inner_offset: Vec2 = Vec2::splat((ENTITY_SIZE - inner_size) / 2.0);
            let pulse_core: f32 = (self.time_since_start * 3.0 + agent.id as f32).sin().abs() * 0.5 + 0.5;

            ctx.draw_rect(
                agent.pos + inner_offset + shake_offset,
                Vec2::splat(inner_size),
                Color::from_rgba8(255, 255, 255, (pulse_core * 200.0) as u8),
            );

            let hp_percent: f32 = (agent.health.max(0) as f32) / 100.0;
            let hp_pos: Vec2 = agent.pos - (Vec2::Y * 10.0);

            ctx.draw_rect(hp_pos + shake_offset, Vec2::new(ENTITY_SIZE, 4.0), Color::rgb(0.7, 0.1, 0.1));
            ctx.draw_rect(
                hp_pos + shake_offset,
                Vec2::new(ENTITY_SIZE * hp_percent, 4.0),
                Color::rgb(0.1, 0.8, 0.2),
            );
        }

        let bullets_len: usize = self.bullets.len();
        for i in 0..bullets_len {
            let b: &Bullet = &self.bullets[i];
            let (r, g, b_c): (u8, u8, u8) = Self::get_weapon_color(b.weapon);

            let b_color: Color = Color::from_rgba8(r, g, b_c, 255);
            let glow_color: Color = Color::from_rgba8(r, g, b_c, 50);

            ctx.draw_rect(
                b.pos + Vec2::splat(3.0) + shake_offset,
                Vec2::splat(b.size),
                Color::from_rgba8(0, 0, 0, 100),
            );

            ctx.draw_rect(b.pos - Vec2::splat(4.0) + shake_offset, Vec2::splat(b.size + 8.0), glow_color);

            ctx.draw_rect(b.pos + shake_offset, Vec2::splat(b.size), b_color);
        }

        if let Some(mouse_pos) = ctx.input().mouse_position() {
            let crosshair_pos: Vec2 = mouse_pos + shake_offset * 0.5;
            ctx.draw_rect(crosshair_pos - Vec2::new(2.0, 12.0), Vec2::new(4.0, 24.0), Color::WHITE);
            ctx.draw_rect(crosshair_pos - Vec2::new(12.0, 2.0), Vec2::new(24.0, 4.0), Color::WHITE);
        }
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() -> Result<(), RedixelError> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize WASM logger");
    redixel::run(Shooter::new()).map_err(|e: RedixelError| RedixelError::JsException(format!("Engine error: {e}")))?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    if let Err(e) = redixel::run(Shooter::new()) {
        eprintln!("Engine error: {e}");
        std::process::exit(1);
    }
}
