use std::cell::RefCell;
use std::rc::Rc;
use crate::N_LAT;

#[derive(Debug)]
pub struct Point {
    pub lat: f32,
    pub lon: f32,
}

impl Point {
    pub fn new(lat: f32, lon: f32) -> Self {
        Self { lat, lon }
    }
}

#[derive(Debug)]
pub struct Index {
    pub idx_lat: u32,
    pub idx_lon: u32,
}

impl Index {
    pub fn new(idx_lat: u32, idx_lon: u32) -> Self {
        Self { idx_lat, idx_lon }
    }
}

impl PartialEq for Index {
    fn eq(&self, other: &Self) -> bool {
        self.idx_lat == other.idx_lat && self.idx_lon == other.idx_lon
    }
}

pub type RCNode = Rc<RefCell<Node>>;

#[derive(Debug)]
pub struct Node {
    pub index: Index,
    pub visited: bool,
    pub obstacle: bool,
    pub fscore: f32,
    pub gscore: f32,
    pub parent: Option<RCNode>,
}

impl Node {
    pub fn new(idx: Index) -> Self {
        Self {
            index: idx,
            visited: false,
            obstacle: false,
            fscore: f32::INFINITY,
            gscore: f32::INFINITY,
            parent: None,
        }
    }

    pub fn hash_idx(&self) -> u32 {
        self.index.idx_lon * N_LAT + (N_LAT - self.index.idx_lat - 1)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
