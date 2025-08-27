use std::{error::Error, fmt::Display};

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
    pub normal: SlotProbability,
    pub rush: SlotProbability,
    pub rush_continue: SlotProbability,
    pub rush_continue_fn: fn(usize) -> f64,
}

#[derive(Debug, Clone, Default)]
pub struct ConfigError {
    errors: Vec<String>,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConfigError: {}", self.errors.join("\n"))
    }
}

impl Error for ConfigError {}

impl ConfigError {
    pub(crate) fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub(crate) fn append(&mut self, error: &mut ConfigError) {
        self.errors.append(&mut error.errors);
    }

    pub(crate) fn push(&mut self, error: String) {
        self.errors.push(error);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}

impl Config {
    pub(crate) fn validate(&self) -> Result<(), ConfigError> {
        let mut error = ConfigError::new();
        if let Err(mut err) = self.balls.validate() {
            error.append(&mut err);
        }
        if let Err(mut err) = self.probability.validate() {
            error.append(&mut err);
        }
        if error.is_empty() { Ok(()) } else { Err(error) }
    }
}

impl BallsConfig {
    pub(crate) fn validate(&self) -> Result<(), ConfigError> {
        let mut error = ConfigError::new();
        if self.init_balls < 1 {
            error.push("initial balls must be greater than 0".to_string());
        }
        if error.is_empty() { Ok(()) } else { Err(error) }
    }
}

impl Probability {
    pub(crate) fn validate(&self) -> Result<(), ConfigError> {
        let mut error = ConfigError::new();
        if let Err(mut err) = self.normal.validate() {
            error.append(&mut err);
        }
        if let Err(mut err) = self.rush.validate() {
            error.append(&mut err);
        }
        if let Err(mut err) = self.rush_continue.validate() {
            error.append(&mut err);
        }
        if error.is_empty() { Ok(()) } else { Err(error) }
    }
}

impl SlotProbability {
    pub(crate) fn validate(&self) -> Result<(), ConfigError> {
        let mut error = ConfigError::new();
        if self.win < 0.0 || self.win > 1.0 {
            error.push("win probability must be between 0.0 and 1.0".to_string());
        }
        if self.fake_win < 0.0 || self.fake_win > 1.0 {
            error.push("fake_win probability must be between 0.0 and 1.0".to_string());
        }
        if self.fake_lose < 0.0 || self.fake_lose > 1.0 {
            error.push("fake_lose probability must be between 0.0 and 1.0".to_string());
        }
        if error.is_empty() { Ok(()) } else { Err(error) }
    }
}
