use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::EventPump;

pub struct Controller {
    pub quit : bool,
    pub left_pressed : bool,
    pub right_pressed : bool,
    pub up_pressed : bool,
    pub down_pressed : bool,
    pub fire_pressed : bool,
    pub fire_just_pressed : bool,
    pub just_changed : bool,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            quit: false,
            left_pressed: false,
            right_pressed: false,
            up_pressed: false,
            down_pressed: false,
            fire_pressed: false,
            fire_just_pressed: false,
            just_changed: false,
        }
    }
}

pub fn update_controller(event_pump : &mut EventPump, controller : &mut Controller) {
    controller.just_changed = false;
    controller.fire_just_pressed = false;
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => controller.quit = true,

            Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                controller.left_pressed = true;
                controller.just_changed = true;
            },
            Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
                controller.left_pressed = false;
                controller.just_changed = true;
            },

            Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                controller.right_pressed = true;
                controller.just_changed = true;
            },
            Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
                controller.right_pressed = false;
                controller.just_changed = true;
            },

            Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                controller.up_pressed = true;
                controller.just_changed = true;
            },
            Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
                controller.up_pressed = false;
                controller.just_changed = true;
            },

            Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                controller.down_pressed = true;
                controller.just_changed = true;
            },
            Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                controller.down_pressed = false;
                controller.just_changed = true;
            },

            Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                controller.fire_pressed = true;
                controller.fire_just_pressed = true;
                controller.just_changed = true;
            },
            Event::KeyUp { keycode: Some(Keycode::Space), repeat: false, .. } => {
                controller.fire_pressed = false;
                controller.just_changed = true;
            },

            _ => { }
        }
    }
}