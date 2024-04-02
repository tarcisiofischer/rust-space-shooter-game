mod controller;

extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::rect::Rect;
use crate::controller::{Controller, update_controller};

const SCALE_FACTOR : u32 = 2;

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

    let mut pos_x = 0;
    let mut pos_y = 0;
    'running: loop {
        update_controller(&mut event_pump, &mut controller);

        if controller.quit {
            break 'running;
        }

        if controller.right_pressed {
            pos_x += 5;
        }
        if controller.left_pressed {
            pos_x -= 5;
        }
        if controller.up_pressed {
            pos_y -= 5;
        }
        if controller.down_pressed {
            pos_y += 5;
        }

        canvas.clear();
        canvas.copy(&bg_texture, Rect::new(0, 0, 128, 256), Rect::new(0, 0, 128 * SCALE_FACTOR, 256 * SCALE_FACTOR)).unwrap();
        canvas.copy(&ships_texture, Rect::new(8, 0, 8, 8), Rect::new(pos_x, pos_y, 8 * SCALE_FACTOR, 8 * SCALE_FACTOR)).unwrap();
        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
