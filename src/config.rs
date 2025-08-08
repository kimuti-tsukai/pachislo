/// Configuration for the game.
pub struct Config {
    pub balls: BallsConfig,
    /// Probability of winning a ball.
    pub probability: Probability,
}

pub struct BallsConfig {
    /// Initial number of balls in the game.
    pub init_balls: usize,
    /// Incremental balls.
    pub incremental_balls: usize,
    /// Incremental rush balls.
    pub incremental_rush: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct SlotProbability {
    pub win: f64,
    pub fake_win: f64,
    pub fake_lose: f64,
}

/// Probability of winning a ball.
#[derive(Debug, Clone, Copy)]
pub struct Probability {
    pub start_hole: f64,
    pub normal: SlotProbability,
    pub rush: SlotProbability,
    pub rush_continue: SlotProbability,
    pub rush_continue_fn: fn(usize) -> f64,
}
