use crate::{
    game::Game,
    interface::{UserInput, UserOutput},
};

pub enum Command<I, O>
where
    I: UserInput<O>,
    O: UserOutput,
{
    FinishGame,
    Control(Box<dyn ControlCommand<I, O>>),
}

pub trait ControlCommand<I, O> {
    fn execute(&mut self, game: &mut Game<I, O>);
}

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
