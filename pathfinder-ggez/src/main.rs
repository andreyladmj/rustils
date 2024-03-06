mod lib;
mod scene;

use std::rc::Rc;
use ggez::{GameError, timer};
use ggez::{conf, Context, ContextBuilder, GameResult, glam};
use ggez::conf::{FullscreenType, WindowMode};
use ggez::graphics::{self, Color, Image, ImageFormat};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::input::keyboard::KeyCode;
use ggez::mint::Vector2;

use crate::lib::*;
use crate::lib::grid::Grid;
use crate::lib::nodes::NodesMap;
use crate::scene::Scene;


const N_LAT: u32 = 10800;
const N_LON: u32 = 21600;

fn main() {

    let (mut ctx, event_loop) = ContextBuilder::new("Path Finder", "Andrii")
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
        resize_on_scale_factor_change: true,
        logical_size: None,
    }).expect("set mode failed");

    event::run(ctx, event_loop, state);
}

struct MainState {
    grid: Grid,
    scene: Scene,
    is_zooming: bool,
    is_moving: bool,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> MainState {
        let grid = Grid::new();
        let scene = Scene::new(_ctx, &grid);

        let nodes_map = NodesMap::new(&grid);

        let n1 = nodes_map.get_node(&Index::new(0,0));
        let n2 = nodes_map.get_node(&Index::new(0,0));

        let nrc = Rc::new(nodes_map.get_node(&Index::new(1,0)));

        println!("n1 == n2 {}", n1 == n2);
        println!("n1 == n1 {}", n1 == n1);
        println!("rc n1 == rc n2 {}", Rc::new(n1) == Rc::new(n2));
        println!("nrc == nrc {}", nrc == nrc);
        println!("nrc == nrc clone {}", nrc == Rc::clone(&nrc));

        MainState {
            grid: grid,
            scene: scene,
            is_zooming: false,
            is_moving: false,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let k_ctx = &_ctx.keyboard;

        if k_ctx.is_key_pressed(KeyCode::Z) {
            self.is_zooming = true;
        } else {
            self.is_zooming = false;
        }
        if k_ctx.is_key_pressed(KeyCode::X) {
            self.is_moving = true;
        } else {
            self.is_moving = false;
        }


        // const DESIRED_FPS: u32 = 60;
        //
        // while _ctx.time.check_update_time(DESIRED_FPS) {
        //     for keycode in &self.keysdown {
        //         if keycode == &KeyCode::Up {
        //             self.origin.y = self.origin.y - 2.0;
        //         }
        //         if keycode == &KeyCode::Down {
        //             self.origin.y = self.origin.y + 2.0;
        //         }
        //         if keycode == &KeyCode::Left {
        //             self.origin.x = self.origin.x - 2.0;
        //         }
        //         if keycode == &KeyCode::Right {
        //             self.origin.x = self.origin.x + 2.0;
        //         }
        //     }
        //     if self.origin.x < 0.0 {
        //         self.origin.x = 0.0;
        //     } else if self.origin.x > WORLD_WIDTH {
        //         self.origin.x = WORLD_WIDTH
        //     }
        //     if self.origin.y < 0.0 {
        //         self.origin.y = 0.0;
        //     } else if self.origin.y > WORLD_HEIGHT {
        //         self.origin.y = WORLD_HEIGHT;
        //     }
        // }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // canvas.draw(&self.worldmap, graphics::DrawParam::new());
        self.scene.draw(ctx, &mut canvas).expect("Scene draw failed");

        canvas.finish(ctx)
    }


    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) -> Result<(), GameError> {
        if self.is_zooming {
            if y > 0.0 {
                let zoom = self.scene.get_zoom() + 0.010 * self.scene.get_zoom();
                self.scene.set_zoom(zoom);
            } else if y < 0.0 {
                let zoom = self.scene.get_zoom() - 0.010 * self.scene.get_zoom();
                self.scene.set_zoom(zoom);
            }
        }
        if self.is_moving {
            self.scene.update_origin(_x * 5.0, y * 5.0);
        }
        Ok(())
    }
}