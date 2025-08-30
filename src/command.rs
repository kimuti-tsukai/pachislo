use rand::{Rng, rngs::ThreadRng};

use crate::{
    game::Game,
    interface::{UserInput, UserOutput},
};

/// Represents a command that can be executed within the game.
///
/// Commands are the primary mechanism for controlling game flow and executing actions.
/// They can either be control commands that modify game state or special commands like
/// finishing the game.
///
/// # Type Parameters
///
/// * `I` - User input handler type implementing `UserInput<O>`
/// * `O` - User output handler type implementing `UserOutput`
pub enum Command<I, O>
where
    I: UserInput<O>,
    O: UserOutput,
{
    /// Command to finish the current game session.
    FinishGame,
    /// A boxed control command that can modify game state.
    Control(Box<dyn ControlCommand<I, O>>),
}

impl<I, O> Command<I, O>
where
    I: UserInput<O>,
    O: UserOutput,
{
    /// Creates a new control command from any type implementing `ControlCommand`.
    ///
    /// This is a convenience method for wrapping control commands in the Command enum.
    ///
    /// # Arguments
    ///
    /// * `control` - The control command to wrap
    ///
    /// # Returns
    ///
    /// A `Command::Control` variant containing the boxed control command.
    pub fn control<C>(control: C) -> Self
    where
        C: ControlCommand<I, O> + 'static,
    {
        Self::Control(Box::new(control))
    }
}

/// Trait for commands that can control and modify game state.
///
/// This trait defines the interface for all game control operations. Implementing
/// types can encapsulate specific game actions and be executed through the command system.
///
/// # Type Parameters
///
/// * `I` - User input handler type implementing `UserInput<O>`
/// * `O` - User output handler type implementing `UserOutput`
pub trait ControlCommand<I, O>
where
    I: UserInput<O>,
    O: UserOutput,
{
    /// Executes the command, potentially modifying the game state.
    ///
    /// # Arguments
    ///
    /// * `game` - Mutable reference to the game instance to operate on
    fn execute(&mut self, game: &mut Game<I, O>);
}

/// Command to launch a single ball in the game.
///
/// This command decrements the ball count and may cause state transitions
/// if it's the last ball available.
pub struct LaunchBall;

impl<I, O> ControlCommand<I, O> for LaunchBall
where
    I: UserInput<O>,
    O: UserOutput,
{
    fn execute(&mut self, game: &mut Game<I, O>) {
        let _ = game.launch_ball();
    }
}

/// Command to trigger a lottery event.
///
/// This command initiates the lottery system based on the current game mode
/// (normal or rush) and handles the resulting outcomes including potential
/// rush mode transitions.
pub struct CauseLottery;

impl<I, O> ControlCommand<I, O> for CauseLottery
where
    I: UserInput<O>,
    O: UserOutput,
{
    fn execute(&mut self, game: &mut Game<I, O>) {
        game.cause_lottery();
    }
}

/// Command to start a new game session.
///
/// This command initializes the game with the configured starting ball count
/// and transitions from the uninitialized state to normal gameplay mode.
pub struct StartGame;

impl<I, O> ControlCommand<I, O> for StartGame
where
    I: UserInput<O>,
    O: UserOutput,
{
    fn execute(&mut self, game: &mut Game<I, O>) {
        let _ = game.start();
    }
}

/// Command to finish the current game session.
///
/// This command performs cleanup operations and transitions the game back
/// to the uninitialized state, making it ready for a new session.
pub struct FinishGame;

impl<I, O> ControlCommand<I, O> for FinishGame
where
    I: UserInput<O>,
    O: UserOutput,
{
    fn execute(&mut self, game: &mut Game<I, O>) {
        let _ = game.finish();
    }
}

/// Producer for generating ball launch flow commands with lottery probability.
///
/// This struct manages the probability of whether a launched ball will trigger
/// a lottery event, simulating the physical behavior of balls entering special holes.
pub struct LaunchBallFlowProducer {
    /// Probability that a launched ball will trigger a lottery (0.0 to 1.0).
    start_hole_probability: f64,
    /// Random number generator for probability calculations.
    rng: ThreadRng,
}

impl LaunchBallFlowProducer {
    /// Creates a new ball launch flow producer with the specified lottery probability.
    ///
    /// # Arguments
    ///
    /// * `start_hole_probability` - Probability (0.0 to 1.0) that a ball will trigger lottery
    ///
    /// # Returns
    ///
    /// A new `LaunchBallFlowProducer` instance.
    pub fn new(start_hole_probability: f64) -> Self {
        Self {
            start_hole_probability,
            rng: rand::rng(),
        }
    }

    /// Generates a new ball launch flow command.
    ///
    /// This method randomly determines whether the ball launch should trigger
    /// a lottery event based on the configured probability.
    ///
    /// # Returns
    ///
    /// A `LaunchBallFlow` command that may or may not include lottery activation.
    pub fn produce(&mut self) -> LaunchBallFlow {
        LaunchBallFlow::new(self.rng.random_bool(self.start_hole_probability))
    }
}

/// Command that represents a complete ball launch flow.
///
/// This command combines ball launching with optional lottery triggering,
/// simulating the complete sequence of events when a ball is launched in the game.
pub struct LaunchBallFlow {
    /// Whether this ball launch should trigger a lottery event.
    is_lottery: bool,
}

impl LaunchBallFlow {
    /// Creates a new ball launch flow command.
    ///
    /// # Arguments
    ///
    /// * `is_lottery` - Whether this ball launch should trigger a lottery event
    ///
    /// # Returns
    ///
    /// A new `LaunchBallFlow` command.
    pub fn new(is_lottery: bool) -> Self {
        Self { is_lottery }
    }
}

impl<I, O> ControlCommand<I, O> for LaunchBallFlow
where
    I: UserInput<O>,
    O: UserOutput,
{
    fn execute(&mut self, game: &mut Game<I, O>) {
        let _ = game.launch_ball();

        if self.is_lottery {
            game.cause_lottery();
        }
    }
}
