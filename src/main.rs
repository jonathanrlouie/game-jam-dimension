use std::time::Duration;
use amethyst;
use amethyst::{
    animation::AnimationBundle,
    config::Config,
    prelude::{
        Application,
        GameDataBuilder
    },
    core::{
        frame_limiter::FrameRateLimitStrategy,
        transform::TransformBundle
    },
    input::InputBundle,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, SpriteRender, Stage},
    utils::application_root_dir
};

mod bull_maze;
mod components;
mod systems;
mod bundle;

use bundle::GameBundle;
use bull_maze::{BullMaze, Tile};

const TILE_SIZE: f32 = 32.0;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

const MAP_WIDTH: usize = 20;
const MAP_HEIGHT: usize = 15;

const MAP_SIZE: usize = get_map_size(MAP_WIDTH, MAP_HEIGHT);

const fn get_map_size(width: usize, height: usize) -> usize {
    width * height
}

fn match_tile(tile: &Tile) -> usize {
    match tile {
        Tile::Grass => 0,
        Tile::Wall => 1,
        Tile::AlienDirt => 3
    }
}

const FRAME_LIMIT: u32 = 60;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum AnimationId {
    Walk
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("resources/display.ron");

    let assets_dir = app_root.join("assets/");

    let key_bindings_path = app_root.join("resources/input.ron");

    let config = DisplayConfig::load(&display_config_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?
        )?
        .with_bundle(GameBundle)?
        .with_bundle(TransformBundle::new().with_dep(&["movement_system"]))?
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?;

    let mut game = Application::build(assets_dir, BullMaze)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            FRAME_LIMIT,
        )
        .build(game_data)?;

    game.run();
    Ok(())
}