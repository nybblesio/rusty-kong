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

use sdl2::controller::GameController;
use super::GameState;

pub fn state_nop_enter(_game_state:&GameState) {
}

pub fn state_nop_leave(_game_state:&GameState) {
}

pub fn state_nop_update(_game_state:&GameState, _controller:&GameController) {
}
