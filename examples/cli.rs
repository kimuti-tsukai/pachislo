use std::io::{self, Stdin};

use pachislo::{
    CONFIG_EXAMPLE as CONFIG, Game,
    command::{ControllCommand, FinishGame, LaunchBall, StartGame},
    game::{GameState, Transition},
    interface::{UserInput, UserOutput},
    lottery::LotteryResult,
    slot::SlotProducer,
};

fn main() {
    let input = CuiInput::new();

    let output = CuiOutput::new();

    let mut game = Game::new(CONFIG, input, output).unwrap();

    game.run();
}

pub struct CuiInput {
    stdin: Stdin,
}

impl<O: UserOutput> UserInput<O> for CuiInput {
    fn wait_for_input(&mut self) -> Option<Box<dyn ControllCommand<Self, O>>> {
        loop {
            let mut s = String::new();
            self.stdin.read_line(&mut s).ok()?;
            match s.trim() {
                "s" => return Some(Box::new(StartGame)),
                "l" | "" => return Some(Box::new(LaunchBall)),
                "q" => return Some(Box::new(FinishGame)),
                _ => (),
            }
        }
    }
}

impl CuiInput {
    pub fn new() -> Self {
        Self { stdin: io::stdin() }
    }
}

impl Default for CuiInput {
    fn default() -> Self {
        Self::new()
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
