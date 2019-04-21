use amethyst::{
    ecs::prelude::{Entity, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    ui::UiText
};

use crate::components::{
    Bull,
    MapPos,
    Exit
};

const VELOCITY: f32 = 160.0;

pub struct WinSystem;

impl<'s> System<'s> for WinSystem {
    type SystemData = (
        ReadStorage<'s, MapPos>,
        ReadStorage<'s, Bull>,
        ReadStorage<'s, Exit>,
        ReadExpect<'s, WinText>,
        WriteStorage<'s, UiText>,
    );

    fn run(&mut self, (map_positions, bulls, exits, win_text, mut text): Self::SystemData) {
        let mut bull_pos = &MapPos {
            x: 0,
            y: 0
        };

        for (map_pos, bull) in (&map_positions, &bulls).join() {
            bull_pos = map_pos;
        }

        for (map_pos, exit) in (&map_positions, &exits).join() {
            if (bull_pos.x == map_pos.x && bull_pos.y == map_pos.y) {
                if let Some(text) = text.get_mut(win_text.win_text) {
                    text.text = "You win!".to_string();
                }
            }
        }
    }
}

pub struct WinText {
    pub win_text: Entity
}