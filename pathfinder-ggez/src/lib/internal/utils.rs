use std::fs::File;
use std::io::Read;
use crate::N_LON;

pub fn read_bin(filename: String) -> Vec<u8> {
    let mut f = File::open(filename).unwrap();
    let mut buffer = vec![];
    f.read_to_end(&mut buffer).unwrap();
    buffer
}

pub fn bisect(v: &Vec<f32>, mut hi: u32, x: f32) -> u32 {
    let mut lo: u32 = 0;
    let mut mid: u32;

    while lo < hi {
        mid = ((lo + hi) as f32 / 2.0).floor() as u32;

        if x < v[mid as usize] {
            hi = mid;
        } else {
           lo = mid + 1;
        }
    }
    if lo > 0 {
        return lo - 1;
    }
    0
}

pub fn bisect_reversed(v: &Vec<f32>, mut hi: u32, x: f32) -> u32 {
    let mut lo: u32 = 0;
    let mut mid: u32;

    while lo < hi {
        mid = ((lo + hi) as f32 / 2.0).floor() as u32;
        if x > v[mid as usize] {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    if lo > 0 {
        return lo - 1;
    }

    0
}

pub fn normalize_lon_index(idx: i32) -> i32 {
    if idx < 0 {
        return idx + N_LON as i32;
    }

    if idx >= N_LON as i32 {
        return idx - N_LON as i32;
    }

    idx
}

pub fn haversine_rad(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f32 {
    let lat = lat2 - lat1;
    let lon = lon2 - lon1;
    2.0 * ((lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (lon / 2.0).sin().powi(2)).sqrt().asin()
}
