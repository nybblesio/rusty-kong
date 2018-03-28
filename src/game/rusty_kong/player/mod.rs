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
pub struct JumpMan {
    y: u32,
    x: u32,
    lives: u32,
    score: u32
}

impl JumpMan {
    pub fn new() -> JumpMan {
        JumpMan {
            y: 0,
            x: 0,
            lives: 3,
            score: 0
        }
    }
}