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

extern crate sdl2;

use std::env;
use std::vec::Vec;
use std::path::Path;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::image::{LoadTexture, INIT_PNG};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[allow(dead_code)]
enum ToolMode {
    Palette,
    Tiles,
    Sprites
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct PaletteEntry(u8, u8, u8);

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Palette {
    entries: [PaletteEntry; 4]
}

fn run(mode:ToolMode, png: &Path) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    struct WindowSize(u32, u32);
    let window_size = match mode {
        ToolMode::Palette   => WindowSize(800,  600),
        ToolMode::Tiles     => WindowSize(1151, 17),
        ToolMode::Sprites   => WindowSize(1155, 33)
    };

    let _image_context = sdl2::image::init(INIT_PNG).unwrap();
    let window = video_subsystem.window("Rusty Kong Ripper Tool", window_size.0, window_size.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().software().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(png).unwrap();
    let texture_info = texture.query();

    canvas
        .copy(
            &texture,
            None,
            Rect::new(0, 0, texture_info.width, texture_info.height))
        .expect("Render failed");
    canvas.present();

    match mode {
        ToolMode::Palette => {
            let mut palettes = [Palette {entries: [PaletteEntry(0, 0, 0); 4]}; 64];
            let mut current_palette: Palette = Palette {entries: [PaletteEntry(0, 0, 0); 4]};

            let mut palette_idx:usize = 0;
            let mut palette_entry_idx = 0;

            let mut _cy = 2;
            let mut _cx = 2;

            for _y in 0..16 {
                for _x in 0..16 {
                    if palette_entry_idx == 4 {
                        palettes[palette_idx] = current_palette;
                        palette_idx += 1;
                        palette_entry_idx = 0;
                    }

                    let pixels = canvas
                        .read_pixels(Rect::new(_cx, _cy, 1, 1), canvas.default_pixel_format())
                        .unwrap();
                    current_palette.entries[palette_entry_idx] = PaletteEntry(pixels[0], pixels[1], pixels[2]);
                    palette_entry_idx += 1;

                    _cx += 50;
                }

                _cx = 2;
                _cy += 38;
            }

            println!("lazy_static! {{");
            println!("\tstatic ref PAL_CNTL:Vec<Palette> = vec!([");
            let mut palette_number = 0;
            for palette in palettes.iter() {
                println!("\t\t// #{}", palette_number);
                println!("\t\tPalette {{");
                println!("\t\t\tentries: vec!([");
                for entry in palette.entries.iter() {
                    println!(
                        "\t\t\t\tPaletteEntry {{r: 0x{:02x}, g: 0x{:02x}, b: 0x{:02x}}},",
                        entry.0,
                        entry.1,
                        entry.2);
                }
                println!("\t\t\t])");
                println!("\t\t}},");
                palette_number += 1;
            }
            println!("\t]);");
            println!("}}");
        }

        ToolMode::Tiles => {
            let mut _cy = 0;
            let mut _cx = 0;
            let mut match_palette: Palette = Palette {entries: [
                PaletteEntry(0x00, 0x00, 0x00),
                PaletteEntry(0x94, 0x31, 0xec),
                PaletteEntry(0x06, 0x04, 0x8e),
                PaletteEntry(0xff, 0xf3, 0x14),
            ]};

            let mut tiles = [[0 as u8; 8*8]; 256];
            let mut tile_idx = 0;

            for _ty in 0..2 {
                for _tx in 0..128 {
                    let mut tile_data = [0 as u8; 8*8];
                    for _y in 0..8 as usize {
                        for _x in 0..8 as usize {
                            let pixels = canvas
                                .read_pixels(
                                    Rect::new((_cx + _x) as i32, (_cy + _y) as i32, 1, 1),
                                    canvas.default_pixel_format())
                                .unwrap();
                            let mut entry_idx = 0;
                            for entry in match_palette.entries.iter() {
                                if entry.0 == pixels[0]
                                && entry.1 == pixels[1]
                                && entry.2 == pixels[2] {
                                    tile_data[_y * 8 + _x] = entry_idx;
                                    break;
                                }
                                entry_idx += 1;
                            }
                        }
                    }
                    tiles[tile_idx] = tile_data;
                    tile_idx += 1;
                    _cx += 9;
                }
                _cx = 0;
                _cy += 9;
            }

            println!("lazy_static! {{");
            println!("\tstatic ref TILE_BITMAPS = [");
            let mut tile_number = 0;
            for tile_data in tiles.iter() {
                println!("\t\t// tile #{}", tile_number);
                print!("\t\t[");
                for _y in 0..8 as usize {
                    for _x in 0..8 as usize {
                        if _x > 0 {
                            print!(",");
                        }
                        print!("0x{:02x}", tile_data[_y * 8 + _x]);
                    }
                    if _y < 7 {
                        print!("\n\t\t ");
                    }
                }
                println!("],\n");
                tile_number += 1;
            }
            println!("];");
            println!("}}");
        }

        ToolMode::Sprites => {
            let mut _cy = 0;
            let mut _cx = 0;
            let mut match_palette: Palette = Palette {entries: [
                PaletteEntry( 0x00, 0x00, 0x00),
                PaletteEntry( 0xff, 0x9d, 0x9e),
                PaletteEntry( 0xff, 0xf3, 0x14),
                PaletteEntry( 0x0a, 0x07, 0xe8),
            ]};

            let mut sprites = [[0 as u8; 16*16]; 128];
            let mut sprite_idx = 0;

            for _ty in 0..2 {
                for _tx in 0..67 {
                    if sprite_idx == 96 {
                        break;
                    }
                    let mut spr_data = [0 as u8; 16*16];
                    for _y in 0..16 as usize {
                        for _x in 0..16 as usize {
                            let pixels = canvas
                                .read_pixels(
                                    Rect::new((_cx + _x) as i32, (_cy + _y) as i32, 1, 1),
                                    canvas.default_pixel_format())
                                .unwrap();
                            let mut entry_idx = 0;
                            for entry in match_palette.entries.iter() {
                                if entry.0 == pixels[0]
                                    && entry.1 == pixels[1]
                                    && entry.2 == pixels[2] {
                                    spr_data[_y * 16 + _x] = entry_idx;
                                    break;
                                }
                                entry_idx += 1;
                            }
                        }
                    }
                    sprites[sprite_idx] = spr_data;
                    sprite_idx += 1;
                    _cx += 17;
                }
                _cx = 0;
                _cy += 17;
            }

            println!("lazy_static! {{");
            println!("\tstatic ref SPRITE_BITMAPS = [");
            let mut sprite_number = 0;
            for spr_data in sprites.iter() {
                println!("\t\t// sprite #{}", sprite_number);
                print!("\t\t[");
                for _y in 0..16 as usize {
                    for _x in 0..16 as usize {
                        if _x > 0 {
                            print!(",");
                        }
                        print!("0x{:02x}", spr_data[_y * 16 + _x]);
                    }
                    if _y < 15 {
                        print!("\n\t\t ");
                    }
                }
                println!("],\n");
                sprite_number += 1;
            }
            println!("];");
            println!("}}");
        }
    }

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }
    }

}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run pal|tiles|sprites /path/to/image.png")
    } else {
        let mode = match args[1].as_ref() {
            "pal"     => ToolMode::Palette,
            "tile"    => ToolMode::Tiles ,
            "sprite"  => ToolMode::Sprites,
            _         => panic!("Unknown mode!")
        };

        run(mode, Path::new(&args[2]));
    }
}