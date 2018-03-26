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
