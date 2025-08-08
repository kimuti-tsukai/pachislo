use crate::{game::Game, interface::{UserInput, UserOutput}};

pub trait ControllCommand<I, O> {
    fn execute(&mut self, game: &mut Game<I, O>);
}

pub struct LaunchBall;

impl<I, O> ControllCommand<I, O> for LaunchBall
where
    I: UserInput<O>,
    O: UserOutput,
{
    fn execute(&mut self, game: &mut Game<I, O>) {
        let _ = game.launch_ball();
    }
}

pub struct StartGame;

impl<I, O> ControllCommand<I, O> for StartGame
where
    I: UserInput<O>,
    O: UserOutput,
{
    fn execute(&mut self, game: &mut Game<I, O>) {
        let _ = game.start();
    }
}

pub struct FinishGame;

impl<I, O> ControllCommand<I, O> for FinishGame
where
    I: UserInput<O>,
    O: UserOutput,
{
    fn execute(&mut self, game: &mut Game<I, O>) {
        let _ = game.finish();
    }
}
