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

use rusty_kong::player::player_update;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum GameState {
    None,
    Boot,
    Attract,
    LongIntroduction,
    HowHigh,
    GamePlay,
    PlayerDies,
    PlayerWins,
    KongRetreats,
}

struct StateHandlers {
    enter: fn(),
    update: fn(),
    leave: fn()
}

static mut PREVIOUS_STATE:GameState = GameState::None;
static mut CURRENT_STATE:GameState = GameState::None;
static mut NEXT_STATE:GameState = GameState::None;

#[allow(dead_code)]
lazy_static! {
    static ref STATE_HANDLERS:Vec<StateHandlers> = vec!(
        StateHandlers {enter: state_nop,     update: state_nop,      leave: state_nop},
        StateHandlers {enter: boot_enter,    update: boot_update,    leave: boot_leave},
        StateHandlers {enter: attract_enter, update: attract_update, leave: attract_leave}
    );
}

fn state_nop() {
}

fn boot_enter() {
}

fn boot_update() {
}

fn boot_leave() {
}

fn attract_enter() {
}

fn attract_update() {
}

fn attract_leave() {
}

pub fn game_state_go(state:GameState) {
    unsafe {
        NEXT_STATE = state;
    }
}

pub fn game_state_init() {
    game_state_go(GameState::Boot);
}

pub fn game_state_update() {
}