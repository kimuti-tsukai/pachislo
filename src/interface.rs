use crate::{
    command::Command,
    game::{GameState, Transition},
    lottery::LotteryResult,
};

pub trait UserInput<O: UserOutput>: Sized {
    fn wait_for_input(&mut self) -> Vec<Command<Self, O>>;
}

pub trait UserOutput {
    fn default(&mut self, state: Transition);
    fn finish_game(&mut self, state: &GameState);
    fn lottery_normal(&mut self, result: LotteryResult);
    fn lottery_rush(&mut self, result: LotteryResult);
    fn lottery_rush_continue(&mut self, result: LotteryResult);
}
