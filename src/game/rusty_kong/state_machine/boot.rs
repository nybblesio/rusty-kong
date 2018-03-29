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

use super::SystemInterfaces;
use super::GameStates;
use super::GameState;

pub fn boot_enter(game_state:&mut GameState) {
    game_state.transition_to(GameStates::LongIntroduction);
}

pub fn boot_leave(game_state:&mut GameState) {
}

pub fn boot_update(game_state:&mut GameState) {
}
