use std::f32::consts::PI;
use crate::lib::{bisect, bisect_reversed, Index, Point, read_bin};
use crate::{N_LAT, N_LON};

pub struct Grid {
    bathymetry: Vec<u8>,
    longitudes: Vec<f32>,
    latitudes: Vec<f32>,
    step: f32,
}

impl Grid {
    pub fn new() -> Self {
        let data = read_bin("assets/bathymetry.bin".to_string());
        let longitudes = get_longitudes(N_LON as usize);
        let latitudes = get_longitudes(N_LAT as usize);

        Self {
            bathymetry: data,
            //0.000290870667 - original step - 1.8552022011926999 km
            step: (&latitudes[0] - &latitudes[1]) * 10.0, // 18km
            longitudes: longitudes,
            latitudes: latitudes,
        }
    }

    pub fn is_traversable(&self, idx: &Index) -> u8 {
        let idx = idx.idx_lon * N_LAT + (N_LAT - idx.idx_lat - 1);
        self.bathymetry[idx as usize]
    }

    pub fn get_step(&self) -> f32 {
        self.step
    }

    pub fn get_point(&self, idx: Index) -> Point {
        Point::new(self.latitudes[idx.idx_lat as usize], self.longitudes[idx.idx_lon as usize])
    }

    pub fn get_index(&self, point: &Point) -> Index {
        let idx_lat = bisect_reversed(&self.latitudes, N_LAT, point.lat);
        let idx_lon = bisect(&self.longitudes, N_LON, point.lon);
        Index::new(idx_lat, idx_lon)
    }

}

fn get_latitudes(size: usize) -> Vec<f32> {
    let mut v = vec![0.0f32; size];
    let step = PI / (size as f32 - 1.0);
    for i in 0..size {
        v[i] = PI / 2.0 - step * i as f32;
    }
    v
}
fn get_longitudes(size: usize) -> Vec<f32> {
    let mut v = vec![0.0f32; size];
    let step = 2.0 * PI / (size as f32 - 1.0);
    for i in 0..size {
        v[i] = - PI - step * i as f32;
    }
    v
}