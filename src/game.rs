use shipyard::{World, View, IntoIter, UniqueViewMut};
use crate::state::State;
use ggez::{Context, graphics};
use ggez::graphics::{DrawParam, Drawable};
use std::collections::HashMap;
use crate::sprite::{SpriteCache, draw_sprites, Sprite};
use crate::anchor::{Anchor, VerticalAnchor, HorizontalAnchor};

struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

struct Ninja {}

pub fn init(world: &World, ctx: &mut Context) {
    world.add_unique(SpriteCache::new());
    world.run_with_data(draw_sprites, ctx);
}

pub fn init_entities(world: &World) {
    world.entity_builder()
        .with(Sprite::new("/throne.png"))
        .with(Position::new(0., 0.))
        .build();

    world.entity_builder()
        .with(Sprite::new("/ninja1_idle.png"))
        .with(Position::new(0., 0.))
        .with(Anchor::new(VerticalAnchor::Bottom, HorizontalAnchor::Middle))
        .build();
}


