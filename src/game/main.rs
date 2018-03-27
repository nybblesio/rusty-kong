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

extern crate sdl2;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate simple_logger;

mod rusty_kong;

fn main() {
    simple_logger::init().unwrap();
    rusty_kong::game_run();
}
