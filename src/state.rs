use ggez::*;
use ggez::graphics::{Color, Drawable, DrawParam};
use mint::Vector2;
use shipyard::World;
use crate::SCREEN_WIDTH;
use std::collections::HashMap;

pub struct State {
    world: World,
    images: HashMap<String, graphics::Image>,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            world: World::new(),
            images: HashMap::new(),
        }
    }

    pub fn image(&mut self, ctx: &mut Context, path: &str) -> &graphics::Image {
        self.images.entry(path.to_string()).or_insert_with(||
            graphics::Image::new(ctx, path).unwrap()
        )
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // if input::keyboard::is_key_pressed(ctx, KeyCode::A) {
        //     println!("The A key is pressed");
        //     if input::keyboard::is_mod_active(ctx, input::keyboard::KeyMods::SHIFT) {
        //         println!("The shift key is held too.");
        //     }
        //     println!(
        //         "Full list of pressed keys: {:?}",
        //         input::keyboard::pressed_keys(ctx)
        //     );
        // }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));

        // let image =
        //     graphics::Image::new(ctx, "/throne.png")?;

        self.image(ctx, "/throne.png").draw(ctx, DrawParam::new());

        let font = graphics::Font::new(ctx, "/fonts/Open Sans Px/OpenSansPX.ttf")?;

        let mut scoreboard_text = graphics::Text::new("hi jart what's going on? huh? pew pew pew 1234567890");
        scoreboard_text.set_font(font, graphics::Scale::uniform(70.0));

        let coords = [
            SCREEN_WIDTH / 2.0 - scoreboard_text.width(ctx) as f32 / 2.0,
            90.0,
        ];

        let coords: [f32; 2] = [0.0, 10.];

        let params = graphics::DrawParam::default()
            .dest(coords)
            .color(Color::new(0., 0., 0., 1.0))
            .scale(Vector2 { x: 0.2, y: 0.2 });
        graphics::draw(ctx, &scoreboard_text, params).expect("error drawing scoreboard text");

        graphics::present(ctx).expect("error presenting");
        Ok(())
    }
}

