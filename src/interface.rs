use crate::{
    command::ControllCommand,
    game::{GameState, Transition},
    lottery::LotteryResult,
};

pub trait UserInput<O: UserOutput> {
    fn wait_for_input(&mut self) -> Option<Box<dyn ControllCommand<Self, O>>>;
}

pub trait UserOutput {
    fn default(&mut self, state: Transition<'_>);
    fn finish_game(&mut self, state: &GameState);
    fn lottery_normal(&mut self, result: LotteryResult);
    fn lottery_rush(&mut self, result: LotteryResult);
    fn lottery_rush_continue(&mut self, result: LotteryResult);
}
