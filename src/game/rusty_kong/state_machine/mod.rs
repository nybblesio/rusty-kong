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

mod boot;
use self::boot::*;

mod attract;
use self::attract::*;

mod long_introduction;
use self::long_introduction::*;

mod how_high;
use self::how_high::*;

mod game_play;
use self::game_play::*;

mod player_dies;
use self::player_dies::*;

mod player_wins;
use self::player_wins::*;

mod kong_retreats;
use self::kong_retreats::*;

mod state_nop;
use self::state_nop::*;

use super::TileMaps;
use super::SystemInterfaces;

use std::rc::Rc;
use std::fmt::Error;
use std::fmt::Display;
use std::cell::RefCell;
use std::fmt::Formatter;
use std::borrow::BorrowMut;

use super::level::Level;
use super::player::JumpMan;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum GameStates {
    None = 0 as isize,
    Boot,
    Attract,
    LongIntroduction,
    HowHigh,
    GamePlay,
    PlayerDies,
    PlayerWins,
    KongRetreats,
}

impl Display for GameStates {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &GameStates::None                => write!(f, "state_nop"),
            &GameStates::Boot                => write!(f, "boot"),
            &GameStates::Attract             => write!(f, "attract"),
            &GameStates::LongIntroduction    => write!(f, "long_intro"),
            &GameStates::HowHigh             => write!(f, "how_high"),
            &GameStates::GamePlay            => write!(f, "game_play"),
            &GameStates::PlayerDies          => write!(f, "player_dies"),
            &GameStates::PlayerWins          => write!(f, "player_wins"),
            &GameStates::KongRetreats        => write!(f, "kong_retreats")
        }
    }
}

struct StateHandlers {
    state: GameStates,
    first_update: RefCell<bool>,
    enter: fn(&mut GameState),
    leave: fn(&mut GameState),
    update: fn(&mut GameState),
}

impl StateHandlers {
    pub fn perform_enter(&self, game_state:&mut GameState) {
        debug!("calling {}_enter.", self.state);
        (self.enter)(game_state);
    }

    pub fn perform_leave(&self, game_state:&mut GameState) {
        debug!("calling {}_leave.", self.state);
        (self.leave)(game_state);
        let mut first_update = self.first_update.borrow_mut();
        *first_update = true;
    }

    pub fn perform_update(&self, game_state:&mut GameState) {
        let mut first_update = self.first_update.borrow_mut();
        if *first_update {
            debug!("calling {}_update.", self.state);
            debug!("NOTE: only the first call is logged to avoid noise.");
            *first_update = false;
        }
        (self.update)(game_state);
    }
}

pub struct GameState {
    level: Level,
    player: JumpMan,
    next: GameStates,
    current: GameStates,
    previous: GameStates,
    handlers: Vec<Rc<StateHandlers>>,
    system: Option<Rc<RefCell<&'static SystemInterfaces>>>,
}

impl GameState {
    pub fn init(system:&'static SystemInterfaces) -> GameState {
        let mut game_state = GameState::new();
        game_state.system(system);
        game_state.transition_to(GameStates::Boot);
        game_state
    }

    pub fn new() -> GameState {
        GameState {
            system:   None,
            level:    Level::new(),
            player:   JumpMan::new(),
            previous: GameStates::None,
            current:  GameStates::None,
            next:     GameStates::None,
            handlers: vec![
                Rc::new(StateHandlers {
                    state: GameStates::None,
                    enter: state_nop_enter,
                    update: state_nop_update,
                    leave: state_nop_leave,
                    first_update: RefCell::new(true)
                }),

                Rc::new(StateHandlers {
                    state: GameStates::Boot,
                    enter: boot_enter,
                    update: boot_update,
                    leave: boot_leave,
                    first_update: RefCell::new(true)
                }),

                Rc::new(StateHandlers {
                    state: GameStates::Attract,
                    enter: attract_enter,
                    update: attract_update,
                    leave: attract_leave,
                    first_update: RefCell::new(true)
                }),

                Rc::new(StateHandlers {
                    state: GameStates::LongIntroduction,
                    enter: long_intro_enter,
                    update: long_intro_update,
                    leave: long_intro_leave,
                    first_update: RefCell::new(true)
                }),

                Rc::new(StateHandlers {
                    state: GameStates::HowHigh,
                    enter: how_high_enter,
                    update: how_high_update,
                    leave: how_high_leave,
                    first_update: RefCell::new(true)
                }),

                Rc::new(StateHandlers {
                    state: GameStates::GamePlay,
                    enter: game_play_enter,
                    update: game_play_update,
                    leave: game_play_leave,
                    first_update: RefCell::new(true)
                }),

                Rc::new(StateHandlers {
                    state: GameStates::PlayerDies,
                    enter: player_dies_enter,
                    update: player_dies_update,
                    leave: player_dies_leave,
                    first_update: RefCell::new(true)
                }),

                Rc::new(StateHandlers {
                    state: GameStates::PlayerWins,
                    enter: player_wins_enter,
                    update: player_wins_update,
                    leave: player_wins_leave,
                    first_update: RefCell::new(true)
                }),

                Rc::new(StateHandlers {
                    state: GameStates::KongRetreats,
                    enter: kong_retreats_enter,
                    update: kong_retreats_update,
                    leave: kong_retreats_leave,
                    first_update: RefCell::new(true)
                }),
            ]
        }
    }

    fn system(&mut self, system:&'static SystemInterfaces) {
        let mut clone = self.system.as_ref().unwrap().clone();
        let mut sys = (*clone).borrow_mut();
        *sys = system;
    }

    fn is_pending_transition(&self) -> bool {
        self.next != GameStates::None
    }

    fn transition_states(&mut self) {
        debug!("transition to: {}.", self.next);
        self.previous = self.current;
        self.current = self.next;
        self.next = GameStates::None;
        self.leave_previous();
        self.enter_current();
    }

    pub fn transition_to(&mut self, state: GameStates) {
        self.next = state;
    }

    fn enter_current(&mut self) {
        let handlers = self.handlers[self.current as usize].clone();
        handlers.perform_enter(self);
    }

    fn leave_previous(&mut self) {
        let handlers = self.handlers[self.previous as usize].clone();
        handlers.perform_leave(self);
    }

    fn update_current(&mut self) {
        let handlers = self.handlers[self.current as usize].clone();
        handlers.perform_update(self);
    }

    pub fn update(&mut self) {
        if self.is_pending_transition() {
            self.transition_states();
        } else {
            self.update_current();
        }
    }

    pub fn set_bg(&mut self, tile_map:TileMaps) {
        let mut clone = self.system.as_ref().unwrap().clone();
        let sys = clone.borrow_mut();
        //let mut video_gen = sys.video_gen.as_ref().unwrap().clone();
    }
}
