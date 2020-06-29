use shipyard::{World, View, IntoIter, UniqueViewMut};
use crate::state::State;
use ggez::{Context, graphics};
use ggez::graphics::{DrawParam, Drawable};
use std::collections::HashMap;

struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct Sprite {
    pub name: String,
}

impl Sprite {
    fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

struct Ninja {}

enum VerticalAnchor {
    Top,
    Middle,
    Bottom,
}

enum HorizontalAnchor {
    Left,
    Middle,
    Right,
}

struct Anchor {
    vertical: VerticalAnchor,
    horizontal: HorizontalAnchor,
}

impl Anchor {
    fn new(vertical: VerticalAnchor, horizontal: HorizontalAnchor) -> Self {
        Self { vertical, horizontal }
    }
}

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

pub fn draw_sprites(
    ctx: &mut Context,
    mut sprite_cache: UniqueViewMut<SpriteCache>,
    sprites: View<Sprite>,
    positions: View<Sprite>,
) {
    for (sprite, position) in (&sprites, &positions).iter() {
        sprite_cache.image(ctx, &sprite.name).draw(ctx, DrawParam::new());
    }
}

pub struct SpriteCache {
    images: HashMap<String, graphics::Image>,
}

impl SpriteCache {
    fn new() -> Self {
        Self { images: HashMap::new() }
    }

    pub fn image(&mut self, ctx: &mut Context, path: &str) -> &graphics::Image {
        self.images.entry(path.to_string()).or_insert_with(||
            graphics::Image::new(ctx, path).unwrap()
        )
    }
}
