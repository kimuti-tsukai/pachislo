# Pachislo Game Simulator

A Rust-based simulation library for Japanese pachislo (slot machine) games, featuring configurable probability systems, game state management, and extensible user interfaces.

## Features

- **Game State Management**: Support for Normal and Rush game modes
- **Configurable Lottery System**: Customizable win/loss probabilities for different game states
- **Slot Result Generation**: Realistic slot machine result patterns with win/lose variations
- **Command Pattern**: Extensible command system for game control
- **Trait-based Interface**: Flexible user input/output system for different UI implementations

## Game Modes

### Normal Mode

- Standard gameplay with configurable win probability
- Ball consumption and management
- Transition to Rush mode on winning

### Rush Mode

- Enhanced win probability during rush periods
- Rush continuation system with decreasing probability over time
- Bonus ball distribution during rush

## Architecture

The simulator is built around several core components:

- **`Game<I, O>`**: Main game controller with generic input/output interfaces
- **`GameState`**: State machine managing game progression (Uninitialized → Normal → Rush)
- **`Lottery`**: Probability-based lottery system for win/loss determination
- **`SlotProducer<T>`**: Generates slot machine results based on lottery outcomes
- **`Config`**: Comprehensive configuration system for game parameters

## Usage

```rust
use pachislo::{Game, CONFIG_EXAMPLE};

// Create game with your input/output implementations
let mut game = Game::new(CONFIG_EXAMPLE, your_input, your_output).unwrap();

// Run the game loop
game.run();
```

## Configuration

The game supports extensive configuration through the `Config` struct:

```rust
pub struct Config {
    pub balls: BallsConfig,        // Ball management settings
    pub probability: Probability,  // Win/loss probabilities
}
```

### Ball Configuration

- `init_balls`: Starting number of balls
- `incremental_balls`: Balls awarded on win
- `incremental_rush`: Additional balls during rush mode

### Probability Configuration

- `start_hole`: Probability of hitting the start trigger
- `normal`: Win probabilities for normal mode
- `rush`: Enhanced probabilities during rush
- `rush_continue`: Probabilities for rush continuation
- `rush_continue_fn`: Function defining rush continuation decay

## Commands

The simulator supports the following commands:

- **`StartGame`**: Initialize a new game session
- **`LaunchBall`**: Launch a ball and execute game logic
- **`FinishGame`**: End the current game session

## Extending the Simulator

### Custom Input/Output

Implement the `UserInput<O>` and `UserOutput` traits to create custom interfaces:

```rust
impl UserInput<MyOutput> for MyInput {
    fn wait_for_input(&mut self) -> Option<Box<dyn ControllCommand<Self, MyOutput>>> {
        // Your input handling logic
    }
}

impl UserOutput for MyOutput {
    fn default(&mut self, state: Transition<'_>) { /* ... */ }
    fn finish_game(&mut self, state: &GameState) { /* ... */ }
    fn lottery_normal(&mut self, result: LotteryResult) { /* ... */ }
    fn lottery_rush(&mut self, result: LotteryResult) { /* ... */ }
    fn lottery_rush_continue(&mut self, result: LotteryResult) { /* ... */ }
}
```

### Custom Commands

Extend the command system by implementing `ControllCommand<I, O>`:

```rust
pub struct MyCommand;

impl<I, O> ControllCommand<I, O> for MyCommand
where
    I: UserInput<O>,
    O: UserOutput,
{
    fn execute(&mut self, game: &mut Game<I, O>) {
        // Your command logic
    }
}
```

## Dependencies

- `rand`: Random number generation for lottery and slot systems

## License

This project is licensed under the terms specified in the LICENSE file.
