use crate::players::player::Player;

pub struct Team{
    players: Vec<Player>,

}

pub struct Lineup{
    goalie: Option<Player>,
    defenders: Vec<Player>,
    midfielders: Vec<Player>,
    attackers: Vec<Player>,
}

impl Team {
    fn new() -> Self {
        Team {
            players: Vec::new(),
        }
    }
}

impl Lineup {
    fn new() -> Self {
        Lineup {
            goalie: None,
            defenders: Vec::new(),
            midfielders: Vec::new(),
            attackers: Vec::new()
        }
    }
}