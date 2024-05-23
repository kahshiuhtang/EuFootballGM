use rand::Rng;

use crate::teams::academy::{get_academy_impact, Academy};

use super::player_stats::{generate_attacker_potential, generate_attacker_stats, generate_defender_potential, generate_defender_stats, generate_goalie_potential, generate_goalie_stats, generate_midfielder_potential, generate_midfielder_stats, PlayerPotential, PlayerStats, Position};

pub struct Player{
    stats: PlayerStats,
    max_potential: PlayerPotential,
    age: i32,
    peak: i32,
}
const LOWER_BOUND_PEAK:i32 = 22;
const UPPER_BOUND_PEAK:i32 = 32;
pub fn generate_player(position: Position) -> Player{
    let age = 14;
    let mut rng = rand::thread_rng();
    let random_player_peak = rng.gen_range(LOWER_BOUND_PEAK..UPPER_BOUND_PEAK);
    match position {
        Position::GOALIE => Player { stats: generate_goalie_stats(), max_potential: generate_goalie_potential(), age, peak: random_player_peak },
        Position::DEFENDER => Player { stats: generate_defender_stats(), max_potential: generate_defender_potential(), age, peak: random_player_peak },
        Position::MIDFIELDER => Player { stats: generate_midfielder_stats(), max_potential: generate_midfielder_potential(), age, peak: random_player_peak },
        Position::ATTACKER => Player { stats: generate_attacker_stats(), max_potential: generate_attacker_potential(), age, peak: random_player_peak },
    }
}

pub fn progress_player(mut player: Player, academy: &Academy) -> Player{
    let progression_probability = get_academy_impact(academy);

    player.age += 1;
    if player.age <= player.peak{

    }
    return player
}