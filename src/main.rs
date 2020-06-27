mod state;

use ggez::*;
use ggez::graphics::{Color, Rect, Drawable, DrawParam, FilterMode};
use ggez::conf::FullscreenType;
use std::env;
use std::path;
use crate::state::State;

const SCREEN_WIDTH: f32 = 320.;
const SCREEN_HEIGHT: f32 = 200.;

fn main() {
    let mut state = State::new();

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

    event::run(ctx, event_loop, &mut state).unwrap();
}
