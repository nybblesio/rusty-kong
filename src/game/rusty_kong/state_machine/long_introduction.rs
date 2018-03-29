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

pub fn long_intro_enter(system:&SystemInterfaces) {
    use super::super::video::TileMaps;

    info!("set bg1_cntl to long introduction tilemap.");
    system.set_bg(TileMaps::LongIntroduction);
}

pub fn long_intro_leave(system:&SystemInterfaces) {

}

pub fn long_intro_update(system:&SystemInterfaces) {

}
