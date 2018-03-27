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

use std::vec::Vec;

use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

const SCREEN_WIDTH:  u32 = 224;
const SCREEN_HEIGHT: u32 = 256;

const TILE_WIDTH:    u32 = 8;
const TILE_HEIGHT:   u32 = 8;
const TILE_COL_COUNT:u32 = SCREEN_WIDTH / TILE_WIDTH;
const TILE_ROW_COUNT:u32 = SCREEN_HEIGHT / TILE_HEIGHT;

const SPRITE_WIDTH:  u32 = 16;
const SPRITE_HEIGHT: u32 = 16;
const SPRITE_MAX:    u32 = 96;

const F_SPR_NONE:    u8 = 0b00000000;
const F_SPR_ENABLED: u8 = 0b00000001;
const F_SPR_COLLIDED:u8 = 0b00000010;
const F_SPR_HFLIP:   u8 = 0b00000100;
const F_SPR_VFLIP:   u8 = 0b00001000;

const F_BG_NONE:     u8 = 0b00000000;
const F_BG_ENABLED:  u8 = 0b00000001;
const F_BG_HFLIP:    u8 = 0b00000010;
const F_BG_VFLIP:    u8 = 0b00000100;

struct Palette {
    entries: Vec<PaletteEntry>
}

struct PaletteEntry {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

struct SpriteControlBlock {
    y: u16,
    x: u16,
    tile: u16,
    palette: u8,
    flags: u8,
    user_data1: u32,
    user_data2: u32
}

struct BackgroundControlBlock {
    tile: u16,
    flags: u8,
    palette: u8,
    user_data1: u32,
    user_data2: u32
}

lazy_static! {
    static ref SPR_CNTL:Vec<SpriteControlBlock> = vec!();
    static ref BG1_CNTL:Vec<BackgroundControlBlock> = vec!();
}

fn video_bg(canvas: &mut WindowCanvas) {

}

fn video_fg(canvas: &mut WindowCanvas) {

}

pub fn video_update(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    video_bg(canvas);
    video_fg(canvas);
    canvas.present();
}

pub fn video_init(sdl_context: &Sdl) -> WindowCanvas {
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
