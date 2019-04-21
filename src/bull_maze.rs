use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    prelude::{State, StateEvent, StateData, GameData, SimpleState},
    ecs::prelude::{World, Builder},
    renderer::{Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
               Texture, TextureMetadata}
};
use super::components;
use super::components::{Direction, Action};
use crate::{match_tile, SCREEN_WIDTH, SCREEN_HEIGHT, MAP_WIDTH, MAP_HEIGHT, MAP_SIZE, TILE_SIZE, get_map_size};

struct Bull {
    pos: (u32, u32)
}

impl Bull {
    fn new(pos: (u32, u32)) -> Self {
        Bull {
            pos
        }
    }

    fn initialise(&self, world: &mut World, sprite_sheet_handle: SpriteSheetHandle, map: &Map) {
        let mut local_transform = Transform::default();
        let (x, y) = map.get_tile_pos(self.pos.0, self.pos.1);
        local_transform.set_translation_xyz(
            x,
            y,
            0.0
        );

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 2,
        };

        world
            .create_entity()
            .with(components::Bull)
            .with(components::MapPos {
                x: self.pos.0,
                y: self.pos.1
            })
            .with(components::ScreenPos {
                x,
                y
            })
            .with(components::EntityState {
                direction: Direction::NoDir,
                action: Action::Stop
            })
            .with(sprite_render.clone())
            .with(local_transform)
            .build();
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Tile {
    Grass,
    Wall,
    AlienDirt,
}

const MAP: [Tile; MAP_SIZE] = [
    Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall,
    Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall,
    Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall,
    Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall,
    Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall,
    Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall,
    Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall,
    Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall,
    Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall,
    Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall,
    Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall,
    Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall,
    Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall, Tile::Grass, Tile::Wall,
    Tile::Wall, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Wall, Tile::Grass, Tile::Wall,
    Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall,
];

const MAP2: [Tile; MAP_SIZE] = [
    Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall,
    Tile::Wall, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::Wall,
    Tile::Wall, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::Wall,
    Tile::Wall, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::Wall,
    Tile::Wall, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::Wall,
    Tile::Wall, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::Wall,
    Tile::Wall, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::Wall,
    Tile::Wall, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::Wall,
    Tile::Wall, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::Wall,
    Tile::Wall, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::Wall,
    Tile::Wall, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::Wall,
    Tile::Wall, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::Wall,
    Tile::Wall, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::Wall, Tile::Wall, Tile::Wall,
    Tile::Wall, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::AlienDirt, Tile::Wall, Tile::AlienDirt, Tile::Wall,
    Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall,
];

struct Map {
    tiles: [Tile; MAP_SIZE]
}

impl Map {
    fn new(tiles: [Tile; MAP_SIZE]) -> Self {
        Map {
            tiles
        }
    }

    fn get_tile_pos(&self, x: u32, y: u32) -> (f32, f32) {
        (x as f32 * TILE_SIZE + (TILE_SIZE / 2.0), SCREEN_HEIGHT - (y as f32 * TILE_SIZE) - (TILE_SIZE / 2.0))
    }

    fn initialise(
        &self,
        world: &mut World,
        sprite_sheet_handle: SpriteSheetHandle
    ) {

        for (i, tile) in self.tiles.iter().enumerate() {
            let mut local_transform = Transform::default();
            local_transform.set_translation_xyz(
                (i % MAP_WIDTH) as f32 * TILE_SIZE + (TILE_SIZE / 2.0),
                SCREEN_HEIGHT - ((i / MAP_WIDTH) as f32 * TILE_SIZE) - (TILE_SIZE / 2.0),
                -0.1
            );

            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet_handle.clone(),
                sprite_number: match_tile(tile),
            };

            world
                .create_entity()
                .with(components::MapTile)
                .with(sprite_render.clone())
                .with(local_transform)
                .build();
        }

        world
            .create_entity()
            .with(sprite_sheet_handle.clone())
            .with(components::Map {
                map_change_cooldown: 0,
                current_layer: 0,
                layers: vec![
                    self.tiles.iter().map(|t: &Tile| *t).collect::<Vec<Tile>>(),
                    MAP2.iter().map(|t: &Tile| *t).collect::<Vec<Tile>>()]
            })
            .build();
    }
}

pub struct BullMaze;

impl SimpleState for BullMaze {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        let sprite_sheet_handle = load_sprite_sheet(world);

        initialise_camera(world);

        let map = Map::new(MAP);
        map.initialise(world, sprite_sheet_handle.clone());

        let bull = Bull::new((1, 1));
        bull.initialise(world, sprite_sheet_handle.clone(), &map);

        initialise_exit(world, sprite_sheet_handle, &map);
    }
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/spritesheet.png",
            PngFormat,
            TextureMetadata::srgb(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            SCREEN_WIDTH,
            0.0,
            SCREEN_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn initialise_exit(world: &mut World, sprite_sheet_handle: SpriteSheetHandle, map: &Map) {
    let map_x = 18;
    let map_y = 13;

    let mut transform = Transform::default();
    let (x, y) = map.get_tile_pos(map_x, map_y);
    transform.set_translation_xyz(
        x,
        y,
        -0.01
    );

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 5,
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(components::MapPos {
            x: map_x,
            y: map_y
        })
        .with(components::Exit)
        .with(transform)
        .build();
}