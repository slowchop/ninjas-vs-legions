mod sprite;
mod game;
mod state;

use ggez::*;
use ggez::graphics::{Color, Rect, Drawable, DrawParam, FilterMode};
use ggez::conf::FullscreenType;
use std::env;
use std::path;
use crate::state::State;
use ncollide2d::shape::{Ball, ShapeHandle};
use ncollide2d::pipeline::{DBVTBroadPhase, BroadPhase, CollisionGroups, GeometricQueryType, ContactEvent, CollisionObjectSlabHandle};
use ::nalgebra::{Isometry2, Vector2};
use ncollide2d::world::CollisionWorld;

const SCREEN_WIDTH: f32 = 320.;
const SCREEN_HEIGHT: f32 = 200.;

struct Collider {
}

fn main() {
    // Testing...
    let ball1_shape = ShapeHandle::new(Ball::new(0.5));
    let ball1_pos = Isometry2::new(Vector2::new(1., 1.), 0.);
    let ball2_shape = ShapeHandle::new(Ball::new(0.5));
    let ball2_pos = Isometry2::new(Vector2::new(1.9, 1.), 0.);

    let mut collisions = CollisionWorld::new(0.02);
    let contact_query = GeometricQueryType::Contacts(0.0, 0.0);
    let (slab1, co1) = collisions.add(ball1_pos, ball1_shape, CollisionGroups::new(), contact_query, 5);
    dbg!(slab1);

    collisions.add(ball2_pos, ball2_shape, CollisionGroups::new(), contact_query, 6);

    collisions.update();

    for a in collisions.contact_events() {
        match a {
            ContactEvent::Started(c1, c2) => {
                let pair  = collisions.contact_pair(*c1, *c2, false).unwrap();
                let (h1, h2, algo, manifold) = pair;
                dbg!(manifold.deepest_contact());

                // let co = collisions.collision_object(*c1);
                // let co = co.unwrap();
                // dbg!(co.data());
            }
            _ => (),
        }
        dbg!(a);
        // let what = contact_query.contact_pair(a);
    }

    let mut c = conf::Conf::new();
    c.window_setup.title = "Ninjas vs Legions".to_string();

    let mut cb = ContextBuilder::new("ninjas-vs-legions", "Slowchop Studios")
        .conf(c);

    // Hack to run in dev mode.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }

    let (ref mut ctx, ref mut event_loop) = cb.build().unwrap();

    graphics::set_screen_coordinates(ctx, Rect::new(0., 0., SCREEN_WIDTH, SCREEN_HEIGHT));

    // Do this instead of graphics::fullscreen because of a bug.
    let window = graphics::window(ctx);
    let monitor = window.get_current_monitor();
    window.set_fullscreen(Some(monitor));

    println!("Full filesystem info: {:#?}", ctx.filesystem);

    println!("Resource stats:");
    filesystem::print_all(ctx);

    graphics::set_default_filter(ctx, FilterMode::Nearest);

    let mut state = State::new();

    println!("init");
    game::init(&state.world, ctx);

    event::run(ctx, event_loop, &mut state).unwrap();
}
