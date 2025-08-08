use rand::{rngs::ThreadRng, Rng};

use crate::config::{Probability, SlotProbability};

pub struct Lottery {
    rng: ThreadRng,
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

impl Lottery {
    pub fn new(probability: Probability) -> Self {
        Self {
            rng: ThreadRng::default(),
            probability,
        }
    }

    pub fn start_hole(&mut self) -> bool {
        self.rng.random_bool(self.probability.start_hole)
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

    pub fn lottery_rush_continue(&mut self, n: usize) -> LotteryResult {
        let mut probability = self.probability.rush_continue;

        probability.win *= (self.probability.rush_continue_fn)(n);

        self.lottery(probability)
    }
}