use ggez::{Context, graphics};
use shipyard::{UniqueViewMut, View, IntoIter, Shiperator, Get, ViewMut};

use ggez::graphics::{DrawParam, Drawable, Rect};
use ggez::nalgebra::Point2;
use std::collections::HashMap;
use crate::game::Position;

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
    positions: View<Position>,
    anchors: View<Anchor>,
) {
    for (id, (sprite, position)) in (&sprites, &positions).iter().with_id() {
        let image = sprite_cache.image(ctx, &sprite.name);

        let mut real_position = *position;
        if let Ok(anchor) = (&anchors).get(id) {
            anchor.apply(&mut real_position, &image.dimensions())
        }

        dbg!(&real_position);

        // image.draw(ctx, DrawParam{dest: real_position.point2(), ..DrawParam::default()});
        // image.draw(ctx, DrawParam{dest: real_position.point2(), ..DrawParam::default()});
        graphics::draw(ctx, image, (real_position.point2(),));
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

#[derive(Debug)]
pub enum VerticalAnchor {
    Top,
    Middle,
    Bottom,
}

#[derive(Debug)]
pub enum HorizontalAnchor {
    Left,
    Middle,
    Right,
}

#[derive(Debug)]
pub struct Anchor {
    vertical: VerticalAnchor,
    horizontal: HorizontalAnchor,
}

impl Anchor {
    pub fn new(vertical: VerticalAnchor, horizontal: HorizontalAnchor) -> Self {
        Self { vertical, horizontal }
    }

    pub fn apply(&self, position: &mut Position, size: &Rect) {
        position.x -= match self.horizontal {
            HorizontalAnchor::Left => 0.,
            HorizontalAnchor::Middle => size.w / 2.,
            HorizontalAnchor::Right => size.w,
        };
        position.y -= match self.vertical {
            VerticalAnchor::Top => 0.,
            VerticalAnchor::Middle => size.h / 2.,
            VerticalAnchor::Bottom => size.h,
        };
    }
}

