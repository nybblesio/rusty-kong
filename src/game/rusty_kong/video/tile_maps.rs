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

use super::common::*;

pub static INTRO_MAP:[TileMapEntry; 32*32] = [
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x18, 0x00, 0x00),TileMapEntry(0x19, 0x00, 0x00),TileMapEntry(0x17, 0x00, 0x00),TileMapEntry(0x18, 0x00, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x23, 0x00, 0x00),TileMapEntry(0x13, 0x00, 0x00),TileMapEntry(0x1f, 0x00, 0x00),TileMapEntry(0x22, 0x00, 0x00),TileMapEntry(0x15, 0x00, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x00, 0x01, 0x00),TileMapEntry(0x00, 0x01, 0x00),TileMapEntry(0x00, 0x01, 0x00),TileMapEntry(0x00, 0x01, 0x00),TileMapEntry(0x00, 0x01, 0x00),TileMapEntry(0x00, 0x01, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x00, 0x01, 0x00),TileMapEntry(0x00, 0x01, 0x00),TileMapEntry(0x07, 0x01, 0x00),TileMapEntry(0x06, 0x01, 0x00),TileMapEntry(0x05, 0x01, 0x00),TileMapEntry(0x00, 0x01, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xff, 0x02, 0x00),TileMapEntry(0xff, 0x02, 0x00),TileMapEntry(0xff, 0x02, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x1c, 0x02, 0x00),TileMapEntry(0x34, 0x02, 0x00),TileMapEntry(0x00, 0x02, 0x00),TileMapEntry(0x01, 0x02, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xd4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xd4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xd4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0xf4, 0x23, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xc4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xc4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0xe4, 0x23, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0xf5, 0x23, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0xe5, 0x23, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0xf6, 0x23, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0xe6, 0x23, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0xb8, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0xe7, 0x23, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xc0, 0x06, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0xd0, 0x23, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0xf1, 0x23, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
    TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0xb6, 0x00, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),TileMapEntry(0x0a, 0x0f, 0x00),
];
