use std::{collections::VecDeque, error::Error, fmt::Display, ops::ControlFlow};

use rand::{Rng, rngs::ThreadRng};

use crate::{
    command::Command,
    config::{BallsConfig, Config, ConfigError},
    interface::{UserInput, UserOutput},
    lottery::Lottery,
};

/// Represents a state transition in the game.
///
/// This structure captures both the previous state (if any) and the new state
/// after a game action has been executed. Used primarily for output and logging purposes.
#[derive(Debug, Clone, Copy)]
pub struct Transition {
    /// The game state before the transition occurred.
    pub before: Option<GameState>,
    /// The game state after the transition occurred.
    pub after: GameState,
}

/// Error indicating that an operation was attempted on an uninitialized game.
///
/// This error occurs when trying to perform game actions (like launching a ball)
/// before the game has been properly started.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UninitializedError;

impl Display for UninitializedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UninitializedError")
    }
}

impl Error for UninitializedError {}

/// Error indicating that an attempt was made to start a game that is already running.
///
/// This error prevents accidentally reinitializing a game that is in progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlreadyStartedError;

impl Display for AlreadyStartedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AlreadyStartedError")
    }
}

impl Error for AlreadyStartedError {}

/// Represents the current state of the pachislot game.
///
/// The game can be in one of three states:
/// - `Uninitialized`: Game has not been started yet
/// - `Normal`: Standard gameplay mode with a certain number of balls
/// - `Rush`: Special bonus mode with additional balls and continuation mechanics
#[derive(Clone, Copy, Debug)]
pub enum GameState {
    /// Game has not been initialized or has ended.
    Uninitialized,
    /// Normal gameplay mode.
    Normal {
        /// Number of balls available for play.
        balls: usize,
    },
    /// Rush (bonus) mode with enhanced winning chances.
    Rush {
        /// Total number of balls available.
        balls: usize,
        /// Number of balls specifically for rush mode play.
        rush_balls: usize,
        /// Number of consecutive rush rounds achieved.
        n: usize,
    },
}

impl GameState {
    /// Launches a ball and updates the game state accordingly.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the ball was successfully launched
    /// - `Err(UninitializedError)` if the game is not initialized
    ///
    /// # State Changes
    ///
    /// - In Normal mode: Decrements balls count, transitions to Uninitialized if no balls remain
    /// - In Rush mode: Decrements rush_balls count, transitions to Normal mode when rush_balls reach 0
    pub(crate) fn launch_ball(&mut self) -> Result<(), UninitializedError> {
        match self {
            Self::Uninitialized => Err(UninitializedError),
            Self::Normal { balls } => {
                let balls = *balls - 1;
                if balls == 0 {
                    *self = Self::Uninitialized;
                } else {
                    *self = Self::Normal { balls };
                }
                Ok(())
            }
            Self::Rush {
                balls,
                rush_balls,
                n,
            } => {
                let rush_balls = *rush_balls - 1;
                if rush_balls == 0 {
                    *self = Self::Normal { balls: *balls };
                } else {
                    *self = Self::Rush {
                        balls: *balls,
                        rush_balls,
                        n: *n,
                    };
                }
                Ok(())
            }
        }
    }

    pub(crate) fn is_uninitialized(&self) -> bool {
        matches!(self, Self::Uninitialized)
    }

    pub(crate) fn init(&mut self, config: &BallsConfig) -> Result<(), AlreadyStartedError> {
        if self.is_uninitialized() {
            *self = Self::Normal {
                balls: config.init_balls,
            };
            Ok(())
        } else {
            Err(AlreadyStartedError)
        }
    }

    pub(crate) fn increment_balls(&mut self, config: &BallsConfig) {
        match self {
            Self::Uninitialized => unreachable!(),
            Self::Normal { balls } => *balls += config.incremental_balls,
            Self::Rush { balls, .. } => {
                *balls += config.incremental_balls;
            }
        }
    }

    pub(crate) fn is_rush(&self) -> bool {
        match self {
            Self::Uninitialized => false,
            Self::Normal { .. } => false,
            Self::Rush { .. } => true,
        }
    }

    /// Into RUSH or Continue RUSH
    /// Include Incremental Balls and Rush Balls
    pub(crate) fn trigger_rush(&mut self, config: &BallsConfig) {
        match self {
            Self::Uninitialized => unreachable!(),
            Self::Normal { balls } => {
                *self = Self::Rush {
                    balls: *balls + config.incremental_balls,
                    rush_balls: config.incremental_rush,
                    n: 1,
                }
            }
            Self::Rush {
                balls,
                rush_balls,
                n,
            } => {
                *balls += config.incremental_balls;
                *rush_balls += config.incremental_rush;
                *n += 1;
            }
        }
    }
}

/// The main game controller that manages the pachislot game state and flow.
///
/// This struct orchestrates all game components including state management,
/// user input/output handling, lottery system, and command processing.
///
/// # Type Parameters
///
/// - `I`: User input handler implementing `UserInput<O>`
/// - `O`: User output handler implementing `UserOutput`
/// - `F`: Probability function type implementing `FnMut(usize) -> f64`
/// - `R`: Random number generator implementing `Rng`
pub struct Game<I, O, F: FnMut(usize) -> f64 = fn(usize) -> f64, R = ThreadRng>
where
    I: UserInput<O, F, R>,
    O: UserOutput,
    R: Rng,
{
    /// Previous game state for transition tracking.
    before_state: Option<GameState>,
    /// Current game state.
    state: GameState,
    /// Lottery system for determining outcomes.
    lottery: Lottery<F, R>,
    /// Ball-related configuration settings.
    config: BallsConfig,
    /// User input handler.
    input: I,
    /// User output handler.
    output: O,
}

impl<I, O, F, R> Game<I, O, F, R>
where
    I: UserInput<O, F, R>,
    O: UserOutput,
    F: FnMut(usize) -> f64,
    R: Rng + Default,
{
    /// Creates a new Game instance with the specified configuration and I/O handlers.
    ///
    /// # Arguments
    ///
    /// - `config`: Game configuration including probabilities and ball settings
    /// - `input`: User input handler
    /// - `output`: User output handler
    ///
    /// # Returns
    ///
    /// - `Ok(Game)` if the configuration is valid
    /// - `Err(ConfigError)` if the configuration contains invalid values
    ///
    /// # Examples
    ///
    /// ```
    /// use pachislo::{Game, CONFIG_EXAMPLE};
    /// // Assuming you have input and output handlers
    /// let game = Game::new(CONFIG_EXAMPLE, input, output)?;
    /// ```
    pub fn new(config: Config<F>, input: I, output: O) -> Result<Self, ConfigError> {
        config.validate()?;
        Ok(Self {
            before_state: None,
            state: GameState::Uninitialized,
            lottery: Lottery::new(config.probability),
            config: config.balls,
            input,
            output,
        })
    }
}

impl<I, O, F, R> Game<I, O, F, R>
where
    I: UserInput<O, F, R>,
    O: UserOutput,
    F: FnMut(usize) -> f64,
    R: Rng,
{
    /// Executes a single step of the game loop.
    ///
    /// This method processes one command from the input queue and updates the game state accordingly.
    /// It handles user input, executes commands, and manages state transitions.
    ///
    /// # Returns
    ///
    /// - `ControlFlow::Continue(())` if the game should continue running
    /// - `ControlFlow::Break(())` if the game should terminate
    pub fn run_step(&mut self) -> ControlFlow<()> {
        self.output.default(Transition {
            before: self.before_state,
            after: self.state,
        });

        let mut command = match self.input.wait_for_input() {
            Command::Control(cmd) => cmd,
            Command::FinishGame => return ControlFlow::Break(()),
        };

        self.before_state = Some(self.state);

        command.execute(self);

        ControlFlow::Continue(())
    }

    /// Runs the main game loop until termination.
    ///
    /// This method continuously calls `run_step()` until the game decides to terminate.
    /// Use this for a complete game session from start to finish.
    pub fn run(&mut self) {
        loop {
            if self.run_step().is_break() {
                break;
            }
        }
    }

    /// Starts the game by initializing it with the configured number of balls.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the game was successfully started
    /// - `Err(AlreadyStartedError)` if the game is already running
    pub fn start(&mut self) -> Result<(), AlreadyStartedError> {
        self.state.init(&self.config)
    }

    /// Finishes the current game session and resets to uninitialized state.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the game was successfully finished
    /// - `Err(UninitializedError)` if the game was not running
    pub fn finish(&mut self) -> Result<(), UninitializedError> {
        if self.state.is_uninitialized() {
            return Err(UninitializedError);
        }

        self.output.finish_game(&self.state);

        self.state = GameState::Uninitialized;

        Ok(())
    }

    /// Launches a ball in the game.
    ///
    /// This decrements the available ball count and may trigger state transitions
    /// (e.g., from Rush mode back to Normal mode when rush balls are exhausted).
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the ball was successfully launched
    /// - `Err(UninitializedError)` if the game is not running
    pub fn launch_ball(&mut self) -> Result<(), UninitializedError> {
        self.state.launch_ball()
    }

    /// Triggers a lottery event based on the current game state.
    ///
    /// The lottery behavior depends on whether the game is in Normal or Rush mode:
    /// - In Normal mode: Uses normal lottery probabilities
    /// - In Rush mode: Uses enhanced rush probabilities and handles continuation logic
    ///
    /// Winning a lottery may trigger rush mode or continue an existing rush sequence.
    pub fn cause_lottery(&mut self) {
        let result;
        if self.state.is_rush() {
            result = self.lottery.lottery_rush();
            self.output.lottery_rush(result);
        } else {
            result = self.lottery.lottery_normal();
            self.output.lottery_normal(result);
        }

        if !(result.is_win()) {
            return;
        }

        // When win the lottery

        let GameState::Rush { n, .. } = self.state else {
            self.state.trigger_rush(&self.config);
            return;
        };

        // When rush

        let continue_lottery = match self.lottery.lottery_rush_continue(n) {
            Ok(lottery) => lottery,
            Err(error) => {
                println!("Warn: {}", error);
                return;
            }
        };

        self.output.lottery_rush_continue(continue_lottery);

        if continue_lottery.is_win() {
            self.state.trigger_rush(&self.config)
        } else {
            self.state.increment_balls(&self.config)
        };
    }

    /// Returns a reference to the current game state.
    ///
    /// # Returns
    ///
    /// A reference to the current `GameState` (Uninitialized, Normal, or Rush).
    pub fn state(&self) -> &GameState {
        &self.state
    }

    /// Returns a reference to the output handler.
    ///
    /// # Returns
    ///
    /// A reference to the user output handler for external access.
    pub fn output(&self) -> &O {
        &self.output
    }
}
