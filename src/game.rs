use shipyard::{World, View, IntoIter, UniqueViewMut};

use ggez::{Context};
use ggez::nalgebra::Point2;

use std::collections::HashMap;
use crate::sprite::{SpriteCache, draw_sprites, Sprite, Anchor, VerticalAnchor, HorizontalAnchor};

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn point2(self) -> Point2<f32> {
        return Point2::<f32>::new(self.x, self.y);
    }
}

struct Ninja {}

pub fn init(world: &World, ctx: &mut Context) {
    world.add_unique(SpriteCache::new());
    init_entities(&world);
}

pub fn update(world: &World, ctx: &mut Context) {
}

pub fn draw(world: &World, ctx: &mut Context) {
    world.run_with_data(draw_sprites, ctx)
}

pub fn init_entities(world: &World) {
    world.entity_builder()
        .with(Sprite::new("/throne.png"))
        .with(Position::new(0., 0.))
        .build();

    world.entity_builder()
        .with(Sprite::new("/ninja1_idle.png"))
        .with(Position::new(135., 190.))
        .with(Anchor::new(VerticalAnchor::Bottom, HorizontalAnchor::Middle))
        .build();
}


