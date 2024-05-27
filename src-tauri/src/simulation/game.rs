pub struct Game{
    team1_name: String,
    team2_name: String,
    date: Date,
    final_score: Score,
    events: Vec<String>,
}

pub struct Score {
    team1_score: u32,
    team2_score: u32,
}

pub struct Date {
    day: u32,
    month: u32,
    year: u32,
}