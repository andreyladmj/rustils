use ggez::{Context, GameResult, glam, graphics};
use ggez::graphics::{Canvas, Color, Image, ImageFormat};
use ggez::mint::Vector2;
use crate::lib::{grid, Index, Node, Point};
use crate::lib::grid::Grid;
use crate::{N_LAT, N_LON};

struct Tile {
    img: Image,
    point: Point
}
pub struct Scene {
    tiles: Vec<Tile>,
    origin: Point,
    zoom: f32,
}

impl Scene {
    pub fn new(_ctx: &mut Context, grid: &Grid) -> Self {
        let mut tiles = vec![];
        let mut slice: Vec<u8> = vec![];

        let n_tiles = 4;
        let nklat = N_LAT / n_tiles;
        let nklon = N_LON / n_tiles;
        let mut land_points = 0;
        println!("nklat: {}", nklat);
        println!("nklon: {}", nklon);

        for ix in 0..n_tiles {
            for iy in 0..n_tiles {
                // TODO: change loop to go row by row to access memory on the same page
                for ilat in 0..nklat {
                    for ilon in 0..nklon {
                        let depth = grid.is_traversable(&Index::new(ilat + iy * nklat, ilon + ix * nklon));
                        if depth == 0 {
                            // slice1.push((128 + depth, 128 + depth, 200 + depth / 2, 100).to_rgba_u32());
                            // slice1.push(Color::from((128, 128, 200, 100)).to_rgba_u32() as u8);
                            slice.push(150);
                            land_points += 1;
                        } else {
                            // slice1.push(Color::from((0, 0, 135, 100)).to_rgba_u32() as u8);
                            slice.push(0);
                        }
                    }
                }
                tiles.push(Tile{
                    img: Image::from_pixels(_ctx, &slice, ImageFormat::R8Unorm, nklon, nklat),
                    point: Point::new(iy as f32 * nklat as f32, ix as f32 * nklon as f32)
                });
                slice.clear();
                println!("Add tile: {} {}", iy*nklat, ix*nklon);

            }
        }
        let node_size = std::mem::size_of::<Node>();
        println!("map size: {}", N_LAT * N_LON);
        println!("land points: {}, its: {}%", land_points, land_points as f32 / (N_LAT * N_LON) as f32 * 100.0);
        println!("size of node: {}b", node_size);
        println!("size of whole nodes: {}Mb", node_size as f32 * N_LAT as f32 * N_LON as f32 * 0.000001);

        Self {
            tiles: tiles,
            origin: Point::new(0.0, 0.0),
            zoom: 0.12
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        for tile in &self.tiles {
            let dst = glam::Vec2::new((self.origin.lon + tile.point.lon) * self.zoom, (self.origin.lat + tile.point.lat) * self.zoom);
            canvas.draw(&tile.img, graphics::DrawParam::new().scale(Vector2 { x: self.zoom, y: self.zoom }).dest(dst));
        }

        Ok(())
    }

    pub fn update_origin(&mut self, _dx: f32, _dy: f32) {
        self.origin.lat += _dy;
        self.origin.lon += _dx;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
        println!("zoom: {}", self.zoom);
    }

    pub fn get_zoom(&mut self) -> f32 {
        self.zoom
    }
}