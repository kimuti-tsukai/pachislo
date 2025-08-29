use rand::{Rng, rngs::ThreadRng};

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

impl<I, O> Command<I, O>
where
    I: UserInput<O>,
    O: UserOutput,
{
    pub fn control<C>(control: C) -> Self
    where
        C: ControlCommand<I, O> + 'static,
    {
        Self::Control(Box::new(control))
    }
}

pub trait ControlCommand<I, O>
where
    I: UserInput<O>,
    O: UserOutput,
{
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

pub struct LaunchBallFlowProducer {
    start_hole_probability: f64,
    rng: ThreadRng,
}

impl LaunchBallFlowProducer {
    pub fn new(start_hole_probability: f64) -> Self {
        Self {
            start_hole_probability,
            rng: rand::rng(),
        }
    }

    pub fn produce(&mut self) -> LaunchBallFlow {
        LaunchBallFlow::new(self.rng.random_bool(self.start_hole_probability))
    }
}

pub struct LaunchBallFlow {
    is_lottery: bool,
}

impl LaunchBallFlow {
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
