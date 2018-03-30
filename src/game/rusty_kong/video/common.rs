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

use std::cell::RefCell;
use std::option::Option;

use super::tiles::get_tile_bitmap;
use super::sprites::get_sprite_bitmap;
use super::palettes::get_palette_colors;

use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Palette as SdlPalette;

pub const SCREEN_WIDTH:  u32 = 256;
pub const SCREEN_HEIGHT: u32 = 256;

pub const TILE_WIDTH:    u32 = 8;
pub const TILE_HEIGHT:   u32 = 8;
pub const TILE_MAX:      u32 = 256;
pub const TILE_COL_COUNT:u32 = 32;
pub const TILE_ROW_COUNT:u32 = 32;
pub const TILE_CNTL_MAX: u32 = TILE_ROW_COUNT * TILE_COL_COUNT;

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

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum TileMaps {
    LongIntroduction,
    Level1,
}

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

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct TileMapEntry(pub u16, pub u8, pub u8);

#[derive(Copy, Clone)]
pub struct TileMap {
    pub entries: [TileMapEntry; (TILE_ROW_COUNT * TILE_COL_COUNT) as usize]
}

#[derive(Copy, Clone)]
pub struct SpriteControlBlock {
    y: u16,
    x: u16,
    flags: u8,
    tile: u16,
    palette: u8,
    user_data1: u32,
    user_data2: u32
}

pub struct SpriteControlTable {
    table: [SpriteControlBlock; SPRITE_MAX as usize],
    surfaces: Vec<RefCell<Surface<'static>>>
}

impl SpriteControlTable {
    pub fn new() -> SpriteControlTable {
        let mut table = SpriteControlTable {
            surfaces: vec![],
            table: [SpriteControlBlock::new_empty(); SPRITE_MAX as usize],
        };
        for _i in 0..SPRITE_MAX {
            let mut surface = Surface::new(
                16,
                16,
                PixelFormatEnum::Index8).unwrap();
            let palette = SdlPalette::with_colors(&get_palette_colors()).unwrap();
            surface.set_palette(&palette);
            table.surfaces.push(RefCell::new(surface));
        }
        table
    }

    pub fn update(&mut self) {
    }
}

impl<'a> SpriteControlBlock {
    pub fn new_empty() -> SpriteControlBlock {
        SpriteControlBlock {
            y: 0,
            x: 0,
            tile: 0,
            palette: 0,
            flags: F_SPR_NONE,
            user_data1: 0,
            user_data2: 0,
        }
    }

    pub fn update(&mut self) {
        use super::palettes::get_palette;
        use super::sprites::get_sprite_bitmap;

    }

    pub fn tile(&mut self, number:u16) {
        self.tile = number;
    }

    pub fn is_changed(&self) -> bool {
        self.flags & F_SPR_CHANGED != 0
    }

    pub fn is_enabled(&self) -> bool {
        self.flags & F_SPR_ENABLED != 0
    }

    pub fn is_collided(&self) -> bool {
        self.flags & F_SPR_COLLIDED != 0
    }

    pub fn enable(&mut self, flag:bool) {
        if flag {
            self.flags |= F_SPR_ENABLED;
        } else {
            self.flags &= !F_SPR_ENABLED;
        }
    }

    pub fn changed(&mut self, flag:bool) {
        if flag {
            self.flags |= F_SPR_CHANGED;
        } else {
            self.flags &= !F_SPR_CHANGED;
        }
    }

    pub fn collided(&mut self, flag:bool) {
        if flag {
            self.flags |= F_SPR_COLLIDED;
        } else {
            self.flags &= !F_SPR_COLLIDED;
        }
    }

    pub fn palette(&mut self, number:u8) {
        self.palette = number;
    }

    pub fn get_position(&self) -> (u16, u16) {
        (self.x, self.y)
    }

    pub fn position(&mut self, x:u16, y:u16) {
        self.x = x;
        self.y = y;
    }

    pub fn vertical_flip(&mut self, flag:bool) {
        if flag {
            self.flags |= F_SPR_VFLIP;
        } else {
            self.flags &= !F_SPR_VFLIP;
        }
    }

    pub fn is_vertically_flipped(&self) -> bool {
        self.flags & F_SPR_VFLIP != 0
    }

    pub fn horizontal_flip(&mut self, flag:bool) {
        if flag {
            self.flags |= F_SPR_HFLIP;
        } else {
            self.flags &= !F_SPR_HFLIP;
        }
    }

    pub fn is_horizontally_flipped(&self) -> bool {
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

pub struct BackgroundControlTable {
    table: [BackgroundControlBlock; TILE_CNTL_MAX as usize],
    surfaces: Vec<RefCell<Surface<'static>>>
}

impl BackgroundControlTable {
    pub fn new() -> BackgroundControlTable {
        let mut table = BackgroundControlTable {
            surfaces: vec![],
            table: [BackgroundControlBlock::new_empty(); TILE_CNTL_MAX as usize],
        };
        for _i in 0..TILE_CNTL_MAX {
            let mut surface = Surface::new(
                8,
                8,
                PixelFormatEnum::Index8).unwrap();
            let palette = SdlPalette::with_colors(&get_palette_colors()).unwrap();
            surface.set_palette(&palette);
            table.surfaces.push(RefCell::new(surface));
        }
        table
    }

    pub fn update(&mut self, bg_surface:&mut Surface) {
        let mut block_number:usize = 0;
        let mut tile_rect = Rect::new(0, 0, 8, 8);

        for block in self.table.iter_mut() {
            if block.is_changed() && block.is_enabled() {
                let tile_bitmap = get_tile_bitmap(block.tile);
                let mut surface = self.surfaces[block_number].borrow_mut();
                let palette_offset = block.palette * 4;
                surface.with_lock_mut(|pixels: &mut [u8]| {
                    for i in 0..64 {
                        pixels[i] = tile_bitmap[i] + palette_offset;
                    }

                    if block.is_horizontally_flipped() {
                        let mut sx = 7;
                        let mut sy = 0;
                        for y in 0..8 {
                            for x in 0..4 {
                                let temp = pixels[y * 8 + x];
                                pixels[y * 8 + x] = pixels[sy * 8 + sx];
                                pixels[sy * 8 + sx] = temp;
                                sx -= 1;
                            }
                            sx = 7;
                            sy += 1;
                        }
                    }

                    if block.is_vertically_flipped() {
                        let mut sx = 0;
                        let mut sy = 7;
                        for y in 0..4 {
                            for x in 0..8 {
                                let temp = pixels[y * 8 + x];
                                pixels[y * 8 + x] = pixels[sy * 8 + sx];
                                pixels[sy * 8 + sx] = temp;
                                sx += 1
                            }
                            sx = 0;
                            sy -= 1;
                        }
                    }
                });
                block.changed(false);
                surface.blit(None, bg_surface, tile_rect);
            }

            tile_rect.offset(8, 0);
            if tile_rect.x() == 256 {
                tile_rect.offset(-256, 8);
            }

            block_number += 1;
        }
    }

    pub fn set(&mut self, tile_map:TileMaps) {
        use super::tile_maps::INTRO_MAP;

        match tile_map {
            TileMaps::LongIntroduction => {
                let mut index = 0;
                for entry in INTRO_MAP.iter() {
                    let mut block = &mut self.table[index];
                    block.enable(true);
                    block.tile(entry.0);
                    block.palette(entry.1);
                    block.vertical_flip(entry.2 & F_BG_VFLIP != 0);
                    block.horizontal_flip(entry.2 & F_BG_HFLIP != 0);
                    index += 1;
                }
            }

            _ => {
            }
        }
    }
}

impl BackgroundControlBlock {
    pub fn new_empty() -> BackgroundControlBlock {
        BackgroundControlBlock {
            tile: 0,
            flags: F_BG_NONE,
            palette: 0,
            user_data1: 0,
            user_data2: 0}
    }

    pub fn is_changed(&self) -> bool {
        self.flags & F_BG_CHANGED != 0
    }

    pub fn is_enabled(&self) -> bool {
        self.flags & F_BG_ENABLED != 0
    }

    pub fn tile(&mut self, number:u16) {
        self.tile = number;
        self.changed(true);
    }

    pub fn changed(&mut self, flag:bool) {
        if flag {
            self.flags |= F_BG_CHANGED;
        } else {
            self.flags &= !F_BG_CHANGED;
        }
    }

    pub fn palette(&mut self, number:u8) {
        self.palette = number;
        self.changed(true);
    }

    pub fn enable(&mut self, flag:bool) {
        if flag {
            self.flags |= F_BG_ENABLED;
        } else {
            self.flags &= !F_BG_ENABLED;
        }
    }

    pub fn vertical_flip(&mut self, flag:bool) {
        if flag {
            self.flags |= F_BG_VFLIP;
        } else {
            self.flags &= !F_BG_VFLIP;
        }
        self.changed(true);
    }

    pub fn is_vertically_flipped(&self) -> bool {
        self.flags & F_BG_VFLIP != 0
    }

    pub fn horizontal_flip(&mut self, flag:bool) {
        if flag {
            self.flags |= F_BG_HFLIP;
        } else {
            self.flags &= !F_BG_HFLIP;
        }
        self.changed(true);
    }

    pub fn is_horizontally_flipped(&self) -> bool {
        self.flags & F_BG_HFLIP != 0
    }
}