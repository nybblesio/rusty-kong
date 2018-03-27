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

pub fn long_intro_enter() {
    use super::super::video::TileMaps;
    use super::super::video::video_set_bg;

    video_set_bg(TileMaps::LongIntroduction);
}

pub fn long_intro_update() {

}

pub fn long_intro_leave() {

}
