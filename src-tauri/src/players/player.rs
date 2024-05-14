extern crate rand;
use rand::Rng;

pub enum Position {
    GOALIE,
    DEFENDER,
    MIDFIELDER,
    ATTACKER,
}

const LOWER_BOUND_FLOOR_POOR_SKILL:i64 = 1;
const LOWER_BOUND_FLOOR_BASIC_SKILL:i64 = 3;
const LOWER_BOUND_FLOOR_INTERMEDIATE_SKILL:i64 = 5;
const LOWER_BOUND_FLOOR_ADVANCED_SKILL:i64 = 7;

const UPPER_BOUND_FLOOR_POOR_SKILL:i64 = 4;
const UPPER_BOUND_FLOOR_BASIC_SKILL:i64 = 6;
const UPPER_BOUND_FLOOR_INTERMEDIATE_SKILL:i64 = 8;
const UPPER_BOUND_FLOOR_ADVANCED_SKILL:i64 = 10;


pub fn generate_goalie() -> Player{
    Player{
        physical: generate_physical(Position::GOALIE),
        mental: generate_mental(Position::GOALIE),
        technique: generate_technique(Position::GOALIE),
        hidden: generate_hidden(Position::GOALIE),
        goalkeeping: generate_goalkeeping(Position::GOALIE),
    }
}

pub fn generate_midfielder() -> Player{
    Player{
        physical: generate_physical(Position::MIDFIELDER),
        mental: generate_mental(Position::MIDFIELDER),
        technique: generate_technique(Position::MIDFIELDER),
        hidden: generate_hidden(Position::MIDFIELDER),
        goalkeeping: generate_goalkeeping(Position::MIDFIELDER),
    }
}

pub fn generate_attacker() -> Player{
    Player{
        physical: generate_physical(Position::ATTACKER),
        mental: generate_mental(Position::ATTACKER),
        technique: generate_technique(Position::ATTACKER),
        hidden: generate_hidden(Position::ATTACKER),
        goalkeeping: generate_goalkeeping(Position::ATTACKER),
    }
}

pub fn generate_defender() -> Player{
    Player{
        physical: generate_physical(Position::DEFENDER),
        mental: generate_mental(Position::DEFENDER),
        technique: generate_technique(Position::DEFENDER),
        hidden: generate_hidden(Position::DEFENDER),
        goalkeeping: generate_goalkeeping(Position::DEFENDER),
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
    };
    return PhysicalTraits{
        acceleration: 0,
        agility: 0,
        jump_reach: 0,
        natural_fitness: 0,
        pace:0,
        stamina: 0,
        strength: 0,
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

pub struct Player {
    physical: PhysicalTraits,
    mental: MentalTraits,
    technique: TechnicalTraits,
    hidden: HiddenTraits,
    goalkeeping: GoalKeepingTraits
}
pub struct GoalKeepingTraits{ 
    aerial_ability: i64,
    command_of_area: i64,
    communication: i64,
    eccentricity: i64,
    handling: i64,
    kicking: i64,
    one_on_one: i64,
    reflexes: i64,
    rushing_out: i64,
    tendency_to_punch: i64,
    throwing: i64,
}

pub struct TechnicalTraits {
    dribbling: i64,
    first_touch: i64,
    free_kick_taking: i64,
    heading: i64,
    long_shots: i64,
    long_throws: i64,
    marking: i64,
    passing: i64,
    penalty_taking: i64,
    tackling: i64,
    technique: i64,
}

pub struct PhysicalTraits {
    acceleration: i64,
    agility: i64,
    jump_reach: i64,
    natural_fitness: i64,
    pace: i64,
    stamina: i64,
    strength: i64,
}
pub struct MentalTraits {
    aggression: i64,
    bravery: i64,
    composure: i64,
    concentration: i64,
    decisions: i64,
    determination: i64,
    flair: i64,
    leadership: i64,
    off_ball: i64,
    positioning: i64,
    teamwork: i64,
    vision: i64,
    work_rate: i64,
}

pub struct HiddenTraits {
    adaptability: i64,
    consistency: i64,
    dirtiniess: i64,
    injury_proneness: i64,
    versatility: i64
}