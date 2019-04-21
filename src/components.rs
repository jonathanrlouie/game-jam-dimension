use amethyst::ecs::prelude::{Component, DenseVecStorage};
use crate::bull_maze::Tile;

pub struct Snake;

impl Component for Snake {
    type Storage = DenseVecStorage<Self>;
}

pub struct Exit;

impl Component for Exit {
    type Storage = DenseVecStorage<Self>;
}

pub struct MapTile;

impl Component for MapTile {
    type Storage = DenseVecStorage<Self>;
}

pub struct Bull;

impl Component for Bull {
    type Storage = DenseVecStorage<Self>;
}

pub struct Map {
    pub map_change_cooldown: u32,
    pub current_layer: usize,
    pub layers: Vec<Vec<Tile>>
}

impl Component for Map {
    type Storage = DenseVecStorage<Self>;
}

pub struct MapPos {
    pub x: u32,
    pub y: u32
}

impl Component for MapPos {
    type Storage = DenseVecStorage<Self>;
}

pub struct ScreenPos {
    pub x: f32,
    pub y: f32
}

impl Component for ScreenPos {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Copy, Clone, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    NoDir
}

#[derive(Copy, Clone, Hash)]
pub enum Action {
    Stop,
    Move
}

pub struct EntityState {
    pub direction: Direction,
    pub action: Action
}

impl Component for EntityState {
    type Storage = DenseVecStorage<Self>;
}