use std::fmt::Display;

use rand::{Rng, rngs::ThreadRng};

use crate::config::{Probability, SlotProbability};

pub struct Lottery<R: Rng = ThreadRng> {
    rng: R,
    probability: Probability,
}

#[derive(Clone, Copy, Debug)]
pub enum LotteryResult {
    Win(Win),
    Lose(Lose),
}

#[derive(Clone, Copy, Debug)]
pub enum Win {
    Default,
    FakeWin,
}

#[derive(Clone, Copy, Debug)]
pub enum Lose {
    Default,
    FakeLose,
}

impl LotteryResult {
    pub fn is_win(&self) -> bool {
        matches!(self, LotteryResult::Win(_))
    }
}

impl<R: Rng + Default> Lottery<R> {
    pub fn new(probability: Probability) -> Self {
        Self {
            rng: R::default(),
            probability,
        }
    }
}

impl<R: Rng> Lottery<R> {
    pub fn with_rng(probability: Probability, rng: R) -> Self {
        Self { rng, probability }
    }

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

    pub fn lottery_normal(&mut self) -> LotteryResult {
        self.lottery(self.probability.normal)
    }

    pub fn lottery_rush(&mut self) -> LotteryResult {
        self.lottery(self.probability.rush)
    }

    pub fn lottery_rush_continue(&mut self, n: usize) -> Result<LotteryResult, ProbabilityError> {
        let mut probability = self.probability.rush_continue;

        probability.win *= (self.probability.rush_continue_fn)(n);

        if probability.win > 1.0 {
            return Err(ProbabilityError);
        }

        Ok(self.lottery(probability))
    }
}

/// Error type for invalid probability values.
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
