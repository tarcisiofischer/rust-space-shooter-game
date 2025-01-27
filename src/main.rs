mod controller;

extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::rect::{Point, Rect};
use crate::controller::{Controller, update_controller};
use rand::Rng;
use sdl2::render::{Texture, WindowCanvas};

const SCALE_FACTOR : u32 = 2;
const MAX_ENEMIES : usize = 12;

fn check_aabb(a: Rect, b: Rect) -> bool {
    return
        a.x       < b.x + b.w &&
        a.x + a.w > b.x       &&
        a.y       < b.y + b.h &&
        a.y + a.h > b.y
    ;
}

struct Player<'rawshipstexture, 'rawmisctexture> {
    pub pos : Point,
    ships_texture : &'rawshipstexture Texture<'rawshipstexture>,
    misc_texture : &'rawmisctexture Texture<'rawmisctexture>,
    state : PlayerState,
}

enum PlayerState {
    Idle,
    MovingLeft,
    MovingRight,
    Dying { frame : i32, nticks : i32 },
    None,
}

impl<'rawshipstexture, 'rawmisctexture> Player<'rawshipstexture, 'rawmisctexture> {

    fn paint(&self, canvas : &mut WindowCanvas) {
        match self.state {
            PlayerState::Idle => {
                canvas.copy(
                    &self.ships_texture,
                    Rect::new(8 * 1, 0, 8, 8),
                    Rect::new(self.pos.x, self.pos.y, 8 * SCALE_FACTOR, 8 * SCALE_FACTOR)
                ).unwrap();
            }
            PlayerState::MovingLeft => {
                canvas.copy(
                    &self.ships_texture,
                    Rect::new(8 * 0, 0, 8, 8),
                    Rect::new(self.pos.x, self.pos.y, 8 * SCALE_FACTOR, 8 * SCALE_FACTOR)
                ).unwrap();
            }
            PlayerState::MovingRight => {
                canvas.copy(
                    &self.ships_texture,
                    Rect::new(8 * 2, 0, 8, 8),
                    Rect::new(self.pos.x, self.pos.y, 8 * SCALE_FACTOR, 8 * SCALE_FACTOR)
                ).unwrap();
            }
            PlayerState::Dying{ frame, .. } => {
                canvas.copy(
                    &self.misc_texture,
                    Rect::new(8 * (9 + frame), 8 * 6, 8, 8),
                    Rect::new(self.pos.x, self.pos.y, 8 * SCALE_FACTOR, 8 * SCALE_FACTOR)
                ).unwrap();
            }
            PlayerState::None => {}
        }
    }

    fn update(&mut self) {
        match self.state {
            PlayerState::Dying { frame, nticks } => {
                if nticks < 5 {
                    self.state = PlayerState::Dying { frame, nticks: nticks + 1 };
                } else {
                    if frame + 1 == 4 {
                        self.state = PlayerState::None;
                    } else {
                        self.state = PlayerState::Dying { frame: frame + 1, nticks: 0 };
                    }
                }
            }
            _ => { }
        }
    }

    fn set_state(&mut self, new_state: PlayerState) {
        self.state = new_state;
    }

    fn is_dead(&self) -> bool {
        match self.state {
            PlayerState::Dying { .. } => { true }
            PlayerState::None => { true }
            _ => { false }
        }
    }

    fn is_none(&self) -> bool {
        match self.state {
            PlayerState::None => { true }
            _ => { false }
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Yet another space shooter game", 128 * SCALE_FACTOR, 256 * SCALE_FACTOR)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(33, 33, 33));
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut controller = Controller::new();

    let texture_creator = canvas.texture_creator();
    let bg_texture = texture_creator.load_texture("assets/backgrounds.png").unwrap();
    let ships_texture = texture_creator.load_texture("assets/ships.png").unwrap();
    let misc_texture = texture_creator.load_texture("assets/misc.png").unwrap();
    let projectiles_texture = texture_creator.load_texture("assets/projectiles.png").unwrap();

    let mut player = Player{
        pos: Point::new(64 * SCALE_FACTOR as i32, 128 * SCALE_FACTOR as i32),
        ships_texture: &ships_texture,
        misc_texture: &misc_texture,
        state: PlayerState::Idle,
    };
    let mut projectiles = vec![];
    let mut enemies = vec![];
    let mut enemy_spawn_cooldown = 10;
    let mut rng = rand::thread_rng();
    'running: loop {
        update_controller(&mut event_pump, &mut controller);

        if controller.quit {
            break 'running;
        }

        if !player.is_dead() {
            if controller.right_pressed {
                player.pos.x += 5;
                player.set_state(PlayerState::MovingRight);
            } else if controller.left_pressed {
                player.pos.x -= 5;
                player.set_state(PlayerState::MovingLeft);
            } else {
                player.set_state(PlayerState::Idle);
            }
            if controller.up_pressed {
                player.pos.y -= 5;
            }
            if controller.down_pressed {
                player.pos.y += 5;
            }
            if controller.fire_pressed && controller.fire_just_pressed {
                projectiles.push(player.pos);
            }
        }
        projectiles.retain_mut(|projectile| {
            projectile.y -= 12;
            return projectile.y >= 0;
        });
        if enemy_spawn_cooldown == 0 && enemies.len() < MAX_ENEMIES {
            if rng.gen_range(0..2) == 0 {
                enemies.push(Point::new(rng.gen_range(-20..150 * SCALE_FACTOR as i32), 0))
            }
            enemy_spawn_cooldown = 10;
        }
        if enemy_spawn_cooldown > 0 {
            enemy_spawn_cooldown -= 1;
        }

        for enemy in &mut enemies {
            if enemy.y < 256 * SCALE_FACTOR as i32 {
                enemy.y += 4;
                enemy.x += ((enemy.y as f32 / 24.0).sin() * 8.0) as i32;
            } else {
                enemy.y = -8;
                enemy.x = 54 * SCALE_FACTOR as i32;
            }
        }

        for projectile in &projectiles {
            enemies.retain(|enemy| {
                return !check_aabb(Rect::new(projectile.x, projectile.y, 8 * SCALE_FACTOR, 8 * SCALE_FACTOR), Rect::new(enemy.x, enemy.y, 8 * SCALE_FACTOR, 8 * SCALE_FACTOR));
            });
        }
        if !player.is_dead() {
            for enemy in &enemies {
                if check_aabb(Rect::new(enemy.x, enemy.y, 8 * SCALE_FACTOR, 8 * SCALE_FACTOR), Rect::new(player.pos.x, player.pos.y, 8 * SCALE_FACTOR, 8 * SCALE_FACTOR)) {
                    player.set_state(PlayerState::Dying { frame: 0, nticks: 0 });
                }
            }
        }

        if player.is_none() {
            // Reset player
            player.pos = Point::new(64 * SCALE_FACTOR as i32, 128 * SCALE_FACTOR as i32);
            player.set_state(PlayerState::Idle);
        }

        player.update();

        canvas.clear();
        canvas.copy(&bg_texture, Rect::new(0, 0, 128, 256), Rect::new(0, 0, 128 * SCALE_FACTOR, 256 * SCALE_FACTOR)).unwrap();
        player.paint(&mut canvas);

        for enemy in &enemies {
            canvas.copy(&ships_texture, Rect::new(40, 0, 8, 8), Rect::new(enemy.x, enemy.y, 8 * SCALE_FACTOR, 8 * SCALE_FACTOR)).unwrap();
        }
        for projectile in &projectiles {
            canvas.copy(&projectiles_texture, Rect::new(16, 0, 8, 8), Rect::new(projectile.x, projectile.y, 8 * SCALE_FACTOR, 8 * SCALE_FACTOR)).unwrap();
        }
        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
