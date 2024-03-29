use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;
use crate::lib::grid::Grid;
use crate::lib::{Index, Node, RCNode};
use crate::{N_LAT, N_LON};

pub struct NodesMap<'a> {
    grid: &'a Grid,
    nodes: Vec<Option<RCNode>>
}

const SIGNS: [(i32, i32); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (1, 1), (1, -1), (-1, 1), (-1, -1)];


impl <'a>NodesMap<'a> {
    pub fn new(grid: &'a Grid) -> Self {
        let start = Instant::now();
        let nodes = vec![None; (N_LAT * N_LON) as usize];
        // let mut nodes = Vec::with_capacity((N_LAT * N_LON) as usize);
        //
        // for x in 0..N_LON {
        //     for y in 0..N_LAT {
        //         let mut n = Node::new(Index::new(y, x));
        //         n.obstacle = grid.is_traversable(&n.index) != 0;
        //         nodes.push(RCNode::new(RefCell::new(n)));
        //     }
        // }

        let duration = start.elapsed();
        println!("Time elapsed in NodesMap creation is: {:?}", duration);

        // land points: 79600971, its: 34.1225%
        // so its better to fill whole map with None
        // and create node only when accessing it
        let node_size = std::mem::size_of::<Vec<RCNode>>() + nodes.capacity() * std::mem::size_of::<RCNode>();
        println!("RCNode size: {}Mb", node_size as f32 * 0.000001);

        let node_size2 = std::mem::size_of::<Vec<Node>>() + nodes.capacity() * std::mem::size_of::<Node>();
        println!("Node size: {}Mb", node_size2 as f32 * 0.000001);


        Self {
            grid,
            nodes,
        }
    }

    pub fn get_neighbours(&mut self, node: &RCNode) -> Vec<RCNode> {
        let mut neighbours = vec![];

        for (dx, dy) in SIGNS {
            if node.as_ref().borrow().index.idx_lat as i32 + dy < 0 || node.as_ref().borrow().index.idx_lon as i32 + dx < 0 {
                continue;
            }
            let idx = Index::new((node.as_ref().borrow().index.idx_lat as i32 + dy) as u32, (node.as_ref().borrow().index.idx_lon as i32 + dx) as u32);

            if self.valid(&idx) {
                let n = self.get_node(&idx);

                if !n.as_ref().borrow().visited {
                    neighbours.push(n);
                }
            }
        }

        neighbours
    }

    pub fn get_node(&mut self, idx: &Index) -> RCNode {
        let n_idx = (idx.idx_lon * N_LAT + idx.idx_lat) as usize;
        if self.nodes[n_idx].is_none() {
            self.nodes[n_idx] = Some(RCNode::new(RefCell::new(Node::new(Index::new(idx.idx_lat, idx.idx_lon)))));
            self.nodes[n_idx].as_ref().unwrap().borrow_mut().obstacle = self.grid.is_traversable(idx) == 0
        }
        Rc::clone(&self.nodes[n_idx].as_ref().unwrap())
    }

    pub fn is_obstacle(&mut self, idx: &Index) -> bool {
        self.get_node(idx).as_ref().borrow().obstacle
    }

    pub fn valid(&mut self, idx: &Index) -> bool {
        self.inside_map(idx) && !self.is_obstacle(idx)
    }

    pub fn inside_map(&self, idx: &Index) -> bool {
        return idx.idx_lat > 0 && idx.idx_lon > 0 && idx.idx_lat < N_LAT - 1 && idx.idx_lon < N_LON - 1
    }
}