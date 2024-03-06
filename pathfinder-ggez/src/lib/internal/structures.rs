pub struct Point {
    lat: f64,
    lon: f64,
}

impl Point {
    pub fn new(lat: f64, lon: f64) -> Self {
        Self {lat, lon}
    }
}

pub struct Index {
    idx_lat: u32,
    idx_lon: u32,
}

