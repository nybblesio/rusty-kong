extern crate sdl2;
#[macro_use]
extern crate log;
extern crate simple_logger;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::controller::GameController;

const SCREEN_WIDTH:u32 = 224;
const SCREEN_HEIGHT:u32 = 256;

fn video_init(sdl_context:&Sdl) -> WindowCanvas {
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(
            "Rusty Kong",
            SCREEN_WIDTH * 3,
            SCREEN_HEIGHT * 3)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    return window.into_canvas().build().unwrap();
}

fn game_controller_init(sdl_context:&Sdl) -> GameController {
    let game_controller_subsystem = sdl_context.game_controller().unwrap();

    let available =
        match game_controller_subsystem.num_joysticks() {
            Ok(n)  => n,
            Err(e) =>  {
                error!("can't enumerate joysticks: {}", e);
                panic!();
            },
        };

    info!("{} joysticks available", available);

    let mut controller = None;

    for id in 0..available {
        if game_controller_subsystem.is_game_controller(id) {
            info!("Attempting to open controller {}", id);

            match game_controller_subsystem.open(id) {
                Ok(c) => {
                    info!("Success: opened \"{}\"", c.name());
                    controller = Some(c);
                    break;
                },
                Err(e) => {
                    error!("failed: {:?}", e);
                    panic!();
                },
            }

        } else {
            warn!("{} is not a game controller", id);
        }
    }

    let controller =
        match controller {
            Some(c) => c,
            None     => {
                error!("Couldn't open any controller");
                panic!();
            },
        };

    info!("Controller mapping: {}", controller.mapping());

    return controller;
}

fn main() {
    simple_logger::init().unwrap();

    let sdl_context = sdl2::init().unwrap();
    let game_controller = game_controller_init(&sdl_context);
    let mut canvas = video_init(&sdl_context);

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));


    }
}
