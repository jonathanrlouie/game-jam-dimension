use crate::systems::{
    input::GameInputSystem,
    movement::MovementSystem,
    win::WinSystem,
};
use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::DispatcherBuilder,
    error::Error
};

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(GameInputSystem, "game_input_system", &["input_system"]);
        builder.add(MovementSystem, "movement_system", &[]);
        builder.add(WinSystem, "win_system", &[]);
        Ok(())
    }
}