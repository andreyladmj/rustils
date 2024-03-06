mod lib;

use ggez::GameError;
use ggez::{conf, Context, ContextBuilder, GameResult, glam};
use ggez::conf::{FullscreenType, WindowMode};
use ggez::graphics::{self, Color, Image, ImageFormat};
use ggez::event::{self, EventHandler};
use ggez::mint::Vector2;

use crate::lib::*;

fn main() {

    let (mut ctx, event_loop) = ContextBuilder::new("PathFinder", "Andrii")
        .backend(conf::Backend::default())
        .build()
        .expect("could not create ggez context!");

    let state = MainState::new(&mut ctx);

    ctx.gfx.set_window_title("PathFinder");
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
    tile1: Image,
    // tile2: Image,
    // tile3: Image,
    // tile4: Image,
    origin: Point,
    zoom: f32,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> MainState {
        let data = read_bin("assets/bathymetry.bin".to_string());
        println!("data: {} - {} - {}", data.len(), (21600*10800), 21600.0/8192.0);

        let mut slice1: Vec<u8> = vec![];

        let n_lat = 10800;
        let n_lon = 21600;

        for idx_lat in 0..8192 {
            for idx_lon in 0..8192 {
                // bathymetry[idxLon * this->size.nLat + (this->size.nLat - idxLat - 1)];
                match data[idx_lon * n_lat + (n_lat - idx_lat - 1)] {
                    0 => {
                        slice1.push(150);
                    },
                    1 => {
                        slice1.push(0);
                    },
                    _ => {
                        println!("mismatch: {}", data[idx_lon * n_lat + (n_lat - idx_lat - 1)]);
                    }
                }
            }
        }

        let tile1 = Image::from_pixels(_ctx, &data, ImageFormat::R8Unorm, 8192, 8192);
        MainState {
            tile1: tile1,
            origin: Point::new(0.0, 0.0),
            zoom: 0.1
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // canvas.draw(&self.worldmap, graphics::DrawParam::new());

        let dst = glam::Vec2::new(0.0, 0.0);
        canvas.draw(&self.tile1, graphics::DrawParam::new().scale(Vector2 { x: self.zoom, y: self.zoom }).dest(dst));

        canvas.finish(ctx)
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) -> Result<(), GameError> {
        if y > 0.0 {
            self.zoom = self.zoom - 0.01;
        } else if y < 0.0 {
            self.zoom = self.zoom + 0.01;
        }
        println!("self.zoom: {}", self.zoom);
        Ok(())
    }
}