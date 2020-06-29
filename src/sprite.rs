use ggez::{Context, graphics};
use shipyard::{UniqueViewMut, View, IntoIter};
use ggez::graphics::{DrawParam, Drawable};
use std::collections::HashMap;

pub struct Sprite {
    pub name: String,
}

impl Sprite {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
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
    pub fn new() -> Self {
        Self { images: HashMap::new() }
    }

    pub fn image(&mut self, ctx: &mut Context, path: &str) -> &graphics::Image {
        self.images.entry(path.to_string()).or_insert_with(||
            graphics::Image::new(ctx, path).unwrap()
        )
    }
}