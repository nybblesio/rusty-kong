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
use self::palettes::get_palette_colors;

mod sprites;
use self::sprites::get_sprite_bitmap;

mod tiles;
use self::tiles::get_tile_bitmap;

mod tile_maps;

use sdl2::Sdl;
use sdl2::pixels::Palette as SdlPalette;
use sdl2::pixels::Color;
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

        let mut bg_surface = Surface::new(256, 256, PixelFormatEnum::Index8)
            .unwrap();

        let palette = SdlPalette::with_colors(&get_palette_colors()).unwrap();
        bg_surface.set_palette(&palette);

        VideoGenerator {
            canvas,
            bg_surface,
            spr_cntl: SpriteControlTable::new(),
            bg1_cntl: BackgroundControlTable::new(),
        }
    }

    pub fn update(&mut self) {
        self.bg1_cntl.update(&mut self.bg_surface);
        let texture_creator = self.canvas.texture_creator();
        let texture = match texture_creator.create_texture_from_surface(&self.bg_surface) {
            Ok(t)  => t,
            Err(s) =>  {
                error!("create_texture_from_surface failed: {}", s);
                panic!();
            }
        };
        match self.canvas.copy(&texture, None, None) {
            Err(s) => error!("canvas copy failed: {}", s),
            _      => {},
        };
        self.spr_cntl.update();
        self.canvas.present();
    }

    pub fn set_bg(&mut self, tile_map:TileMaps) {
        self.bg1_cntl.set(tile_map);
    }
}