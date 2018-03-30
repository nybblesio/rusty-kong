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
use super::super::video::common::F_SPR_HFLIP;
use super::super::video::common::F_SPR_ENABLED;

pub fn long_intro_enter(context:&GameStateContext) {
    use super::super::video::TileMaps;

    info!("set bg1_cntl to long introduction tilemap.");
    context.set_bg(TileMaps::LongIntroduction);
    context.sprite(0, 16, 16, 0, 2, F_SPR_HFLIP | F_SPR_ENABLED);
}

pub fn long_intro_leave(context:&GameStateContext) {

}

pub fn long_intro_update(context:&GameStateContext) {

}
