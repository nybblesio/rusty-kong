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

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Level {
    number: u32,
}

impl Level {
    pub fn new() -> Level {
        Level { number: 1 }
    }
}