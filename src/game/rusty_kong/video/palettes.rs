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

lazy_static! {
	static ref PAL_CNTL:Vec<Palette> = vec!([
		// #0
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0x9d, b: 0x9e},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
			])
		},
		// #1
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
			])
		},
		// #2
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
				PaletteEntry {r: 0xdc, g: 0x02, b: 0x04},
			])
		},
		// #3
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
				PaletteEntry {r: 0x12, g: 0xef, b: 0x11},
			])
		},
		// #4
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
			])
		},
		// #5
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
				PaletteEntry {r: 0xe2, g: 0x0c, b: 0xeb},
				PaletteEntry {r: 0x1a, g: 0xf9, b: 0xf8},
			])
		},
		// #6
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14},
			])
		},
		// #7
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x07, g: 0x05, b: 0xac},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
			])
		},
		// #8
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x07, g: 0x05, b: 0xac},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
			])
		},
		// #9
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
				PaletteEntry {r: 0xe8, g: 0x57, b: 0xf0},
			])
		},
		// #10
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xdc, g: 0x02, b: 0x04},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
				PaletteEntry {r: 0xe8, g: 0x57, b: 0xf0},
			])
		},
		// #11
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
			])
		},
		// #12
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
			])
		},
		// #13
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
			])
		},
		// #14
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x9f, g: 0x9e, b: 0xd6},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
			])
		},
		// #15
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
			])
		},
		// #16
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0x9d, b: 0x9e},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
			])
		},
		// #17
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
			])
		},
		// #18
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
				PaletteEntry {r: 0xdc, g: 0x02, b: 0x04},
			])
		},
		// #19
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
			])
		},
		// #20
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
			])
		},
		// #21
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
			])
		},
		// #22
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
			])
		},
		// #23
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x07, g: 0x05, b: 0xac},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
			])
		},
		// #24
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x07, g: 0x05, b: 0xac},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
			])
		},
		// #25
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
				PaletteEntry {r: 0xe8, g: 0x57, b: 0xf0},
			])
		},
		// #26
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xdc, g: 0x02, b: 0x04},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
				PaletteEntry {r: 0xe8, g: 0x57, b: 0xf0},
			])
		},
		// #27
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
			])
		},
		// #28
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
			])
		},
		// #29
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
			])
		},
		// #30
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x9f, g: 0x9e, b: 0xd6},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
			])
		},
		// #31
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
			])
		},
		// #32
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0x9d, b: 0x9e},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
			])
		},
		// #33
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
			])
		},
		// #34
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
				PaletteEntry {r: 0xdc, g: 0x02, b: 0x04},
			])
		},
		// #35
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x94, g: 0x31, b: 0xec},
				PaletteEntry {r: 0x06, g: 0x04, b: 0x8e},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14},
			])
		},
		// #36
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x94, g: 0x31, b: 0xec},
				PaletteEntry {r: 0x06, g: 0x04, b: 0x8e},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14},
			])
		},
		// #37
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x94, g: 0x31, b: 0xec},
				PaletteEntry {r: 0x06, g: 0x04, b: 0x8e},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14},
			])
		},
		// #38
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x94, g: 0x31, b: 0xec},
				PaletteEntry {r: 0x06, g: 0x04, b: 0x8e},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14},
			])
		},
		// #39
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x07, g: 0x05, b: 0xac},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
			])
		},
		// #40
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x07, g: 0x05, b: 0xac},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
			])
		},
		// #41
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
				PaletteEntry {r: 0xe8, g: 0x57, b: 0xf0},
			])
		},
		// #42
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xdc, g: 0x02, b: 0x04},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
				PaletteEntry {r: 0xe8, g: 0x57, b: 0xf0},
			])
		},
		// #43
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
			])
		},
		// #44
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
			])
		},
		// #45
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
			])
		},
		// #46
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x9f, g: 0x9e, b: 0xd6},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
			])
		},
		// #47
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
			])
		},
		// #48
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0x9d, b: 0x9e},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
			])
		},
		// #49
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
			])
		},
		// #50
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
				PaletteEntry {r: 0xdc, g: 0x02, b: 0x04},
			])
		},
		// #51
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4},
			])
		},
		// #52
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4},
			])
		},
		// #53
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4},
			])
		},
		// #54
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4},
			])
		},
		// #55
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x07, g: 0x05, b: 0xac},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
			])
		},
		// #56
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x07, g: 0x05, b: 0xac},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
			])
		},
		// #57
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
				PaletteEntry {r: 0xe8, g: 0x57, b: 0xf0},
			])
		},
		// #58
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xdc, g: 0x02, b: 0x04},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
				PaletteEntry {r: 0xe8, g: 0x57, b: 0xf0},
			])
		},
		// #59
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
			])
		},
		// #60
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0xff, g: 0xfc, b: 0xfe},
				PaletteEntry {r: 0xff, g: 0xf3, b: 0x14},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
			])
		},
		// #61
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x0a, g: 0x07, b: 0xe8},
				PaletteEntry {r: 0x15, g: 0xba, b: 0xf4},
				PaletteEntry {r: 0x11, g: 0x75, b: 0xee},
			])
		},
		// #62
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x9f, g: 0x9e, b: 0xd6},
				PaletteEntry {r: 0xa0, g: 0xbc, b: 0xf5},
				PaletteEntry {r: 0xff, g: 0x03, b: 0x04},
			])
		},
		// #63
		Palette {
			entries: vec!([
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
				PaletteEntry {r: 0x00, g: 0x00, b: 0x00},
			])
		},
	]);
}
