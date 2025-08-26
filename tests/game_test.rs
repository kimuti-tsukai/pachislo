use std::{
    sync::{Arc, Mutex},
    thread,
};

use pachislo::{
    CONFIG_EXAMPLE as CONFIG, START_HOLE_PROBABILITY_EXAMPLE,
    command::{CauseLottery, Command, LaunchBall, StartGame},
    game::{Game, GameState, Transition},
    interface::{UserInput, UserOutput},
    lottery::LotteryResult,
};
use rand::{Rng, rngs::ThreadRng};

struct TestInput {
    n: usize,
    start_hole_probability: f64,
    rng: ThreadRng,
    first: bool,
}

impl TestInput {
    fn new(n: usize, start_hole_probability: f64) -> Self {
        assert!((0.0..=1.0).contains(&start_hole_probability));
        TestInput {
            n,
            start_hole_probability,
            rng: rand::rng(),
            first: true,
        }
    }
}

impl UserInput<TestOutput> for TestInput {
    fn wait_for_input(&mut self) -> Vec<Command<Self, TestOutput>> {
        if self.first {
            self.first = false;
            vec![Command::Control(Box::new(StartGame))]
        } else if self.n > 0 {
            self.n -= 1;
            if self.rng.random_bool(self.start_hole_probability) {
                vec![
                    Command::Control(Box::new(LaunchBall)),
                    Command::Control(Box::new(CauseLottery)),
                ]
            } else {
                vec![Command::Control(Box::new(LaunchBall))]
            }
        } else {
            vec![Command::FinishGame]
        }
    }
}

struct TestOutput {
    win_normal: usize,
    win_rush: usize,
    win_rush_continue: usize,
    max_continue: usize,
    continue_count: usize,
    continue_sum: usize,
    final_state: Vec<GameState>,
}

impl UserOutput for TestOutput {
    fn default(&mut self, state: Transition<'_>) {
        let Transition {
            before,
            after: state,
        } = state;

        if let (GameState::Normal { .. }, Some(GameState::Rush { n, .. })) = (state, before) {
            self.continue_count += 1;
            self.continue_sum += n;
            if n > self.max_continue {
                self.max_continue = n;
            }
        }
    }

    fn finish_game(&mut self, state: &GameState) {
        self.final_state[0] = *state;
    }

    fn lottery_normal(&mut self, result: LotteryResult) {
        if result.is_win() {
            self.win_normal += 1;
        }
    }

    fn lottery_rush(&mut self, result: LotteryResult) {
        if result.is_win() {
            self.win_rush += 1;
        }
    }

    fn lottery_rush_continue(&mut self, result: LotteryResult) {
        if result.is_win() {
            self.win_rush_continue += 1;
        }
    }
}

impl TestOutput {
    fn new() -> Self {
        Self {
            win_normal: 0,
            win_rush: 0,
            win_rush_continue: 0,
            max_continue: 0,
            continue_count: 0,
            continue_sum: 0,
            final_state: vec![GameState::Uninitialized],
        }
    }

    fn add(&mut self, other: &TestOutput) {
        self.win_normal += other.win_normal;
        self.win_rush += other.win_rush;
        self.win_rush_continue += other.win_rush_continue;
        self.max_continue = self.max_continue.max(other.max_continue);
        self.continue_count += other.continue_count;
        self.continue_sum += other.continue_sum;
        self.final_state.push(other.final_state[0]);
    }
}

#[test]
fn test() {
    thread::scope(|s| {
        let mut handles = Vec::with_capacity(8);

        let global_output = Arc::new(Mutex::new(TestOutput::new()));

        for _ in 0..8 {
            let global_output = Arc::clone(&global_output);

            let handle = s.spawn(move || {
                let input = TestInput::new(20000000, START_HOLE_PROBABILITY_EXAMPLE);

                let output = TestOutput::new();

                let mut game = Game::new(CONFIG, input, output).unwrap();

                game.run();

                global_output.lock().unwrap().add(game.output());
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let output = global_output.lock().unwrap();

        println!("Win normal: {}", output.win_normal);
        println!("Win rush: {}", output.win_rush);
        println!("Win rush continue: {}", output.win_rush_continue);
        println!(
            "Total: {}",
            output.win_normal + output.win_rush + output.win_rush_continue
        );
        println!("Continue count: {}", output.continue_count);
        println!(
            "Average continue: {}",
            output.continue_sum as f64 / output.continue_count as f64
        );
        println!("Max continue: {}", output.max_continue);
        println!("Final state: {:#?}", output.final_state);
    });
}
