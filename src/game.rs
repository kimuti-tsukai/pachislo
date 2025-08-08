use std::{error::Error, fmt::Display};

use crate::{
    config::{BallsConfig, Config, ConfigError},
    interface::{UserInput, UserOutput},
    lottery::Lottery,
};

pub struct Transition<'a> {
    pub before: Option<GameState>,
    pub after: &'a GameState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UninitializedError;

impl Display for UninitializedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UninitializedError")
    }
}

impl Error for UninitializedError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlreadyStartedError;

impl Display for AlreadyStartedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AlreadyStartedError")
    }
}

impl Error for AlreadyStartedError {}

#[derive(Clone, Copy, Debug)]
pub enum GameState {
    Uninitialized,
    Normal {
        balls: usize,
    },
    Rush {
        balls: usize,
        rush_balls: usize,
        n: usize,
    },
}

impl GameState {
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

pub struct Game<I, O> {
    state: GameState,
    lottery: Lottery,
    config: BallsConfig,
    input: I,
    output: O,
}

impl<I, O> Game<I, O>
where
    I: UserInput<O>,
    O: UserOutput,
{
    pub fn new(config: Config, input: I, output: O) -> Result<Self, ConfigError> {
        config.validate()?;
        Ok(Self {
            state: GameState::Uninitialized,
            lottery: Lottery::new(config.probability),
            config: config.balls,
            input,
            output,
        })
    }

    pub fn run(&mut self) {
        let mut before = None;
        loop {
            self.output.default(Transition {
                before,
                after: &self.state,
            });

            let Some(mut command) = self.input.wait_for_input() else {
                let _ = self.finish();
                break;
            };

            before = Some(self.state);

            command.execute(self);
        }
    }

    pub fn start(&mut self) -> Result<(), AlreadyStartedError> {
        self.state.init(&self.config)
    }

    pub fn finish(&mut self) -> Result<(), UninitializedError> {
        if self.state.is_uninitialized() {
            return Err(UninitializedError);
        }

        self.output.finish_game(&self.state);

        self.state = GameState::Uninitialized;

        Ok(())
    }

    pub fn launch_ball(&mut self) -> Result<(), UninitializedError> {
        self.state.launch_ball()?;

        if !self.lottery.start_hole() {
            return Ok(());
        }

        // When hit start hole

        let result;
        if self.state.is_rush() {
            result = self.lottery.lottery_rush();
            self.output.lottery_rush(result);
        } else {
            result = self.lottery.lottery_normal();
            self.output.lottery_normal(result);
        }

        if !(result.is_win()) {
            return Ok(());
        }

        // When win the lottery

        let GameState::Rush { n, .. } = self.state else {
            self.state.trigger_rush(&self.config);
            return Ok(());
        };

        // When rush

        let continue_lottery = self.lottery.lottery_rush_continue(n);
        self.output.lottery_rush_continue(continue_lottery);

        if continue_lottery.is_win() {
            self.state.trigger_rush(&self.config)
        } else {
            self.state.increment_balls(&self.config)
        };

        Ok(())
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn output(&self) -> &O {
        &self.output
    }
}
