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

use std::rc::Rc;
use std::marker::Sync;
use std::cell::RefCell;
use std::option::Option;

use sdl2::timer::Timer;
use sdl2::TimerSubsystem;
use sdl2::timer::TimerCallback;

use super::SpriteControlTable;

pub const F_ACTOR_NONE:u8       = 0b00000000;
pub const F_ACTOR_ENABLED:u8    = 0b00000001;
pub const F_ACTOR_COLLIDED:u8   = 0b00000010;
pub const F_ACTOR_HFLIP:u8      = 0b00000100;
pub const F_ACTOR_VFLIP:u8      = 0b00001000;

#[derive(Copy, Clone)]
pub struct AnimationFrameSprite {
    tile: u16,
    flags: u8,
    palette: u8,
    x_offset: u16,
    y_offset: u16,
}

pub struct AnimationFrame {
    duration: u32,
    sprites: Vec<AnimationFrameSprite>,
    timer: Option<TimerCallback<'static>>,
}

pub struct Animation {
    number: u8,
    frames: Rc<RefCell<Vec<AnimationFrame>>>
}

unsafe impl Sync for Animation {
}

impl Animation {
    pub fn new(frames:Vec<AnimationFrame>) -> Animation {
        Animation { frames: Rc::new(RefCell::new(frames)), number: 0 }
    }

    pub fn reset(&mut self) {
        self.number = 0;
        let clone = self.frames.clone();
        let mut frames = (*clone).borrow_mut();
        for anim_frame in frames.iter_mut() {
            anim_frame.timer = None;
        }
    }

    pub fn update(
            &mut self,
            sprite_control:&mut SpriteControlTable,
            timer_subsystem:&mut TimerSubsystem) {
        let frames_clone = self.frames.clone();
        let frames = (*frames_clone).borrow();

        let anim_frame = &frames[self.number as usize];
        let timer = match anim_frame.timer {
            None => {
                timer_subsystem.add_timer(
                    anim_frame.duration,
                    Box::new(move || self.frame_timer_callback()));
            }

            Some(ref t) => return
        };
    }

    fn frame_timer_callback(&mut self) -> u32 {
        let frames_clone = self.frames.clone();
        let frames = (*frames_clone).borrow();
        self.number += 1;
        if self.number > (frames.len() - 1) as u8 {
            self.number = 0;
        }
        0
    }
}

pub struct Actor {
    y: u16,
    x: u16,
    flags: u8,
    animation: Option<RefCell<Animation>>
}

impl Actor {
    pub fn new(x: u16, y: u16, flags: u8) -> Actor {
        Actor { x, y, flags, animation: None }
    }

    pub fn animation(&mut self, anim:Animation) {
        self.animation = Some(RefCell::new(anim));
    }

    pub fn position(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    pub fn update(
            &mut self,
            sprite_control:&mut SpriteControlTable,
            timer_subsystem:&mut TimerSubsystem) {
        let anim_ref = match self.animation {
            None        => return,
            Some(ref a) => a
        };
        let mut animation = (*anim_ref).borrow_mut();
        animation.update(sprite_control, timer_subsystem);
    }
}

pub struct ActorControlTable {
    actors: Rc<RefCell<Vec<Actor>>>,
}

impl ActorControlTable {
    pub fn new() -> ActorControlTable {
        ActorControlTable { actors: Rc::new(RefCell::new(vec![]))}
    }

    pub fn update(
            &mut self,
            sprite_control:&mut SpriteControlTable,
            timer_subsystem:&mut TimerSubsystem) {
        let clone = self.actors.clone();
        let mut actors = (*clone).borrow_mut();
        for actor in actors.iter_mut() {
            actor.update(sprite_control, timer_subsystem);
        }
    }
}