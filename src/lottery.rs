use std::fmt::Display;

use rand::{Rng, rngs::ThreadRng};

use crate::config::{Probability, SlotProbability};

/// Lottery system that manages probability-based outcomes for the pachislot game.
///
/// This struct handles all lottery operations including normal mode, rush mode,
/// and rush continuation calculations. It uses a random number generator to
/// determine outcomes based on configured probabilities.
///
/// # Type Parameters
///
/// * `R` - Random number generator type implementing the `Rng` trait (defaults to `ThreadRng`)
///
/// # Examples
///
/// ```
/// use pachislo::lottery::Lottery;
/// use pachislo::config::Probability;
///
/// let probability_config = /* your probability configuration */;
/// let mut lottery = Lottery::new(probability_config);
/// let result = lottery.lottery_normal();
/// ```
pub struct Lottery<F: FnMut(usize) -> f64 = fn(usize) -> f64, R: Rng = ThreadRng> {
    rng: R,
    probability: Probability<F>,
}

/// Result of a lottery operation.
///
/// This enum represents the outcome of any lottery draw, which can be either
/// a win or a loss. Each outcome can have additional visual effects (fake results)
/// to enhance gameplay drama and suspense.
#[derive(Clone, Copy, Debug)]
pub enum LotteryResult {
    /// A winning lottery result with possible visual effects.
    Win(Win),
    /// A losing lottery result with possible visual effects.
    Lose(Lose),
}

/// Types of winning lottery outcomes.
///
/// This enum distinguishes between different types of wins that can occur,
/// affecting how the result is presented to the player.
#[derive(Clone, Copy, Debug)]
pub enum Win {
    /// A standard win with normal visual presentation.
    Default,
    /// A win that initially appears as a loss before revealing the actual win.
    /// Creates suspense and excitement for the player.
    FakeWin,
}

/// Types of losing lottery outcomes.
///
/// This enum distinguishes between different types of losses that can occur,
/// affecting how the result is presented to the player.
#[derive(Clone, Copy, Debug)]
pub enum Lose {
    /// A standard loss with normal visual presentation.
    Default,
    /// A loss that initially appears as a win before revealing the actual loss.
    /// Creates false hope and dramatic tension.
    FakeLose,
}

impl LotteryResult {
    /// Checks if this lottery result represents a win.
    ///
    /// # Returns
    ///
    /// `true` if the result is any type of win (Default or FakeWin), `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use pachislo::lottery::{LotteryResult, Win};
    ///
    /// let win_result = LotteryResult::Win(Win::Default);
    /// assert!(win_result.is_win());
    /// ```
    pub fn is_win(&self) -> bool {
        matches!(self, LotteryResult::Win(_))
    }
}

impl<F: FnMut(usize) -> f64, R: Rng + Default> Lottery<F, R> {
    /// Creates a new Lottery instance with default random number generator.
    ///
    /// # Arguments
    ///
    /// * `probability` - Probability configuration for all lottery modes
    ///
    /// # Returns
    ///
    /// A new `Lottery` instance ready for use.
    ///
    /// # Examples
    ///
    /// ```
    /// use pachislo::lottery::Lottery;
    /// use pachislo::CONFIG_EXAMPLE;
    ///
    /// let lottery = Lottery::new(CONFIG_EXAMPLE.probability);
    /// ```
    pub fn new(probability: Probability<F>) -> Self {
        Self {
            rng: R::default(),
            probability,
        }
    }
}

impl<F: FnMut(usize) -> f64, R: Rng> Lottery<F, R> {
    /// Creates a new Lottery instance with a custom random number generator.
    ///
    /// This method is useful for testing or when you need deterministic behavior
    /// by providing a seeded random number generator.
    ///
    /// # Arguments
    ///
    /// * `probability` - Probability configuration for all lottery modes
    /// * `rng` - Custom random number generator implementing `Rng`
    ///
    /// # Returns
    ///
    /// A new `Lottery` instance using the provided RNG.
    pub fn with_rng(probability: Probability<F>, rng: R) -> Self {
        Self { rng, probability }
    }

    /// Performs a lottery draw with the specified probability configuration.
    ///
    /// This is the core lottery method that determines outcomes based on
    /// win probability and visual effect probabilities.
    ///
    /// # Arguments
    ///
    /// * `probability` - Slot probability configuration containing win, fake_win, and fake_lose rates
    ///
    /// # Returns
    ///
    /// A `LotteryResult` indicating the outcome and any visual effects.
    ///
    /// # Algorithm
    ///
    /// 1. First determines if the outcome is a win or loss based on `probability.win`
    /// 2. If win: applies `probability.fake_win` chance for FakeWin effect
    /// 3. If loss: applies `probability.fake_lose` chance for FakeLose effect
    pub fn lottery(&mut self, probability: SlotProbability) -> LotteryResult {
        let SlotProbability {
            win,
            fake_win,
            fake_lose,
        } = probability;

        match self.rng.random_bool(win) {
            true => match self.rng.random_bool(fake_win) {
                true => LotteryResult::Win(Win::FakeWin),
                false => LotteryResult::Win(Win::Default),
            },
            false => match self.rng.random_bool(fake_lose) {
                true => LotteryResult::Lose(Lose::FakeLose),
                false => LotteryResult::Lose(Lose::Default),
            },
        }
    }

    /// Performs a lottery draw using normal mode probabilities.
    ///
    /// This method uses the probability configuration for standard gameplay mode.
    ///
    /// # Returns
    ///
    /// A `LotteryResult` based on normal mode probability settings.
    pub fn lottery_normal(&mut self) -> LotteryResult {
        self.lottery(self.probability.normal)
    }

    /// Performs a lottery draw using rush mode probabilities.
    ///
    /// This method uses enhanced probability configuration for rush (bonus) mode,
    /// typically offering better winning chances than normal mode.
    ///
    /// # Returns
    ///
    /// A `LotteryResult` based on rush mode probability settings.
    pub fn lottery_rush(&mut self) -> LotteryResult {
        self.lottery(self.probability.rush)
    }

    /// Performs a lottery draw to determine rush mode continuation.
    ///
    /// This method calculates whether the current rush sequence should continue
    /// based on the number of consecutive rush rounds. The probability decreases
    /// with each consecutive rush using the configured `rush_continue_fn`.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of consecutive rush rounds (starting from 1)
    ///
    /// # Returns
    ///
    /// * `Ok(LotteryResult)` - The lottery result for rush continuation
    /// * `Err(ProbabilityError)` - If the calculated probability exceeds 1.0
    ///
    /// # Examples
    ///
    /// ```
    /// use pachislo::lottery::Lottery;
    /// use pachislo::CONFIG_EXAMPLE;
    ///
    /// let mut lottery = Lottery::new(CONFIG_EXAMPLE.probability);
    /// let result = lottery.lottery_rush_continue(3); // 3rd consecutive rush
    /// ```
    pub fn lottery_rush_continue(&mut self, n: usize) -> Result<LotteryResult, ProbabilityError> {
        let mut probability = self.probability.rush_continue;

        probability.win *= (self.probability.rush_continue_fn)(n);

        if probability.win > 1.0 {
            return Err(ProbabilityError);
        }

        Ok(self.lottery(probability))
    }
}

/// Error indicating that a probability calculation resulted in an invalid value.
///
/// This error occurs when probability calculations exceed the valid range of 0.0 to 1.0,
/// most commonly in the `lottery_rush_continue` function when the `rush_continue_fn`
/// returns a value that causes the final probability to exceed 1.0.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ProbabilityError;

impl Display for ProbabilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid probability value\nIf it causes in `lottery_rush_continue` function, `Config.probability.rush_continue_fn` may return a value outside the range [0.0, 1.0]"
        )
    }
}
