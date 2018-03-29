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

pub fn boot_enter(system:&SystemInterfaces) {
    system.transition_to(GameStates::LongIntroduction);
}

pub fn boot_leave(system:&SystemInterfaces) {
}

pub fn boot_update(system:&SystemInterfaces) {
}
