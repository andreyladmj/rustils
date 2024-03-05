use ggez::{Context, ContextBuilder, GameResult, glam};
use ggez::conf::{FullscreenType, WindowMode};
use ggez::graphics::{self, Color, Image, ImageFormat};
use ggez::event::{self, EventHandler};

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("PathFinder", "Andrii")
        .build()
        .expect("could not create ggez context!");

    let state = MainState::new(&mut ctx);

    ctx.gfx.set_mode(WindowMode {
        width: 2800.0,
        height: 1700.0,
        maximized: true,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        transparent: false,
        min_width: 640.0,
        min_height: 420.0,
        max_width: 3000.0,
        max_height: 3000.0,
        resizable: true,
        visible: true,
        resize_on_scale_factor_change: false,
        logical_size: None,
    }).expect("set mode failed");

    event::run(ctx, event_loop, state);
}

struct MainState {
    worldmap: Image
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> MainState {
        let worldimage = Image::from_pixels(_ctx, &[150, 150, 150, 150], ImageFormat::R8Unorm, 2, 2);
        MainState {
            worldmap: worldimage
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // canvas.draw(&self.worldmap, graphics::DrawParam::new());

        let dst = glam::Vec2::new(20.0, 20.0);
        canvas.draw(&self.worldmap, graphics::DrawParam::new().dest(dst));

        canvas.finish(ctx)
    }
}