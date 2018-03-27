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
pub const F_SPR_CHANGED: u8 = 0b00010000;

pub const F_BG_NONE:     u8 = 0b00000000;
pub const F_BG_ENABLED:  u8 = 0b00000001;
pub const F_BG_HFLIP:    u8 = 0b00000010;
pub const F_BG_VFLIP:    u8 = 0b00000100;
pub const F_BG_CHANGED:  u8 = 0b00001000;

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

use sdl2::surface::SurfaceRef;

#[derive(Copy, Clone)]
pub struct SpriteControlBlock {
    y: u16,
    x: u16,
    flags: u8,
    tile: u16,
    palette: u8,
    user_data1: u32,
    user_data2: u32,
    //surface: SurfaceRef,
}

impl SpriteControlBlock {
    pub fn new_control_table() -> [SpriteControlBlock; SPRITE_MAX as usize] {
        [SpriteControlBlock::new_empty(); SPRITE_MAX as usize]
    }

    pub fn new_empty() -> SpriteControlBlock {
        SpriteControlBlock {
            y: 0,
            x: 0,
            tile: 0,
            palette: 0,
            flags: F_SPR_NONE,
            user_data1: 0,
            user_data2: 0,
            //surface: ()
        }
    }

    pub fn update(self:&SpriteControlBlock) {
        use super::palettes::get_palette;
        use super::sprites::get_sprite_bitmap;

    }

    pub fn tile(self:&mut SpriteControlBlock, number:u16) {
        self.tile = number;
    }

    pub fn is_changed(self:&SpriteControlBlock) -> bool {
        self.flags & F_SPR_CHANGED != 0
    }

    pub fn is_enabled(self:&SpriteControlBlock) -> bool {
        self.flags & F_SPR_ENABLED != 0
    }

    pub fn is_collided(self:&SpriteControlBlock) -> bool {
        self.flags & F_SPR_COLLIDED != 0
    }

    pub fn enable(self:&mut SpriteControlBlock, flag:bool) {
        if flag {
            self.flags |= F_SPR_ENABLED;
        } else {
            self.flags &= !F_SPR_ENABLED;
        }
    }

    pub fn changed(self:&mut SpriteControlBlock, flag:bool) {
        if flag {
            self.flags |= F_SPR_CHANGED;
        } else {
            self.flags &= !F_SPR_CHANGED;
        }
    }

    pub fn collided(self:&mut SpriteControlBlock, flag:bool) {
        if flag {
            self.flags |= F_SPR_COLLIDED;
        } else {
            self.flags &= !F_SPR_COLLIDED;
        }
    }

    pub fn palette(self:&mut SpriteControlBlock, number:u8) {
        self.palette = number;
    }

    pub fn get_position(self:&SpriteControlBlock) -> (u16, u16) {
        (self.x, self.y)
    }

    pub fn position(self:&mut SpriteControlBlock, x:u16, y:u16) {
        self.x = x;
        self.y = y;
    }

    pub fn vertical_flip(self:&mut SpriteControlBlock, flag:bool) {
        if flag {
            self.flags |= F_SPR_VFLIP;
        } else {
            self.flags &= !F_SPR_VFLIP;
        }
    }

    pub fn is_vertically_flipped(self:&SpriteControlBlock) -> bool {
        self.flags & F_SPR_VFLIP != 0
    }

    pub fn horizontal_flip(self:&mut SpriteControlBlock, flag:bool) {
        if flag {
            self.flags |= F_SPR_HFLIP;
        } else {
            self.flags &= !F_SPR_HFLIP;
        }
    }

    pub fn is_horizontally_flipped(self:&SpriteControlBlock) -> bool {
        self.flags & F_SPR_HFLIP != 0
    }
}

#[derive(Copy, Clone)]
pub struct BackgroundControlBlock {
    tile: u16,
    flags: u8,
    palette: u8,
    user_data1: u32,
    user_data2: u32
}

impl BackgroundControlBlock {
    pub fn new_control_table() -> [BackgroundControlBlock; (TILE_ROW_COUNT * TILE_COL_COUNT) as usize] {
        [BackgroundControlBlock::new_empty(); (TILE_ROW_COUNT * TILE_COL_COUNT) as usize]
    }

    pub fn new_empty() -> BackgroundControlBlock {
        BackgroundControlBlock {
            tile: 0,
            flags: F_BG_NONE,
            palette: 0,
            user_data1: 0,
            user_data2: 0}
    }

    pub fn update(self:&BackgroundControlBlock) {
        use super::palettes::get_palette;
        use super::tiles::get_tile_bitmap;

    }

    pub fn is_changed(self:&BackgroundControlBlock) -> bool {
        self.flags & F_BG_CHANGED != 0
    }

    pub fn is_enabled(self:&BackgroundControlBlock) -> bool {
        self.flags & F_BG_ENABLED != 0
    }

    pub fn tile(self:&mut BackgroundControlBlock, number:u16) {
        self.tile = number;
    }

    pub fn changed(self:&mut BackgroundControlBlock, flag:bool) {
        if flag {
            self.flags |= F_BG_CHANGED;
        } else {
            self.flags &= !F_BG_CHANGED;
        }
    }

    pub fn palette(self:&mut BackgroundControlBlock, number:u8) {
        self.palette = number;
    }

    pub fn enable(self:&mut BackgroundControlBlock, flag:bool) {
        if flag {
            self.flags |= F_BG_ENABLED;
        } else {
            self.flags &= !F_BG_ENABLED;
        }
    }

    pub fn vertical_flip(self:&mut BackgroundControlBlock, flag:bool) {
        if flag {
            self.flags |= F_BG_VFLIP;
        } else {
            self.flags &= !F_BG_VFLIP;
        }
    }

    pub fn is_vertically_flipped(self:&BackgroundControlBlock) -> bool {
        self.flags & F_BG_VFLIP != 0
    }

    pub fn horizontal_flip(self:&mut BackgroundControlBlock, flag:bool) {
        if flag {
            self.flags |= F_BG_HFLIP;
        } else {
            self.flags &= !F_BG_HFLIP;
        }
    }

    pub fn is_horizontally_flipped(self:&BackgroundControlBlock) -> bool {
        self.flags & F_BG_HFLIP != 0
    }
}