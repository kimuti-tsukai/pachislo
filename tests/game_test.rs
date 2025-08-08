use pachislo::{
    CONFIG_EXAMPLE as CONFIG,
    command::{ControlCommand, LaunchBall, StartGame},
    game::{Game, GameState, Transition},
    interface::{UserInput, UserOutput},
    lottery::LotteryResult,
};

struct TestInput {
    n: usize,
    first: bool,
}

impl TestInput {
    fn new(n: usize) -> Self {
        TestInput { n, first: true }
    }
}

impl UserInput<TestOutput> for TestInput {
    fn wait_for_input(&mut self) -> Option<Box<dyn ControlCommand<Self, TestOutput>>> {
        if self.first {
            self.first = false;
            Some(Box::new(StartGame))
        } else if self.n > 0 {
            self.n -= 1;
            Some(Box::new(LaunchBall))
        } else {
            None
        }
    }
}

struct TestOutput {
    win_normal: usize,
    win_rush: usize,
    win_rush_continue: usize,
    max_continue: usize,
}

impl UserOutput for TestOutput {
    fn default(&mut self, state: Transition<'_>) {
        let Transition {
            before,
            after: state,
        } = state;

        if let (GameState::Normal { .. }, Some(GameState::Rush { n, .. })) = (state, before)
            && n > self.max_continue
        {
            self.max_continue = n;
        }
    }

    fn finish_game(&mut self, _state: &GameState) {}

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
        }
    }
}

#[test]
fn test() {
    let input = TestInput::new(100000000);

    let output = TestOutput::new();

    let mut game = Game::new(CONFIG, input, output).unwrap();

    game.run();

    println!("Win normal: {}", game.output().win_normal);
    println!("Win rush: {}", game.output().win_rush);
    println!("Win rush continue: {}", game.output().win_rush_continue);
    println!(
        "Total: {}",
        game.output().win_normal + game.output().win_rush + game.output().win_rush_continue
    );
    println!("Max continue: {}", game.output().max_continue);
    println!(
        "Final balls: {}",
        match game.state() {
            GameState::Uninitialized => 0,
            GameState::Normal { balls } => *balls,
            GameState::Rush { balls, .. } => *balls,
        }
    )
}
