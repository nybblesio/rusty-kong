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

use super::TileMaps;
use super::GameState;
use super::GameStates;
use super::SystemInterfaces;

pub trait GameCommand {
    fn execute(&self, game_state:&mut GameState, system:&mut SystemInterfaces);
}

// --------------------------------------------------------------------------
//
// transition_to
//
// --------------------------------------------------------------------------
pub struct TransitionToGameCommand {
    new_state: GameStates
}

impl TransitionToGameCommand {
    pub fn new(new_state: GameStates) -> TransitionToGameCommand {
        TransitionToGameCommand { new_state }
    }
}

impl GameCommand for TransitionToGameCommand {
    fn execute(&self, game_state:&mut GameState, system:&mut SystemInterfaces) {
        game_state.transition_to(self.new_state);
    }
}

// --------------------------------------------------------------------------
//
// set_bg
//
// --------------------------------------------------------------------------
pub struct SetBackgroundGameCommand {
    tile_map: Option<TileMaps>
}

impl SetBackgroundGameCommand {
    pub fn new(tile_map: Option<TileMaps>) -> SetBackgroundGameCommand {
        SetBackgroundGameCommand { tile_map }
    }
}

impl GameCommand for SetBackgroundGameCommand {
    fn execute(&self, game_state:&mut GameState, system:&mut SystemInterfaces) {
        let clone = system.video_gen.as_ref().unwrap().clone();
        let mut video_gen = (*clone).borrow_mut();
        let tile_map = self.tile_map.unwrap();
        video_gen.set_bg(tile_map);
    }
}

// --------------------------------------------------------------------------
//
// sprite
//
// --------------------------------------------------------------------------
pub struct SpriteGameCommand {
    x: u16,
    y: u16,
    tile: u16,
    flags: u8,
    number: u8,
    palette: u8,
}

impl SpriteGameCommand {
    pub fn new(number:u8, x:u16, y:u16, tile:u16, palette:u8, flags:u8) -> SpriteGameCommand {
        SpriteGameCommand { number, x, y, tile, palette, flags }
    }
}

impl GameCommand for SpriteGameCommand {
    fn execute(&self, game_state:&mut GameState, system:&mut SystemInterfaces) {
        let clone = system.video_gen.as_ref().unwrap().clone();
        let mut video_gen = (*clone).borrow_mut();
        video_gen.sprite(self.number, self.x, self.y, self.tile, self.palette, self.flags);
    }
}
