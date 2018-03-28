// --------------------------------------------------------------------------
//
// Rusty Kong
// Copyright (C) 2018 Jeff Panici
// All rights reserved.
//
// This software source file is licensed according to the
// MIT License.  Refer to the LICENSE file distributed along
// with this source file to learn more.
//
// --------------------------------------------------------------------------

mod video;
mod sound;
mod level;
mod player;
mod state_machine;

use sdl2;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::controller::GameController;

use self::video::VideoGenerator;
use self::state_machine::GameState;

struct SystemInterfaces<'a> {
    controller: GameController,
    video: VideoGenerator<'a>,
    game_state: GameState
}

fn controller_init(sdl_context: &Sdl) -> GameController {
    let subsystem = sdl_context.game_controller().unwrap();

    let available =
        match subsystem.num_joysticks() {
            Ok(n) => n,
            Err(e) => {
                error!("can't enumerate joysticks: {}", e);
                panic!();
            },
        };

    info!("{} joysticks available", available);

    let mut controller = None;

    for id in 0..available {
        if subsystem.is_game_controller(id) {
            info!("Attempting to open controller {}", id);

            match subsystem.open(id) {
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
            None => {
                error!("Couldn't open any controller");
                panic!();
            },
        };

    info!("Controller mapping: {}", controller.mapping());

    return controller;
}

pub fn game_run() {
    use rusty_kong::video::video_update;

    let context = sdl2::init().unwrap();
    let mut system_interfaces = game_init(&context).unwrap();

    let mut event_pump = context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        system_interfaces.game_state.update(&system_interfaces.controller);
        video_update(&mut system_interfaces.video);
    }
}

fn game_init(context:&Sdl) -> Result<SystemInterfaces, String> {
    use self::state_machine::game_state_init;
    use rusty_kong::video::video_init;

    return Ok(SystemInterfaces {
        game_state: game_state_init(),
        video: video_init(&context),
        controller: controller_init(&context),
    });
}
