mod controller;

extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::rect::{Point, Rect};
use crate::controller::{Controller, update_controller};
use rand::Rng;

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
    let projectiles_texture = texture_creator.load_texture("assets/projectiles.png").unwrap();

    let mut pos = Point::new(64 * SCALE_FACTOR as i32, 128 * SCALE_FACTOR as i32);
    let mut projectiles = vec![];
    let mut enemies = vec![];
    let mut enemy_spawn_cooldown = 10;
    let mut rng = rand::thread_rng();
    'running: loop {
        update_controller(&mut event_pump, &mut controller);

        if controller.quit {
            break 'running;
        }

        if controller.right_pressed {
            pos.x += 5;
        }
        if controller.left_pressed {
            pos.x -= 5;
        }
        if controller.up_pressed {
            pos.y -= 5;
        }
        if controller.down_pressed {
            pos.y += 5;
        }
        if controller.fire_pressed && controller.just_changed {
            projectiles.push(pos);
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
                return !check_aabb(Rect::new(projectile.x, projectile.y, 8, 8), Rect::new(enemy.x, enemy.y, 8, 8));
            })
        }

        canvas.clear();
        canvas.copy(&bg_texture, Rect::new(0, 0, 128, 256), Rect::new(0, 0, 128 * SCALE_FACTOR, 256 * SCALE_FACTOR)).unwrap();
        canvas.copy(&ships_texture, Rect::new(8, 0, 8, 8), Rect::new(pos.x, pos.y, 8 * SCALE_FACTOR, 8 * SCALE_FACTOR)).unwrap();
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
