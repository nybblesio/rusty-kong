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

static mut PREVIOUS_STATE:GameState = GameState::None;
static mut CURRENT_STATE:GameState = GameState::None;
static mut NEXT_STATE:GameState = GameState::None;

pub fn game_state_go(next_state:GameState) {
    unsafe {
        NEXT_STATE = next_state;
    }
}

pub fn game_state_init() {
    game_state_go(GameState::Boot);
}

pub fn game_state_update() {
}