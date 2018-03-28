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

use sdl2::Sdl;
use sdl2::surface::Surface;
use sdl2::render::WindowCanvas;
use sdl2::pixels::PixelFormatEnum;

pub struct VideoGenerator<'a> {
    canvas: WindowCanvas,
    bg_surface: Surface<'a>,
    spr_cntl: SpriteControlTable,
    bg1_cntl: BackgroundControlTable
}

fn video_bg(video: &mut VideoGenerator) {
    // XXX: need Surface that is the background buffer
    //      only changed tiles get rendered into this buffer
    //      copy this into canvas
//    for bg_cntl in video.iter() {
//        bg_cntl.update();
//    }
}

fn video_fg(video: &mut VideoGenerator) {
//    for fg_cntl in SPR_CNTL.iter() {
//        // XXX: update internal surface
//        //      copy to canvas
//        fg_cntl.update();
//    }
}

pub fn video_set_bg(map: TileMaps) {

}

pub fn video_update(video:&mut VideoGenerator) {
    video_bg(video);
    //canvas.copy(background_surface, .....);
    video_fg(video);
    video.canvas.present();
}

pub fn video_init(sdl_context: &Sdl) -> VideoGenerator {
    let video_subsystem = sdl_context.video().unwrap();
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
