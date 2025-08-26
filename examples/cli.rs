use std::io::{self, Stdin};

use pachislo::{
    CONFIG_EXAMPLE as CONFIG, Game, START_HOLE_PROBABILITY_EXAMPLE,
    command::{CauseLottery, Command, FinishGame, LaunchBall, StartGame},
    game::{GameState, Transition},
    interface::{UserInput, UserOutput},
    lottery::LotteryResult,
    slot::SlotProducer,
};
use rand::{Rng, rngs::ThreadRng};

fn main() {
    let input = CuiInput::new(START_HOLE_PROBABILITY_EXAMPLE);

    let output = CuiOutput::new();

    let mut game = Game::new(CONFIG, input, output).unwrap();

    game.run();
}

pub struct CuiInput {
    start_hole_probability: f64,
    rng: ThreadRng,
    stdin: Stdin,
}

impl<O: UserOutput> UserInput<O> for CuiInput {
    fn wait_for_input(&mut self) -> Vec<Command<Self, O>> {
        loop {
            let mut s = String::new();
            self.stdin.read_line(&mut s).ok();
            match s.trim() {
                "s" => return vec![Command::Control(Box::new(StartGame))],
                "l" | "" => {
                    return if self.rng.random_bool(self.start_hole_probability) {
                        vec![
                            Command::Control(Box::new(LaunchBall)),
                            Command::Control(Box::new(CauseLottery)),
                        ]
                    } else {
                        vec![Command::Control(Box::new(LaunchBall))]
                    };
                }
                "q" => return vec![Command::Control(Box::new(FinishGame))],
                "q!" => return vec![Command::FinishGame],
                _ => (),
            }
        }
    }
}

impl CuiInput {
    pub fn new(start_hole_probability: f64) -> Self {
        assert!((0.0..=1.0).contains(&start_hole_probability));
        Self {
            start_hole_probability,
            rng: rand::rng(),
            stdin: io::stdin(),
        }
    }
}

pub struct CuiOutput {
    slot_producer: SlotProducer<u8>,
}

impl UserOutput for CuiOutput {
    fn default(&mut self, state: Transition<'_>) {
        let Transition {
            before,
            after: state,
        } = state;

        match (state, before) {
            (GameState::Uninitialized, _) => {
                println!("Welcome to Pachislo!");
                println!();
                return;
            }
            (GameState::Normal { .. }, Some(GameState::Rush { n, .. })) => {
                println!("RUSH finished!, Number of RUSH times: {n}")
            }
            _ => {}
        }

        println!("Current state: {state:?}");
        println!();
    }

    fn finish_game(&mut self, state: &GameState) {
        println!("Game finished!");
        println!("Final state: {state:?}");
    }

    fn lottery_normal(&mut self, result: LotteryResult) {
        let slot = self.slot_producer.produce(&result);
        Self::print_slot(slot);
        println!("Lottery result: {result:?}");
    }

    fn lottery_rush(&mut self, result: LotteryResult) {
        let slot = self.slot_producer.produce(&result);
        Self::print_slot(slot);
        println!("Lottery result in rush mode: {result:?}");
    }

    fn lottery_rush_continue(&mut self, result: LotteryResult) {
        let slot = self.slot_producer.produce(&result);
        Self::print_slot(slot);
        println!("Lottery result in rush continue: {result:?}");
    }
}

impl CuiOutput {
    pub fn new() -> Self {
        Self {
            slot_producer: SlotProducer::new(3, (1..=9).collect()),
        }
    }

    pub fn print_slot(slot: (Vec<u8>, Option<Vec<u8>>)) {
        println!("Slot: {:?}", slot.0);

        if let Some(but) = slot.1 {
            println!("But: {but:?}");
        }
    }
}

impl Default for CuiOutput {
    fn default() -> Self {
        Self::new()
    }
}
