use rand::{
    Rng,
    rngs::ThreadRng,
    seq::{IndexedRandom, SliceRandom},
};

use crate::lottery::{Lose, LotteryResult, Win};

/// Produces visual slot machine sequences based on lottery outcomes.
///
/// This struct generates slot machine symbol sequences that visually represent
/// lottery results. It can produce both winning sequences (all symbols match)
/// and losing sequences (mixed symbols), with support for fake outcomes that
/// create dramatic visual effects.
///
/// # Type Parameters
///
/// * `T` - The type of symbols used in the slot machine (must implement `Clone`)
/// * `R` - Random number generator type implementing `Rng` (defaults to `ThreadRng`)
///
/// # Examples
///
/// ```
/// use pachislo::slot::SlotProducer;
///
/// let symbols = vec!["🍒", "🍋", "🔔", "⭐"];
/// let mut producer = SlotProducer::new(3, symbols);
/// let winning_sequence = producer.produce_win();  // e.g., ["🍒", "🍒", "🍒"]
/// let losing_sequence = producer.produce_lose();  // e.g., ["🍒", "🍋", "🔔"]
/// ```
pub struct SlotProducer<T, R: Rng = ThreadRng> {
    /// Number of symbols in each generated sequence.
    length: usize,
    /// Available symbols that can appear in the slot machine.
    choices: Vec<T>,
    /// Random number generator for symbol selection.
    rng: R,
}

impl<T, R: Rng + Default> SlotProducer<T, R> {
    /// Creates a new SlotProducer with a default random number generator.
    ///
    /// # Arguments
    ///
    /// * `length` - Number of symbols in each generated sequence
    /// * `choices` - Vector of available symbols (must have at least 2 elements)
    ///
    /// # Returns
    ///
    /// A new `SlotProducer` instance ready to generate sequences.
    ///
    /// # Panics
    ///
    /// Panics if `choices` has fewer than 2 elements, as this would make
    /// losing sequences impossible to generate.
    ///
    /// # Examples
    ///
    /// ```
    /// use pachislo::slot::SlotProducer;
    ///
    /// let symbols = vec!["🍒", "🍋", "🔔"];
    /// let producer = SlotProducer::new(3, symbols);
    /// ```
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
    /// Creates a new SlotProducer with a custom random number generator.
    ///
    /// This constructor is useful for testing or when deterministic behavior
    /// is required by providing a seeded random number generator.
    ///
    /// # Arguments
    ///
    /// * `length` - Number of symbols in each generated sequence
    /// * `choices` - Vector of available symbols
    /// * `rng` - Custom random number generator implementing `Rng`
    ///
    /// # Returns
    ///
    /// A new `SlotProducer` instance using the provided RNG.
    pub fn with_rng(length: usize, choices: Vec<T>, rng: R) -> Self {
        Self {
            length,
            choices,
            rng,
        }
    }

    /// Generates a winning slot machine sequence.
    ///
    /// Creates a sequence where all symbols are identical, representing
    /// a visual win condition. The specific symbol is chosen randomly
    /// from the available choices.
    ///
    /// # Returns
    ///
    /// A vector containing `length` identical symbols.
    ///
    /// # Examples
    ///
    /// ```
    /// use pachislo::slot::SlotProducer;
    ///
    /// let mut producer = SlotProducer::new(3, vec!["A", "B", "C"]);
    /// let winning = producer.produce_win();
    /// // Result might be ["A", "A", "A"] or ["B", "B", "B"] or ["C", "C", "C"]
    /// assert_eq!(winning.len(), 3);
    /// assert!(winning.windows(2).all(|w| w[0] == w[1])); // All symbols identical
    /// ```
    pub fn produce_win(&mut self) -> Vec<T> {
        let choice = self.choices.choose(&mut self.rng).unwrap();
        vec![choice.clone(); self.length]
    }

    /// Generates a losing slot machine sequence.
    ///
    /// Creates a sequence with mixed symbols that represents a visual loss.
    /// The algorithm ensures at least two different symbols appear in the
    /// sequence by partitioning the available choices and selecting from
    /// different groups.
    ///
    /// # Returns
    ///
    /// A vector containing `length` mixed symbols that cannot form a winning pattern.
    ///
    /// # Examples
    ///
    /// ```
    /// use pachislo::slot::SlotProducer;
    ///
    /// let mut producer = SlotProducer::new(3, vec!["A", "B", "C"]);
    /// let losing = producer.produce_lose();
    /// // Result might be ["A", "B", "C"] or ["B", "A", "C"] etc.
    /// assert_eq!(losing.len(), 3);
    /// // At least two different symbols should be present
    /// let unique_count = losing.iter().collect::<std::collections::HashSet<_>>().len();
    /// assert!(unique_count >= 2);
    /// ```
    ///
    /// # Algorithm
    ///
    /// 1. Randomly shuffle all available symbol choices
    /// 2. Partition choices into two non-empty groups
    /// 3. Distribute the sequence length between the two groups
    /// 4. Fill positions with symbols from respective groups
    /// 5. Shuffle the final sequence to randomize positions
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

    /// Generates slot machine sequences based on a lottery result.
    ///
    /// This method produces one or two symbol sequences depending on the
    /// lottery outcome. Fake results generate two sequences: the initial
    /// fake sequence and the subsequent real sequence for dramatic effect.
    ///
    /// # Arguments
    ///
    /// * `result` - The lottery result determining what sequences to generate
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - First sequence: The primary visual sequence shown to the player
    /// - Optional second sequence: Additional sequence for fake result reveals
    ///
    /// # Sequence Generation Logic
    ///
    /// - `Win::Default`: Returns (winning_sequence, None)
    /// - `Win::FakeWin`: Returns (losing_sequence, Some(winning_sequence))
    /// - `Lose::Default`: Returns (losing_sequence, None)
    /// - `Lose::FakeLose`: Returns (winning_sequence, Some(losing_sequence))
    ///
    /// # Examples
    ///
    /// ```
    /// use pachislo::slot::SlotProducer;
    /// use pachislo::lottery::{LotteryResult, Win};
    ///
    /// let mut producer = SlotProducer::new(3, vec!["A", "B", "C"]);
    /// let result = LotteryResult::Win(Win::FakeWin);
    /// let (first, second) = producer.produce(&result);
    ///
    /// // FakeWin shows losing sequence first, then winning sequence
    /// assert!(second.is_some());
    /// ```
    pub fn produce(&mut self, result: &LotteryResult) -> (Vec<T>, Option<Vec<T>>) {
        match result {
            LotteryResult::Win(Win::Default) => (self.produce_win(), None),
            LotteryResult::Win(Win::FakeWin) => (self.produce_lose(), Some(self.produce_win())),
            LotteryResult::Lose(Lose::Default) => (self.produce_lose(), None),
            LotteryResult::Lose(Lose::FakeLose) => (self.produce_win(), Some(self.produce_lose())),
        }
    }
}
