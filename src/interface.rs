use rand::{Rng, rngs::ThreadRng};

use crate::{
    command::Command,
    game::{GameState, Transition},
    lottery::LotteryResult,
};

/// Trait for handling user input in the pachislot game.
///
/// This trait defines the interface for capturing and processing user input,
/// converting it into game commands that can be executed by the game engine.
/// Implementors are responsible for handling the specific input mechanism
/// (keyboard, mouse, network, etc.) and translating user actions into commands.
///
/// # Type Parameters
///
/// * `O` - User output handler type that implements `UserOutput`
///
/// # Examples
///
/// ```ignore
/// use pachislo::interface::UserInput;
/// use pachislo::command::Command;
///
/// struct ConsoleInput;
/// struct ConsoleOutput;
///
/// impl pachislo::interface::UserOutput for ConsoleOutput {
///     fn default(&mut self, state: pachislo::game::Transition) {}
///     fn finish_game(&mut self, state: &pachislo::game::GameState) {}
///     fn lottery_normal(&mut self, result: pachislo::lottery::LotteryResult) {}
///     fn lottery_rush(&mut self, result: pachislo::lottery::LotteryResult) {}
///     fn lottery_rush_continue(&mut self, result: pachislo::lottery::LotteryResult) {}
/// }
///
/// impl UserInput<ConsoleOutput> for ConsoleInput {
///     fn wait_for_input(&mut self) -> Command<Self, ConsoleOutput> {
///         // Implementation for reading console input
///         Command::FinishGame
///     }
/// }
/// ```
pub trait UserInput<O: UserOutput, F: FnMut(usize) -> f64 = fn(usize) -> f64, R: Rng = ThreadRng>:
    Sized
{
    /// Waits for user input and returns a command to execute.
    ///
    /// This method should block until user input is available and then
    /// convert that input into an appropriate game command.
    ///
    /// # Returns
    ///
    /// A `Command` instance representing the user's intended action.
    ///
    /// # Implementation Notes
    ///
    /// - This method may block the calling thread while waiting for input
    /// - Implementors should handle input validation and error cases gracefully
    /// - The method should return exactly one command per call
    fn wait_for_input(&mut self) -> Command<Self, O, F, R>;
}

/// Trait for handling user output in the pachislot game.
///
/// This trait defines the interface for presenting game state changes,
/// results, and feedback to the user. Implementors are responsible for
/// the specific output mechanism (console, GUI, network, etc.) and
/// formatting the information appropriately for their medium.
///
/// # Examples
///
/// ```ignore
/// use pachislo::interface::UserOutput;
/// use pachislo::game::{GameState, Transition};
/// use pachislo::lottery::LotteryResult;
///
/// struct ConsoleOutput;
///
/// impl UserOutput for ConsoleOutput {
///     fn default(&mut self, state: Transition) {
///         println!("Game state transition: {:?}", state);
///     }
///
///     fn finish_game(&mut self, state: &GameState) {
///         println!("Game finished in state: {:?}", state);
///     }
///
///     fn lottery_normal(&mut self, result: LotteryResult) {
///         println!("Normal lottery result: {:?}", result);
///     }
///
///     fn lottery_rush(&mut self, result: LotteryResult) {
///         println!("Rush lottery result: {:?}", result);
///     }
///
///     fn lottery_rush_continue(&mut self, result: LotteryResult) {
///         println!("Rush continuation result: {:?}", result);
///     }
/// }
/// ```
pub trait UserOutput {
    /// Handles default game state transitions and updates.
    ///
    /// This method is called for most game state changes to inform the user
    /// about the current game status. It provides information about both
    /// the previous state (if any) and the new current state.
    ///
    /// # Arguments
    ///
    /// * `state` - A `Transition` containing the before and after game states
    ///
    /// # Implementation Notes
    ///
    /// - This is called frequently during gameplay
    /// - Consider efficiency in implementations as this affects game performance
    /// - The `before` state may be `None` for the initial transition
    fn default(&mut self, state: Transition);

    /// Handles game completion and cleanup presentation.
    ///
    /// This method is called when a game session ends, providing the final
    /// game state for presentation to the user. Implementations typically
    /// show final scores, statistics, or cleanup messages.
    ///
    /// # Arguments
    ///
    /// * `state` - Reference to the final `GameState` before termination
    ///
    /// # Implementation Notes
    ///
    /// - This is the last method called before the game returns to uninitialized state
    /// - Good place to display session statistics or final scores
    /// - Consider saving game data or statistics at this point
    fn finish_game(&mut self, state: &GameState);

    /// Handles normal mode lottery result presentation.
    ///
    /// This method is called when a lottery event occurs during normal gameplay mode.
    /// It should present the result in a way appropriate for standard game play.
    ///
    /// # Arguments
    ///
    /// * `result` - The `LotteryResult` containing the outcome and any visual effects
    ///
    /// # Implementation Notes
    ///
    /// - Consider different presentations for `Win::Default`, `Win::FakeWin`, etc.
    /// - May want to implement animations or sound effects for dramatic effect
    /// - This affects the core gameplay experience
    fn lottery_normal(&mut self, result: LotteryResult);

    /// Handles rush mode lottery result presentation.
    ///
    /// This method is called when a lottery event occurs during rush (bonus) mode.
    /// It should present the result with appropriate excitement for the enhanced mode.
    ///
    /// # Arguments
    ///
    /// * `result` - The `LotteryResult` containing the outcome and any visual effects
    ///
    /// # Implementation Notes
    ///
    /// - Rush mode typically warrants more dramatic presentation than normal mode
    /// - Consider enhanced animations, sounds, or visual effects
    /// - This is a high-excitement moment in the game flow
    fn lottery_rush(&mut self, result: LotteryResult);

    /// Handles rush continuation lottery result presentation.
    ///
    /// This method is called when determining whether a rush sequence continues
    /// for another round. This is a critical moment that determines whether
    /// the player's bonus mode extends or returns to normal play.
    ///
    /// # Arguments
    ///
    /// * `result` - The `LotteryResult` determining rush continuation
    ///
    /// # Implementation Notes
    ///
    /// - This is often the most suspenseful moment in the game
    /// - Consider maximum dramatic effect in presentation
    /// - Win results extend the rush, lose results end it
    /// - May want to show countdown or progress indicators
    fn lottery_rush_continue(&mut self, result: LotteryResult);
}
