#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use piston_window::*;
use rand::random;
use std::collections::HashSet;

const PLAYER_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const ENEMY_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BULLET_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

const PLAYER_SPEED: f64 = 200.0; // Pixels/Second

const BULLET_SPEED: f64 = 300.0;
const ENEMY_SPEED: f64 = 50.0;

const PLAYER_SPEED_BOOSTED: f64 = 245.0; // in pixels per second
const BOOST_TIME: f64 = 30.0; // in seconds
const POWERUP_SPEED: f64 = 140.0; // in pixels per second
const POWERUP_SPAWN_TIME: f64 = 20.0; // in seconds
const POWERUP_COLOR: [f32; 4] = [0.5, 0.0, 0.5, 1.0]; // purple
const POWERUP_CHANCE: f32 = 0.5; // 50% chance

const MAX_ENEMIES_ON_BOARD: i32 = 10;

#[derive(Clone, PartialEq)]
struct Entity {
    x: f64,
    y: f64,
}

#[derive(Clone, PartialEq)]
enum PowerUpType {
    SpeedBoost,
    TripleShot,
}

struct PowerUp {
    x: f64,
    y: f64,
    power_up_type: PowerUpType,
}

pub struct Game {
    player: Entity,
    bullets: Vec<Entity>,
    enemies: Vec<Entity>,
    enemy_spawn_timer: f64,
    window_width: f64,
    window_height: f64,
    key_state: Option<Key>,
    power_ups: Vec<PowerUp>,
    power_up_spawn_timer: f64,
    power_up_active: Option<PowerUpType>,
    power_up_active_timer: f64,
}

impl Game {
    pub fn new(window_width: f64, window_height: f64) -> Game {
        Game {
            player: Entity {
                x: window_width / 2.0,
                y: window_height - 20.0,
            },
            bullets: Vec::new(),
            enemies: Vec::new(),
            enemy_spawn_timer: 0.0,
            window_width,
            window_height,
            key_state: None,
            power_ups: Vec::new(),
            power_up_spawn_timer: 0.0,
            power_up_active: None,
            power_up_active_timer: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        self.key_state = Some(key);
        if let Some(Key::Space) = self.key_state {
            self.bullets.push(Entity {
                x: self.player.x,
                y: self.players.y - 20,
            })
        }
    }

    pub fn key_released(&mut self, _key: Key) {
        self.key_state = None;
    }
}

pub fn update(&mut self, dt: f64) {
    match self.key_state {
        Some(Key::Left) => self.player.x -= PLAYER_SPEED * dt,
        Some(Key::Right) => self.player.x += PLAYER_SPEED * dt,
    }

    self.enemy_spawn_timer += dt;
    if slef.enemy_spawn_timer > 1.0 {
        self.enemies.push(Entity {
            x: random::<f64>() * self.window_width,
            y: 0.0,
        });
        self.enemy_spawn_timer = 0;
    }

    // update bullets
    for bullet in &mut self.bullets {
        bullet.y -= BULLET_SPEED * dt;
    }

    // update enemies
    for enemy in &mut self.enemies {
        enemy.y += ENEMY_SPEED * dt;
    }

    // spawn power_ups
    self.power_up_spawn_timer += dt;
    if self.power_up_spawn_timer > POWERUP_SPAWN_TIME {
        let power_up_type = if random::<f32>() < POWERUP_CHANCE {
            PowerUpType::TripleShot
        } else {
            PowerUpType::SpeedBoost
        };
        self.power_ups.push(PowerUp {
            x: random::<f64>() * self.window_width,
            y: 0.0,
            power_up_type,
        });
        self.power_up_spawn_timer = 0.0;
    }

    // Handle active power-up effects
    if let Some(power_up_type) = &self.power_up_active {
        match power_up_type {
            PowerUpType::SpeedBoost => {
                // The player's speed will be boosted when moving
                match self.key_state {
                    Some(Key::Left) => self.player.x -= PLAYER_SPEED_BOOSTED * dt,
                    Some(Key::Right) => self.player.x += PLAYER_SPEED_BOOSTED * dt,
                    _ => (),
                }
            }
            PowerUpType::TripleShot => {
                // The player will shoot three bullets when shooting
                if let Some(Key::Space) = self.key_state {
                    self.bullets.push(Entity {
                        x: self.player.x - 10.0,
                        y: self.player.y - 20.0,
                    });
                    self.bullets.push(Entity {
                        x: self.player.x,
                        y: self.player.y - 20.0,
                    });
                    self.bullets.push(Entity {
                        x: self.player.x + 10.0,
                        y: self.player.y - 20.0,
                    });
                    self.key_state = None;
                }
            }
        }
        // Decrease the active power-up timer
        self.power_up_active_timer -= dt;
        if self.power_up_active_timer <= 0.0 {
            self.power_up_active = None;
        }
    }
}
