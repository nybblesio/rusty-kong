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

mod common;
use self::common::*;

mod palettes;
use self::palettes::get_palette;

mod sprites;
use self::sprites::get_sprite_bitmap;

mod tiles;
use self::tiles::get_tile_bitmap;

use std::vec::Vec;

use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

lazy_static! {
    static ref SPR_CNTL:[SpriteControlBlock; SPRITE_MAX as usize] = SpriteControlBlock::new_control_table();
    static ref BG1_CNTL:[BackgroundControlBlock; (TILE_ROW_COUNT * TILE_COL_COUNT) as usize] = BackgroundControlBlock::new_control_table();
}

fn video_bg(canvas: &mut WindowCanvas) {
    // XXX: need Surface that is the background buffer
    //      only changed tiles get rendered into this buffer
    //      copy this into canvas
    for bg_cntl in BG1_CNTL.iter() {
        bg_cntl.update();
    }
}

// XXX: structure passed in here should hold WindowCanvas, background buffer, and any other state
fn video_fg(canvas: &mut WindowCanvas) {
    for fg_cntl in SPR_CNTL.iter() {
        // XXX: update internal surface
        //      copy to canvas
        fg_cntl.update();
    }
}

// XXX: structure passed in here should hold WindowCanvas, background buffer, and any other state
pub fn video_update(canvas: &mut WindowCanvas) {
//    canvas.set_draw_color(Color::RGB(0, 0, 0));
//    canvas.clear();
    video_bg(canvas);
    //canvas.copy(background_surface, .....);

    video_fg(canvas);

    canvas.present();
}

pub fn video_init(sdl_context: &Sdl) -> WindowCanvas {
    // XXX: set up the background buffer and package it with WindowCanvas
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Rusty Kong", SCREEN_WIDTH * 4, SCREEN_HEIGHT * 4)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    return window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();
}
