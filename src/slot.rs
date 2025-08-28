use rand::{
    Rng,
    rngs::ThreadRng,
    seq::{IndexedRandom, SliceRandom},
};

use crate::lottery::{Lose, LotteryResult, Win};

pub struct SlotProducer<T, R: Rng = ThreadRng> {
    length: usize,
    choices: Vec<T>,
    rng: R,
}

impl<T, R: Rng + Default> SlotProducer<T, R> {
    pub fn new(length: usize, choices: Vec<T>) -> Self {
        assert!(choices.len() > 1, "Choices must have at least two elements");
        Self {
            length,
            choices,
            rng: R::default(),
        }
    }
}

impl<T: Clone, R: Rng> SlotProducer<T, R> {
    pub fn with_rng(length: usize, choices: Vec<T>, rng: R) -> Self {
        Self {
            length,
            choices,
            rng,
        }
    }

    pub fn produce_win(&mut self) -> Vec<T> {
        let choice = self.choices.choose(&mut self.rng).unwrap();
        vec![choice.clone(); self.length]
    }

    pub fn produce_lose(&mut self) -> Vec<T> {
        // Vector with reference choices
        let mut ref_choices: Vec<&T> = self.choices.iter().collect();
        ref_choices.shuffle(&mut self.rng);

        // The index to divide into two vectors having at least one element each
        let partition = self.rng.random_range(1..ref_choices.len());

        // Divide into two vectors having at least one element each
        let (choices1, choices2) = ref_choices.split_at(partition);

        // The number of elements to take from each vector (cnt1 + cnt2 == self.length)
        let cnt1 = self.rng.random_range(1..self.length);
        let cnt2 = self.length - cnt1;

        let mut result1 = Vec::with_capacity(cnt1);

        // Fill result1 with cnt1 elements from choices1
        for _ in 0..cnt1 {
            result1.push(*choices1.choose(&mut self.rng).unwrap());
        }

        let mut result2 = Vec::with_capacity(cnt2);

        // Fill result2 with cnt2 elements from choices2
        for _ in 0..cnt2 {
            result2.push(*choices2.choose(&mut self.rng).unwrap());
        }

        // Integrate result1 and result2 into a single vector
        let mut result: Vec<T> = result1.into_iter().chain(result2).cloned().collect();

        // Shuffle the integrated vector
        result.shuffle(&mut self.rng);

        result
    }

    pub fn produce(&mut self, result: &LotteryResult) -> (Vec<T>, Option<Vec<T>>) {
        match result {
            LotteryResult::Win(Win::Default) => (self.produce_win(), None),
            LotteryResult::Win(Win::FakeWin) => (self.produce_lose(), Some(self.produce_win())),
            LotteryResult::Lose(Lose::Default) => (self.produce_lose(), None),
            LotteryResult::Lose(Lose::FakeLose) => (self.produce_win(), Some(self.produce_lose())),
        }
    }
}
