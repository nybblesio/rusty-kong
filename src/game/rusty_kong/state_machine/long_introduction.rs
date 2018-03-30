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

use super::GameStateContext;

pub fn long_intro_enter(context:&GameStateContext) {
    use super::super::video::TileMaps;

    info!("set bg1_cntl to long introduction tilemap.");
    context.set_bg(TileMaps::LongIntroduction);
}

pub fn long_intro_leave(context:&GameStateContext) {

}

pub fn long_intro_update(context:&GameStateContext) {

}
