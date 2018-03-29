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

use std::borrow::BorrowMut;
use super::GameState;

pub fn long_intro_enter(game_state:&mut GameState) {
    use super::super::video::TileMaps;

    info!("set bg1_cntl to long introduction tilemap.");
    game_state.set_bg(TileMaps::LongIntroduction);
}

pub fn long_intro_leave(game_state:&mut GameState) {

}

pub fn long_intro_update(game_state:&mut GameState) {

}
