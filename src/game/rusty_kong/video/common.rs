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

pub const SCREEN_WIDTH:  u32 = 224;
pub const SCREEN_HEIGHT: u32 = 256;

pub const TILE_WIDTH:    u32 = 8;
pub const TILE_HEIGHT:   u32 = 8;
pub const TILE_MAX:      u32 = 256;
pub const TILE_COL_COUNT:u32 = SCREEN_WIDTH / TILE_WIDTH;
pub const TILE_ROW_COUNT:u32 = SCREEN_HEIGHT / TILE_HEIGHT;

pub const SPRITE_WIDTH:  u32 = 16;
pub const SPRITE_HEIGHT: u32 = 16;
pub const SPRITE_MAX:    u32 = 128;

pub const F_SPR_NONE:    u8 = 0b00000000;
pub const F_SPR_ENABLED: u8 = 0b00000001;
pub const F_SPR_COLLIDED:u8 = 0b00000010;
pub const F_SPR_HFLIP:   u8 = 0b00000100;
pub const F_SPR_VFLIP:   u8 = 0b00001000;

pub const F_BG_NONE:     u8 = 0b00000000;
pub const F_BG_ENABLED:  u8 = 0b00000001;
pub const F_BG_HFLIP:    u8 = 0b00000010;
pub const F_BG_VFLIP:    u8 = 0b00000100;

#[derive(Copy, Clone)]
pub struct Palette {
    pub entries: [PaletteEntry; 4]
}

#[derive(Copy, Clone)]
pub struct PaletteEntry {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

pub struct SpriteControlBlock {
    y: u16,
    x: u16,
    tile: u16,
    palette: u8,
    flags: u8,
    user_data1: u32,
    user_data2: u32
}

pub struct BackgroundControlBlock {
    tile: u16,
    flags: u8,
    palette: u8,
    user_data1: u32,
    user_data2: u32
}
