extern crate rand;
use rand::Rng;

use super::player::Player;


pub struct PlayerStats {
    physical: PhysicalTraits,
    mental: MentalTraits,
    technique: TechnicalTraits,
    hidden: HiddenTraits,
    goalkeeping: GoalKeepingTraits,
}
pub struct PlayerPotential {
    physical: PhysicalTraits,
    mental: MentalTraits,
    technique: TechnicalTraits,
    hidden: HiddenTraits,
    goalkeeping: GoalKeepingTraits,
}
pub struct GoalKeepingTraits{ 
    aerial_ability: i32,
    command_of_area: i32,
    communication: i32,
    eccentricity: i32,
    handling: i32,
    kicking: i32,
    one_on_one: i32,
    reflexes: i32,
    rushing_out: i32,
    tendency_to_punch: i32,
    throwing: i32,
}

pub struct TechnicalTraits {
    dribbling: i32,
    first_touch: i32,
    free_kick_taking: i32,
    heading: i32,
    long_shots: i32,
    long_throws: i32,
    marking: i32,
    passing: i32,
    penalty_taking: i32,
    tackling: i32,
    technique: i32,
}

pub struct PhysicalTraits {
    acceleration: i32,
    agility: i32,
    jump_reach: i32,
    natural_fitness: i32,
    pace: i32,
    stamina: i32,
    strength: i32,
}
pub struct MentalTraits {
    aggression: i32,
    bravery: i32,
    composure: i32,
    concentration: i32,
    decisions: i32,
    determination: i32,
    flair: i32,
    leadership: i32,
    off_ball: i32,
    positioning: i32,
    teamwork: i32,
    vision: i32,
    work_rate: i32,
}

pub struct HiddenTraits {
    adaptability: i32,
    consistency: i32,
    dirtiniess: i32,
    injury_proneness: i32,
    versatility: i32
}

pub enum Position {
    GOALIE,
    DEFENDER,
    MIDFIELDER,
    ATTACKER,
}

const LOWER_BOUND_FLOOR_POOR_SKILL:i32 = 1;
const LOWER_BOUND_FLOOR_BASIC_SKILL:i32 = 3;
const LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL:i32 = 5;
const LOWER_BOUND_FLOOR_ADVANCED_SKILL:i32 = 7;

const UPPER_BOUND_FLOOR_POOR_SKILL:i32 = 4;
const UPPER_BOUND_FLOOR_BASIC_SKILL:i32 = 6;
const UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL:i32 = 8;
const UPPER_BOUND_FLOOR_ADVANCED_SKILL:i32 = 10;

const EXTRA_LOW_POTENTIAL: i32 = 2;
const LOW_POTENTIAL: i32 = 5;
const NORMAL_POTENTIAL: i32 = 7;
const HIGH_POTENTIAL: i32 = 9;
const SUPERB_POTENTIAL: i32 = 11;

pub fn generate_goalie_potential() -> PlayerPotential {
    PlayerPotential{
        physical: generate_physical_potential(Position::GOALIE),
        mental: generate_mental_potential(Position::GOALIE),
        technique: generate_technique_potential(Position::GOALIE),
        hidden: generate_hidden_potential(Position::GOALIE),
        goalkeeping: generate_goalkeeping_potential(Position::GOALIE),
    }
}
pub fn generate_defender_potential() -> PlayerPotential {
    PlayerPotential{
        physical: generate_physical_potential(Position::DEFENDER),
        mental: generate_mental_potential(Position::DEFENDER),
        technique: generate_technique_potential(Position::DEFENDER),
        hidden: generate_hidden_potential(Position::DEFENDER),
        goalkeeping: generate_goalkeeping_potential(Position::DEFENDER),
    }
}
pub fn generate_midfielder_potential() -> PlayerPotential {
    PlayerPotential{
        physical: generate_physical_potential(Position::MIDFIELDER),
        mental: generate_mental_potential(Position::MIDFIELDER),
        technique: generate_technique_potential(Position::MIDFIELDER),
        hidden: generate_hidden_potential(Position::MIDFIELDER),
        goalkeeping: generate_goalkeeping_potential(Position::MIDFIELDER),
    }
}
pub fn generate_attacker_potential() -> PlayerPotential {
    PlayerPotential{
        physical: generate_physical_potential(Position::ATTACKER),
        mental: generate_mental_potential(Position::ATTACKER),
        technique: generate_technique_potential(Position::ATTACKER),
        hidden: generate_hidden_potential(Position::ATTACKER),
        goalkeeping: generate_goalkeeping_potential(Position::ATTACKER),
    }
}

pub fn generate_goalie_stats() -> PlayerStats{
    PlayerStats{
        physical: generate_physical(Position::GOALIE),
        mental: generate_mental(Position::GOALIE),
        technique: generate_technique(Position::GOALIE),
        hidden: generate_hidden(Position::GOALIE),
        goalkeeping: generate_goalkeeping(Position::GOALIE),
    }
}

pub fn generate_midfielder_stats() -> PlayerStats{
    PlayerStats{
        physical: generate_physical(Position::MIDFIELDER),
        mental: generate_mental(Position::MIDFIELDER),
        technique: generate_technique(Position::MIDFIELDER),
        hidden: generate_hidden(Position::MIDFIELDER),
        goalkeeping: generate_goalkeeping(Position::MIDFIELDER),
    }
}

pub fn generate_attacker_stats() -> PlayerStats{
    PlayerStats{
        physical: generate_physical(Position::ATTACKER),
        mental: generate_mental(Position::ATTACKER),
        technique: generate_technique(Position::ATTACKER),
        hidden: generate_hidden(Position::ATTACKER),
        goalkeeping: generate_goalkeeping(Position::ATTACKER),
    }
}

pub fn generate_defender_stats() -> PlayerStats{
    PlayerStats{
        physical: generate_physical(Position::DEFENDER),
        mental: generate_mental(Position::DEFENDER),
        technique: generate_technique(Position::DEFENDER),
        hidden: generate_hidden(Position::DEFENDER),
        goalkeeping: generate_goalkeeping(Position::DEFENDER),
    }
}

pub fn generate_physical_potential(position: Position) -> PhysicalTraits{
    let mut rng = rand::thread_rng();
    match position {
        Position::GOALIE => return PhysicalTraits{
            acceleration: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            agility: rng.gen_range(LOW_POTENTIAL..HIGH_POTENTIAL),
            jump_reach: rng.gen_range(NORMAL_POTENTIAL..NORMAL_POTENTIAL),
            natural_fitness: rng.gen_range(NORMAL_POTENTIAL..HIGH_POTENTIAL),
            pace: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            stamina: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            strength: rng.gen_range(NORMAL_POTENTIAL..HIGH_POTENTIAL),
        },
        Position::DEFENDER => PhysicalTraits{
            acceleration: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            agility: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            jump_reach: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            natural_fitness: rng.gen_range(NORMAL_POTENTIAL..HIGH_POTENTIAL),
            pace: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            stamina: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            strength: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
        },
        Position::MIDFIELDER => PhysicalTraits{
            acceleration: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            agility: rng.gen_range(NORMAL_POTENTIAL..HIGH_POTENTIAL),
            jump_reach: rng.gen_range(EXTRA_LOW_POTENTIAL..SUPERB_POTENTIAL),
            natural_fitness: rng.gen_range(NORMAL_POTENTIAL..HIGH_POTENTIAL),
            pace: rng.gen_range(LOW_POTENTIAL..HIGH_POTENTIAL),
            stamina: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            strength: rng.gen_range(LOW_POTENTIAL..NORMAL_POTENTIAL),
        },
        Position::ATTACKER => PhysicalTraits{
            acceleration: rng.gen_range(EXTRA_LOW_POTENTIAL..SUPERB_POTENTIAL),
            agility: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            jump_reach: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            natural_fitness: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            pace: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            stamina: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            strength: rng.gen_range(EXTRA_LOW_POTENTIAL..SUPERB_POTENTIAL),
        },
    }
}
pub fn generate_mental_potential(position: Position) -> MentalTraits{
    let mut rng = rand::thread_rng();
    match position {
        Position::GOALIE => return MentalTraits{
            aggression: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            bravery: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            composure: rng.gen_range(EXTRA_LOW_POTENTIAL..SUPERB_POTENTIAL),
            concentration: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            decisions: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            determination: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            flair: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            leadership: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            off_ball: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            positioning: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            teamwork: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            vision: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            work_rate: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
        },
        Position::DEFENDER => return MentalTraits{
            aggression: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            bravery: rng.gen_range(EXTRA_LOW_POTENTIAL..SUPERB_POTENTIAL),
            composure: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            concentration: rng.gen_range(NORMAL_POTENTIAL..HIGH_POTENTIAL),
            decisions: rng.gen_range(EXTRA_LOW_POTENTIAL..SUPERB_POTENTIAL),
            determination: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            flair: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            leadership: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            off_ball: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            positioning: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            teamwork: rng.gen_range(EXTRA_LOW_POTENTIAL..SUPERB_POTENTIAL),
            vision: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            work_rate: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
        },
        Position::MIDFIELDER => return MentalTraits{
            aggression: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            bravery: rng.gen_range(LOW_POTENTIAL..NORMAL_POTENTIAL),
            composure: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            concentration: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            decisions: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            determination: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            flair: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            leadership: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            off_ball: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            positioning: rng.gen_range(LOW_POTENTIAL..HIGH_POTENTIAL),
            teamwork: rng.gen_range(EXTRA_LOW_POTENTIAL..SUPERB_POTENTIAL),
            vision: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            work_rate: rng.gen_range(NORMAL_POTENTIAL..HIGH_POTENTIAL),
        },
        Position::ATTACKER => return MentalTraits{
            aggression: rng.gen_range(EXTRA_LOW_POTENTIAL..SUPERB_POTENTIAL),
            bravery: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            composure: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            concentration: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            decisions: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            determination: rng.gen_range(EXTRA_LOW_POTENTIAL..SUPERB_POTENTIAL),
            flair: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            leadership: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            off_ball: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            positioning: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            teamwork: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            vision: rng.gen_range(EXTRA_LOW_POTENTIAL..SUPERB_POTENTIAL),
            work_rate: rng.gen_range(EXTRA_LOW_POTENTIAL..SUPERB_POTENTIAL),
        },
    }
}

pub fn generate_hidden_potential(position: Position) -> HiddenTraits{
    let mut rng = rand::thread_rng();
    match position {
        Position::GOALIE => return HiddenTraits{
            adaptability: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            consistency: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            dirtiniess: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            injury_proneness: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            versatility: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
        },
        Position::DEFENDER => return HiddenTraits{
            adaptability: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            consistency: rng.gen_range(EXTRA_LOW_POTENTIAL..SUPERB_POTENTIAL),
            dirtiniess: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            injury_proneness: rng.gen_range(LOW_POTENTIAL..HIGH_POTENTIAL),
            versatility: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
        },
        Position::MIDFIELDER => return HiddenTraits{
            adaptability: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            consistency: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            dirtiniess: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            injury_proneness: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            versatility: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
        },
        Position::ATTACKER => return HiddenTraits{
            adaptability: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            consistency: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            dirtiniess: rng.gen_range(EXTRA_LOW_POTENTIAL..SUPERB_POTENTIAL),
            injury_proneness: rng.gen_range(LOW_POTENTIAL..HIGH_POTENTIAL),
            versatility: rng.gen_range(LOW_POTENTIAL..HIGH_POTENTIAL),
        },
    }
}

pub fn generate_technique_potential(position: Position) -> TechnicalTraits{
    let mut rng = rand::thread_rng();
    match position {
        Position::GOALIE => return TechnicalTraits{
            dribbling: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            first_touch: rng.gen_range(LOW_POTENTIAL..NORMAL_POTENTIAL),
            free_kick_taking: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            heading: rng.gen_range(LOW_POTENTIAL..NORMAL_POTENTIAL),
            long_shots: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            long_throws: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            marking: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            passing: rng.gen_range(LOW_POTENTIAL..HIGH_POTENTIAL),
            penalty_taking: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            tackling: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            technique: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
        },
        Position::DEFENDER => return TechnicalTraits{
            dribbling: rng.gen_range(LOW_POTENTIAL..NORMAL_POTENTIAL),
            first_touch: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            free_kick_taking: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            heading: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            long_shots: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            long_throws: rng.gen_range(LOW_POTENTIAL..NORMAL_POTENTIAL),
            marking: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            passing: rng.gen_range(NORMAL_POTENTIAL..HIGH_POTENTIAL),
            penalty_taking: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            tackling: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            technique: rng.gen_range(LOW_POTENTIAL..HIGH_POTENTIAL),
        },
        Position::MIDFIELDER => return TechnicalTraits{
            dribbling: rng.gen_range(LOW_POTENTIAL..HIGH_POTENTIAL),
            first_touch: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            free_kick_taking: rng.gen_range(LOW_POTENTIAL..HIGH_POTENTIAL),
            heading: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            long_shots: rng.gen_range(LOW_POTENTIAL..HIGH_POTENTIAL),
            long_throws: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            marking: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            passing: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            penalty_taking: rng.gen_range(LOW_POTENTIAL..HIGH_POTENTIAL),
            tackling: rng.gen_range(EXTRA_LOW_POTENTIAL..HIGH_POTENTIAL),
            technique: rng.gen_range(LOW_POTENTIAL..HIGH_POTENTIAL),
        },
        Position::ATTACKER => return TechnicalTraits{
            dribbling: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            first_touch: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            free_kick_taking: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            heading: rng.gen_range(EXTRA_LOW_POTENTIAL..SUPERB_POTENTIAL),
            long_shots: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            long_throws: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            marking: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            passing: rng.gen_range(EXTRA_LOW_POTENTIAL..SUPERB_POTENTIAL),
            penalty_taking: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            tackling: rng.gen_range(EXTRA_LOW_POTENTIAL..NORMAL_POTENTIAL),
            technique: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
        },
    }
}
pub fn generate_goalkeeping_potential(position: Position) -> GoalKeepingTraits{
    let mut rng = rand::thread_rng();
    match position {
        Position::GOALIE => return GoalKeepingTraits{
            aerial_ability: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            command_of_area: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            communication: rng.gen_range(NORMAL_POTENTIAL..SUPERB_POTENTIAL),
            eccentricity: rng.gen_range(LOW_POTENTIAL..HIGH_POTENTIAL),
            handling: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            kicking: rng.gen_range(LOW_POTENTIAL..HIGH_POTENTIAL),
            one_on_one: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            reflexes: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            rushing_out: rng.gen_range(LOW_POTENTIAL..SUPERB_POTENTIAL),
            tendency_to_punch:rng.gen_range(LOW_POTENTIAL..HIGH_POTENTIAL),
            throwing: rng.gen_range(LOW_POTENTIAL..HIGH_POTENTIAL),
        },
        Position::DEFENDER => return GoalKeepingTraits{
            aerial_ability: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            command_of_area: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            communication: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            eccentricity: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            handling: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            kicking: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            one_on_one: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            reflexes: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            rushing_out: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            tendency_to_punch:rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            throwing: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
        },
        Position::MIDFIELDER => return GoalKeepingTraits{
            aerial_ability: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            command_of_area: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            communication: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            eccentricity: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            handling: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            kicking: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            one_on_one: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            reflexes: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            rushing_out: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            tendency_to_punch:rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            throwing: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
        },
        Position::ATTACKER => return GoalKeepingTraits{
            aerial_ability: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            command_of_area: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            communication: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            eccentricity: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            handling: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            kicking: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            one_on_one: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            reflexes: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            rushing_out: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            tendency_to_punch:rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
            throwing: rng.gen_range(EXTRA_LOW_POTENTIAL..LOW_POTENTIAL),
        },
    }
}
pub fn generate_physical(position: Position) -> PhysicalTraits{
    let mut rng = rand::thread_rng();
    match position {
        Position::GOALIE => return PhysicalTraits{
            acceleration: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            agility: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            jump_reach: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            natural_fitness: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            pace: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            stamina: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            strength: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
        },
        Position::DEFENDER => PhysicalTraits{
            acceleration: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            agility:rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            jump_reach: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            natural_fitness: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            pace: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            stamina: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            strength: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
        },
        Position::MIDFIELDER => PhysicalTraits{
            acceleration: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            agility: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            jump_reach: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            natural_fitness: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            pace: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            stamina: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            strength: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
        },
        Position::ATTACKER => PhysicalTraits{
            acceleration: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            agility: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            jump_reach: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            natural_fitness:rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            pace: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            stamina: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            strength: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
        },
    }
}

pub fn generate_mental(position: Position) -> MentalTraits{
    let mut rng = rand::thread_rng();
    match position {
        Position::GOALIE => MentalTraits{
            aggression: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            bravery: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            composure: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            concentration: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            decisions: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            determination: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            flair: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            leadership: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            off_ball: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            positioning: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            teamwork: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            vision: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            work_rate: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
        },
        Position::DEFENDER =>  MentalTraits{
            aggression: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            bravery: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            composure: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            concentration: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            decisions: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            determination: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            flair: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            leadership: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            off_ball: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            positioning: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            teamwork: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            vision: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            work_rate: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
        },
        Position::MIDFIELDER =>  MentalTraits{
            aggression: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            bravery: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            composure: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            concentration: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            decisions: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            determination:rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            flair: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            leadership: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            off_ball: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            positioning: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            teamwork: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            vision: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            work_rate: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
        },
        Position::ATTACKER =>  MentalTraits{
            aggression: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            bravery: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            composure: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            concentration:rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            decisions: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            determination: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            flair: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            leadership: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            off_ball: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            positioning: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            teamwork: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            vision: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            work_rate:rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
        }, 
    }
}

pub fn generate_technique(position: Position) -> TechnicalTraits{
    let mut rng = rand::thread_rng();
    match position {
        Position::GOALIE => TechnicalTraits{
            dribbling: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            first_touch: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            free_kick_taking: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            heading: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            long_shots:rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            long_throws:rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            marking: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            passing: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            penalty_taking: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            tackling: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            technique: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
        },
        Position::DEFENDER => TechnicalTraits{
            dribbling: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            first_touch: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            free_kick_taking: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            heading: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            long_shots: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            long_throws: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            marking: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            passing: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            penalty_taking: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            tackling: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            technique: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
        },
        Position::MIDFIELDER => TechnicalTraits{
            dribbling: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            first_touch:rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            free_kick_taking: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            heading: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            long_shots: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            long_throws: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            marking: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            passing: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            penalty_taking: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            tackling:rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            technique:rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
        },
        Position::ATTACKER => TechnicalTraits{
            dribbling: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            first_touch: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            free_kick_taking: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            heading: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            long_shots: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            long_throws: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            marking: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            passing: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            penalty_taking: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            tackling: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            technique: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
        },
    }
}
pub fn generate_hidden(position: Position) -> HiddenTraits{
    let mut rng = rand::thread_rng();
    match position {
        Position::GOALIE => HiddenTraits{
            adaptability: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            consistency: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            dirtiniess: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            injury_proneness: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            versatility: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
        },
        Position::DEFENDER => HiddenTraits{
            adaptability: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            consistency: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            dirtiniess: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            injury_proneness: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            versatility: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
        },
        Position::MIDFIELDER => HiddenTraits{
            adaptability: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            consistency: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
            dirtiniess: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            injury_proneness: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            versatility: rng.gen_range(LOWER_BOUND_FLOOR_ADVANCED_SKILL..UPPER_BOUND_FLOOR_ADVANCED_SKILL),
        },
        Position::ATTACKER => HiddenTraits{
            adaptability:rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            consistency: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            dirtiniess: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            injury_proneness: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            versatility: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
        },
    }
    
}
pub fn generate_goalkeeping(position: Position) -> GoalKeepingTraits{
    let mut rng = rand::thread_rng();
    match position {
        Position::GOALIE =>  GoalKeepingTraits{
            aerial_ability: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            command_of_area: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            communication: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            eccentricity: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            handling: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            kicking: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            one_on_one: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            reflexes: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            rushing_out: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            tendency_to_punch: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            throwing: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
        },
        Position::DEFENDER =>  GoalKeepingTraits{
            aerial_ability: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            command_of_area: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            communication: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            eccentricity: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            handling: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            kicking: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            one_on_one: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            reflexes: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            rushing_out: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            tendency_to_punch: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            throwing: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
        },
        Position::MIDFIELDER =>  GoalKeepingTraits{
            aerial_ability: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            command_of_area: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            communication: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            eccentricity: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            handling: rng.gen_range(LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL..UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL),
            kicking: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            one_on_one: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            reflexes: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            rushing_out: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            tendency_to_punch: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            throwing: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
        },
        Position::ATTACKER => GoalKeepingTraits{
            aerial_ability: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            command_of_area: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            communication: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            eccentricity: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            handling: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            kicking: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            one_on_one: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            reflexes: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
            rushing_out: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            tendency_to_punch: rng.gen_range(LOWER_BOUND_FLOOR_POOR_SKILL..UPPER_BOUND_FLOOR_POOR_SKILL),
            throwing: rng.gen_range(LOWER_BOUND_FLOOR_BASIC_SKILL..UPPER_BOUND_FLOOR_BASIC_SKILL),
        },
    }
    
}
