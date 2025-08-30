# Pachislo Game Simulator

A Rust-based simulation library for Japanese pachislo (slot machine) games, featuring configurable probability systems, realistic game state management, and extensible user interfaces.

## Features

- **Comprehensive Game State Management**: Support for Uninitialized, Normal, and Rush game modes with seamless transitions
- **Advanced Lottery System**: Realistic probability-based lottery with configurable win/loss rates and fake results
- **Sophisticated Slot Result Generation**: Multi-reel slot machine simulation with customizable symbols and patterns
- **Extensible Command System**: Flexible command pattern architecture for game control and flow management
- **Trait-based Interface Architecture**: Generic input/output system supporting multiple UI implementations
- **Rush Mode Mechanics**: Advanced rush continuation system with decay probability functions
- **Ball Management System**: Comprehensive ball tracking with configurable rewards and increments

## Game Modes

### Normal Mode
- Standard pachislo gameplay with configurable base win probability
- Ball consumption and management with incremental rewards
- Automatic transition to Rush mode upon winning lottery
- Support for fake win/lose results to enhance realism

### Rush Mode
- Enhanced win probability during rush periods (default: 48% vs 16% in normal mode)
- Dynamic rush continuation system with decreasing probability over time
- Bonus ball distribution system with rush-specific increments
- Configurable rush continuation decay function

## Architecture

The simulator is built around several core components:

- **`Game<I, O>`**: Main game controller with generic input/output interfaces
- **`GameState`**: State machine managing game progression (Uninitialized → Normal → Rush)
- **`Lottery`**: Advanced probability-based system handling win/loss determination with fake results
- **`SlotProducer<T>`**: Configurable slot machine result generator supporting custom symbols
- **`LaunchBallFlowProducer`**: Manages ball launch mechanics and start hole probability
- **`Config`**: Comprehensive configuration system for all game parameters

## Quick Start

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

```rust
BallsConfig {
    init_balls: 1000,           // Starting number of balls
    incremental_balls: 15,      // Balls awarded on normal win
    incremental_rush: 300,      // Additional balls during rush mode
}
```

### Probability Configuration

```rust
Probability {
    normal: SlotProbability {
        win: 0.16,              // Base win probability
        fake_win: 0.3,          // Fake win after actual win
        fake_lose: 0.15,        // Fake lose after actual lose
    },
    rush: SlotProbability {
        win: 0.48,              // Enhanced rush win probability
        fake_win: 0.2,          // Rush fake win probability
        fake_lose: 0.05,        // Rush fake lose probability
    },
    rush_continue: SlotProbability {
        win: 0.8,               // Rush continuation base probability
        fake_win: 0.25,         // Rush continuation fake win
        fake_lose: 0.1,         // Rush continuation fake lose
    },
    // Decay function: 0.6^(n-1) where n is rush count
    rush_continue_fn: |n| 0.6f64.powi(n as i32 - 1),
}
```

## Commands

The simulator supports the following command system:

- **`StartGame`**: Initialize a new game session from uninitialized state
- **`LaunchBallFlowProducer`**: Advanced ball launching with start hole probability
- **`FinishGame`**: End the current game session gracefully
- **`Command::FinishGame`**: Force terminate the game loop

## Example: CLI Implementation

The project includes a complete CLI example demonstrating all features:

```rust
// Run the CLI example
cargo run --example cli
```

**Controls:**
- `s` - Start new game
- `l` or `Enter` - Launch ball
- `q` - Finish current game
- `q!` - Force quit

## Extending the Simulator

### Custom Input/Output Implementation

Implement the `UserInput<O>` and `UserOutput` traits:

```rust
impl UserInput<MyOutput> for MyInput {
    fn wait_for_input(&mut self) -> Vec<Command<Self, O>> {
        // Handle user input and return commands
    }
}

impl UserOutput for MyOutput {
    fn default(&mut self, state: Transition<'_>) {
        // Handle state transitions
    }
    
    fn finish_game(&mut self, state: &GameState) {
        // Handle game completion
    }
    
    fn lottery_normal(&mut self, result: LotteryResult) {
        // Display normal mode lottery results
    }
    
    fn lottery_rush(&mut self, result: LotteryResult) {
        // Display rush mode lottery results
    }
    
    fn lottery_rush_continue(&mut self, result: LotteryResult) {
        // Display rush continuation results
    }
}
```

### Custom Slot Symbols

Create slot machines with custom symbols:

```rust
let slot_producer = SlotProducer::new(3, vec!['🍒', '🍋', '🔔', '⭐']);
```

## Project Structure

```
pachislo/
├── src/
│   ├── lib.rs          # Main library exports and example config
│   ├── game.rs         # Core game logic and state management
│   ├── command.rs      # Command pattern implementation
│   ├── config.rs       # Configuration structures
│   ├── interface.rs    # User input/output traits
│   ├── lottery.rs      # Lottery probability system
│   └── slot.rs         # Slot machine result generation
├── examples/
│   └── cli.rs          # Complete CLI implementation
├── tests/              # Comprehensive test suite
└── Cargo.toml          # Project configuration
```

## Dependencies

- **`rand 0.9.1`**: High-quality random number generation for lottery and slot systems

## Development

```bash
# Run tests
cargo test

# Run CLI example
cargo run --example cli

# Build documentation
cargo doc --open

# Check code formatting
cargo fmt --check

# Run clippy linting
cargo clippy
```

## License

This project is licensed under the terms specified in the LICENSE file.

## Contributing

Contributions are welcome! Please ensure all tests pass and follow the existing code style.