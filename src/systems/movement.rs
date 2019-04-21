use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{
    EntityState,
    Action,
    Direction,
    MapPos,
    ScreenPos
};

const VELOCITY: f32 = 160.0;

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, MapPos>,
        WriteStorage<'s, EntityState>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, ScreenPos>,
        Read<'s, Time>,
    );

    fn run(&mut self, (map_positions, mut states, mut locals, mut screen_positions, time): Self::SystemData) {
        for (map_pos, mut state, local, mut screen_pos) in (&map_positions, &mut states, &mut locals, &mut screen_positions).join() {
            let direction = state.direction;
            match state.action {
                Action::Move => {
                    use crate::{TILE_SIZE, SCREEN_HEIGHT};
                    match direction {
                        Direction::Up => {
                            let distance = VELOCITY * time.delta_seconds();
                            let next_pos = screen_pos.y + distance;
                            let next_tile_pos = SCREEN_HEIGHT - ((map_pos.y as f32 * TILE_SIZE) + (TILE_SIZE / 2.0));
                            if next_pos > next_tile_pos {
                                state.action = Action::Stop;
                                state.direction = Direction::NoDir;
                                local.set_translation_y(next_tile_pos);
                                screen_pos.y = next_tile_pos;
                            } else {
                                screen_pos.y = next_pos;
                                local.set_translation_y(screen_pos.y);
                            }
                        },
                        Direction::Down => {
                            let distance = VELOCITY * time.delta_seconds();
                            let next_pos = screen_pos.y - distance;
                            let next_tile_pos = SCREEN_HEIGHT - ((map_pos.y as f32 * TILE_SIZE) + (TILE_SIZE / 2.0));
                            if next_pos < next_tile_pos {
                                state.action = Action::Stop;
                                state.direction = Direction::NoDir;
                                local.set_translation_y(next_tile_pos);
                                screen_pos.y = next_tile_pos;
                            } else {
                                screen_pos.y = next_pos;
                                local.set_translation_y(screen_pos.y);
                            }
                        },
                        Direction::Right => {
                            let distance = VELOCITY * time.delta_seconds();
                            let next_pos = screen_pos.x + distance;
                            let next_tile_pos = map_pos.x as f32 * TILE_SIZE + (TILE_SIZE / 2.0);
                            if next_pos > next_tile_pos {
                                state.action = Action::Stop;
                                state.direction = Direction::NoDir;
                                local.set_translation_x(next_tile_pos);
                                screen_pos.x = next_tile_pos;
                            } else {
                                screen_pos.x = next_pos;
                                local.set_translation_x(screen_pos.x);
                            }
                        },
                        Direction::Left => {
                            let distance = -VELOCITY * time.delta_seconds();
                            let next_pos = screen_pos.x + distance;
                            let next_tile_pos = map_pos.x as f32 * TILE_SIZE + (TILE_SIZE / 2.0);
                            if screen_pos.x < next_tile_pos {
                                state.action = Action::Stop;
                                state.direction = Direction::NoDir;
                                local.set_translation_x(next_tile_pos);
                                screen_pos.x = next_tile_pos;
                            } else {
                                screen_pos.x = next_pos;
                                local.set_translation_x(screen_pos.x);
                            }
                        },
                        _ => (),
                    }
                },
                _ => ()
            }
        }
    }
}