use crate::players::player::Player;

pub struct Event{
    action: Action,
    player_one: Player,
    player_two: Option<Player>,
}

pub enum Action{
    SHOOT,
    PASS,
    DRIBBLE,
    TACKLE,
    INTERCEPT,
    FOUL,
}