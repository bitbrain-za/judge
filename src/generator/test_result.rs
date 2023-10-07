use scoreboard_db::Score;

#[derive(Clone)]
pub enum TestResult {
    Success(Score),
    Fail(String),
    Stolen(Score, Score),
}
