#![doc = include_str!("../README.md")]

/// Command handling module for game control operations.
pub mod command;
/// Configuration module for game settings and parameters.
pub mod config;
/// Core game logic and state management.
pub mod game;
/// User input and output interface traits.
pub mod interface;
/// Lottery system for determining game outcomes.
pub mod lottery;
/// Slot machine visual representation and logic.
pub mod slot;

/// Re-export of the main Game struct for convenient access.
pub use game::Game;

use crate::config::{BallsConfig, Config, Probability, SlotProbability};

/// Example probability value for starting hole entrance.
///
/// This represents a 12% chance that a launched ball will enter the starting hole
/// and trigger a lottery event.
pub const START_HOLE_PROBABILITY_EXAMPLE: f64 = 0.12;

/// Example configuration for the pachislot game.
///
/// This provides a pre-configured setup with balanced probabilities and ball counts
/// suitable for typical gameplay. The configuration includes:
///
/// - Initial balls: 1000
/// - Ball increments for wins: 15
/// - Rush mode ball bonus: 300
/// - Normal mode win probability: 16%
/// - Rush mode win probability: 48%
/// - Rush continuation probability: 80% (with decay)
pub const CONFIG_EXAMPLE: Config = Config {
    // Config of Balls
    balls: BallsConfig {
        // Initial number of balls
        init_balls: 1000,
        // Incremental number of balls when win the lottery
        incremental_balls: 15,
        // Incremental number of balls in rush mode when become or continue rush mode
        incremental_rush: 300,
    },
    // Config of Probability
    probability: Probability {
        // Probability of lottery in normal mode
        normal: SlotProbability {
            // Probability of winning
            win: 0.16,
            // Probability of fake winning after winning
            fake_win: 0.3,
            // Probability of fake losing after losing
            fake_lose: 0.15,
        },

        // Probability of lottery in rush mode
        rush: SlotProbability {
            win: 0.48,
            fake_win: 0.2,
            fake_lose: 0.05,
        },

        // Probability of lottery of continuing rush
        rush_continue: SlotProbability {
            win: 0.8,
            fake_win: 0.25,
            fake_lose: 0.1,
        },

        // The function to calculate probability of continuing rush
        // The probability is determined by next formula
        // ```
        // a * f(n)
        // ```
        // a: probability.rush_continue.win
        // f: probability.rush_continue_fn
        // n: number of RUSH times
        //
        // This function should be return 1 when n == 1
        // This function should be monotonically non-increasing
        rush_continue_fn: |n| 0.6f64.powi(n as i32 - 1),
    },
};
