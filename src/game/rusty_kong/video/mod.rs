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
pub use self::common::TileMaps;

mod palettes;
use self::palettes::get_palette;

mod sprites;
use self::sprites::get_sprite_bitmap;

mod tiles;
use self::tiles::get_tile_bitmap;

mod tile_maps;

use sdl2;
use sdl2::Sdl;
use sdl2::surface::Surface;
use sdl2::render::WindowCanvas;
use sdl2::pixels::PixelFormatEnum;

pub struct VideoGenerator {
    canvas: WindowCanvas,
    bg_surface: Surface<'static>,
    spr_cntl: SpriteControlTable,
    bg1_cntl: BackgroundControlTable
}

impl VideoGenerator {
    pub fn init(context:&Sdl) -> VideoGenerator {
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem.window("Rusty Kong", SCREEN_WIDTH * 4, SCREEN_HEIGHT * 4)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .unwrap();
        let bg_surface = Surface::new(256, 256, PixelFormatEnum::Index8)
            .unwrap();
        VideoGenerator {
            canvas,
            bg_surface,
            spr_cntl: SpriteControlTable::new(),
            bg1_cntl: BackgroundControlTable::new(),
        }
    }

    pub fn update(&mut self) {
        self.canvas.present();
    }

    pub fn set_bg(&mut self, tile_map:TileMaps) {
        self.bg1_cntl.set(tile_map);
    }
}