use amethyst::{
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    core::transform::Transform,
    renderer::{SpriteSheetHandle, SpriteRender, VirtualKeyCode}
};

use crate::{
    bull_maze::Tile,
    components::{Exit, Bull, EntityState, Direction, Action, Map, MapPos, MapTile}
};

pub struct GameInputSystem;

impl<'s> System<'s> for GameInputSystem {
    type SystemData = (
        ReadStorage<'s, Exit>,
        WriteStorage<'s, Map>,
        WriteStorage<'s, MapPos>,
        WriteStorage<'s, MapTile>,
        ReadStorage<'s, Bull>,
        WriteStorage<'s, EntityState>,
        ReadStorage<'s, SpriteSheetHandle>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
        Entities<'s>,
    );

    fn run(&mut self, (_exit, mut maps, mut map_positions, mut map_tiles, bulls, mut states, sprite_sheet_handles, mut sprite_render_storage, mut transforms, input, entities): Self::SystemData) {
        let opt_ud_movement = input.axis_value("up_down");
        let opt_lr_movement = input.axis_value("left_right");

        for (mut map) in (&mut maps).join() {
            if (map.map_change_cooldown > 0) {
                map.map_change_cooldown -= 1;
            }
        }

        for (mut map, handle) in (&mut maps, &sprite_sheet_handles).join() {

            if input.key_is_down(VirtualKeyCode::Space) && map.map_change_cooldown == 0 {

                use crate::{match_tile, MAP_WIDTH, TILE_SIZE, SCREEN_HEIGHT};

                map.map_change_cooldown = 60;

                for (e, _map_tile) in (&*entities, &map_tiles).join() {
                    entities.delete(e);
                }

                let layer: usize = match map.current_layer {
                    0 => 1,
                    1 => 0,
                    _ => 0
                };

                map.current_layer = layer;

                for (i, tile) in map.layers[layer].iter().enumerate() {
                    let mut local_transform = Transform::default();
                    local_transform.set_translation_xyz(
                        (i % MAP_WIDTH) as f32 * TILE_SIZE + (TILE_SIZE / 2.0),
                        SCREEN_HEIGHT - ((i / MAP_WIDTH) as f32 * TILE_SIZE) - (TILE_SIZE / 2.0),
                        -0.1
                    );

                    let sprite_render = SpriteRender {
                        sprite_sheet: handle.clone(),
                        sprite_number: match_tile(tile),
                    };

                    entities
                        .build_entity()
                        .with(MapTile, &mut map_tiles)
                        .with(sprite_render.clone(), &mut sprite_render_storage)
                        .with(local_transform, &mut transforms)
                        .build();
                }
            }

            for (mut map_position, _bull, mut state) in (&mut map_positions, &bulls, &mut states).join() {
                match state.action {
                    Action::Stop => {
                        state.direction = get_direction(
                            state.direction,
                            opt_ud_movement,
                            opt_lr_movement
                        );
                        match state.direction {
                            Direction::Up => {
                                if valid_pos(map_position.x, map_position.y - 1, map) {
                                    map_position.y -= 1
                                } else {
                                    state.direction = Direction::NoDir;
                                    return;
                                }
                            },
                            Direction::Down => {
                                if valid_pos(map_position.x, map_position.y + 1, map) {
                                    map_position.y += 1;
                                } else {
                                    state.direction = Direction::NoDir;
                                    return;
                                }
                            },
                            Direction::Right => {
                                if valid_pos(map_position.x + 1, map_position.y, map) {
                                    map_position.x += 1
                                } else {
                                    state.direction = Direction::NoDir;
                                    return;
                                }
                            },
                            Direction::Left => {
                                if valid_pos(map_position.x - 1, map_position.y, map) {
                                    map_position.x -= 1
                                } else {
                                    state.direction = Direction::NoDir;
                                    return;
                                }
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
            }
        }

        for mut state in (&mut states).join() {
            match state.action {
                Action::Stop => {
                    match (opt_ud_movement, opt_lr_movement) {
                        (Some(ud), Some(lr)) => state.action = {
                            if ud > 0.0 || ud < 0.0 || lr > 0.0 || lr < 0.0 {
                                Action::Move
                            } else {
                                Action::Stop
                            }
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        }
    }
}

fn get_direction(
    previous_dir: Direction,
    opt_ud_movement: Option<f64>,
    opt_lr_movement: Option<f64>
) -> Direction {
    match (opt_ud_movement, opt_lr_movement) {
        (Some(ud_movement), Some(lr_movement)) => {
            if ud_movement > 0.0 {
                Direction::Down
            } else if ud_movement < 0.0 {
                Direction::Up
            } else {
                if lr_movement > 0.0 {
                    Direction::Right
                } else if lr_movement < 0.0 {
                    Direction::Left
                } else {
                    previous_dir
                }
            }
        },
        _ => Direction::NoDir
    }
}

fn valid_pos(map_x: u32, map_y: u32, map: &Map) -> bool {
    let tiles = &map.layers[map.current_layer];
    match get_tile(tiles, map_x as usize, map_y as usize) {
        Tile::Grass | Tile::AlienDirt => true,
        a => false
    }
}

fn get_tile(tiles: &Vec<Tile>, x: usize, y: usize) -> Tile {
    use crate::MAP_WIDTH;
    let index = y * MAP_WIDTH + x;
    tiles[index]
}