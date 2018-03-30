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

use super::GameStates;
use super::GameStateContext;

pub fn boot_enter(context:&GameStateContext) {
    context.transition_to(GameStates::LongIntroduction);
}

pub fn boot_leave(context:&GameStateContext) {
}

pub fn boot_update(context:&GameStateContext) {
}
