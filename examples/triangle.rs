extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::timer;
use ggez::graphics::{DrawMode, Point};
use std::time::Duration;

// First we make a structure to contain the game's state
struct MainState {
    image1: graphics::Image,
    image2: graphics::Image,
    zoomlevel: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {

        let image1 = graphics::Image::new(ctx, "resources/dragon1.png")?;
        let image2 = graphics::Image::new(ctx, "resources/player.png")?;
        let s = MainState {
            image1: image1,
            image2: image2,
            zoomlevel: 1.0,
        };

        // graphics::set_screen_coordinates(ctx, 0.0, s.zoomlevel, s.zoomlevel, 0.0);
        Ok(s)
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        // graphics::set_screen_coordinates(ctx, 0.0, self.zoomlevel, self.zoomlevel, 0.0);
        // graphics::set_screen_coordinates(ctx, 0.0, self.zoomlevel, 0.0, self.zoomlevel);
        self.zoomlevel += 0.01;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::draw(ctx,
                       &mut self.image1,
                       graphics::Rect::zero(),
                       graphics::Point::zero(),
                       0.0)?;
        let dst = graphics::Point::new(100.0, 100.0);
        let scale = graphics::Point::new(2.0, 2.0);
        // graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0));
        graphics::draw_ex(ctx,
                          &mut self.image2,
                          graphics::Rect::zero(),
                          dst,
                          self.zoomlevel,
                          scale,
                          graphics::Point::zero(),
                          graphics::Point::zero())?;

        let rect = graphics::Rect::new(450.0, 450.0, 50.0, 50.0);
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)?;

        graphics::set_line_width(ctx, 4.0);
        graphics::line(ctx,
                       &[Point {
                             x: 200.0,
                             y: 200.0,
                         },
                         Point {
                             x: 400.0,
                             y: 200.0,
                         },
                         Point {
                             x: 400.0,
                             y: 400.0,
                         },
                         Point {
                             x: 200.0,
                             y: 400.0,
                         },
                         Point {
                             x: 200.0,
                             y: 200.0,
                         }])?;

        graphics::ellipse(ctx,
                          DrawMode::Fill,
                          Point {
                              x: 600.0,
                              y: 200.0,
                          },
                          50.0,
                          120.0,
                          32)?;

        graphics::circle(ctx,
                         DrawMode::Fill,
                         Point {
                             x: 600.0,
                             y: 380.0,
                         },
                         40.0,
                         32)?;

        graphics::present(ctx);
        // println!("Approx FPS: {}", timer::get_fps(ctx));
        // timer::sleep_until_next_frame(ctx, 60);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("helloworld", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
