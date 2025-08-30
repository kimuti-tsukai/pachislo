use std::{error::Error, fmt::Display};

/// Main configuration structure for the pachislot game.
///
/// This structure contains all the necessary settings to configure game behavior,
/// including ball counts, probabilities, and game mechanics.
///
/// # Examples
///
/// ```
/// use pachislo::config::{Config, BallsConfig, Probability, SlotProbability};
///
/// let config = Config {
///     balls: BallsConfig {
///         init_balls: 1000,
///         incremental_balls: 15,
///         incremental_rush: 300,
///     },
///     probability: Probability {
///         normal: SlotProbability { win: 0.16, fake_win: 0.3, fake_lose: 0.15 },
///         rush: SlotProbability { win: 0.48, fake_win: 0.2, fake_lose: 0.05 },
///         rush_continue: SlotProbability { win: 0.8, fake_win: 0.25, fake_lose: 0.1 },
///         rush_continue_fn: |n| 0.6_f64.powi(n as i32 - 1),
///     },
/// };
/// ```
pub struct Config<F: FnMut(usize) -> f64 = fn(usize) -> f64> {
    /// Configuration for ball counts and increments.
    pub balls: BallsConfig,
    /// Configuration for lottery probabilities in different game modes.
    pub probability: Probability<F>,
}

/// Configuration for ball counts and increments in the game.
///
/// This structure defines how many balls the player starts with and how many
/// they receive for various game events.
pub struct BallsConfig {
    /// Initial number of balls when starting a new game.
    ///
    /// Must be greater than 0 for the game to function properly.
    pub init_balls: usize,
    /// Number of balls awarded when winning a lottery.
    ///
    /// This applies to both normal and rush mode lottery wins.
    pub incremental_balls: usize,
    /// Number of additional balls granted when entering or continuing rush mode.
    ///
    /// These are special "rush balls" that are consumed during rush mode play.
    pub incremental_rush: usize,
}

/// Probability configuration for slot machine outcomes.
///
/// This structure defines the probabilities for different types of lottery results,
/// including real wins/losses and fake (visual effect) outcomes.
#[derive(Debug, Clone, Copy)]
pub struct SlotProbability {
    /// Base probability of winning (0.0 to 1.0).
    ///
    /// This is the fundamental chance of a positive lottery outcome.
    pub win: f64,
    /// Probability of showing a fake win animation after a real win (0.0 to 1.0).
    ///
    /// Creates suspense by initially showing a losing result before revealing the actual win.
    pub fake_win: f64,
    /// Probability of showing a fake lose animation after a real loss (0.0 to 1.0).
    ///
    /// Creates false hope by initially showing a winning result before revealing the actual loss.
    pub fake_lose: f64,
}

/// Comprehensive probability configuration for all game modes.
///
/// This structure contains probability settings for different game states and the
/// mathematical function that controls rush mode continuation decay.
#[derive(Debug, Clone, Copy)]
pub struct Probability<F: FnMut(usize) -> f64 = fn(usize) -> f64> {
    /// Probability settings for normal (standard) game mode.
    pub normal: SlotProbability,
    /// Probability settings for rush (bonus) game mode.
    ///
    /// Typically has higher win rates than normal mode.
    pub rush: SlotProbability,
    /// Base probability settings for rush mode continuation.
    ///
    /// These probabilities are modified by `rush_continue_fn` based on the number of consecutive rushes.
    pub rush_continue: SlotProbability,
    /// Function that calculates the multiplier for rush continuation probability.
    ///
    /// Takes the number of consecutive rush rounds (n) and returns a multiplier (0.0 to 1.0)
    /// that is applied to `rush_continue.win`. Should return 1.0 when n=1 and be
    /// monotonically non-increasing to create diminishing returns.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of consecutive rush rounds (starting from 1)
    ///
    /// # Returns
    ///
    /// A multiplier value that should be between 0.0 and 1.0.
    pub rush_continue_fn: F,
}

/// Error type for configuration validation failures.
///
/// This error accumulates all validation issues found in a configuration,
/// allowing users to see all problems at once rather than fixing them one by one.
#[derive(Debug, Clone, Default)]
pub struct ConfigError {
    /// List of error messages describing validation failures.
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

impl<F: FnMut(usize) -> f64> Config<F> {
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

impl<F: FnMut(usize) -> f64> Probability<F> {
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
