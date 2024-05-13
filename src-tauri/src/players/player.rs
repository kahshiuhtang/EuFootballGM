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