use std::rc::Rc;
use crate::lib::grid::Grid;
use crate::lib::{Index, Node};
use crate::{N_LAT, N_LON};

pub struct NodesMap {
    nodes: Vec<Rc<Node>>
}

const SIGNS: [(i32, i32); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (1, 1), (1, -1), (-1, 1), (-1, -1)];


impl NodesMap {
    pub fn new(grid: &Grid) -> Self {
        let mut nodes = Vec::with_capacity((N_LAT * N_LON) as usize);

        for x in 0..N_LON {
            for y in 0..N_LAT {
                let mut n = Node::new(Index::new(y, x));
                n.obstacle = grid.is_traversable(&n.index) != 0;
                nodes.push(Rc::new(n));
            }
        }


        let node_size = std::mem::size_of::<Vec<Rc<Node>>>() + nodes.capacity() * std::mem::size_of::<Rc<Node>>();
        // let node_size = std::mem::size_of_val(&nodes);
        println!("nodes size: {}b", node_size);
        println!("nodes size: {}Mb", node_size as f32 * 0.000001);

        let node_size2 = std::mem::size_of::<Vec<Node>>() + nodes.capacity() * std::mem::size_of::<Node>();
        // let node_size = std::mem::size_of_val(&nodes);
        println!("nodes size: {}b", node_size2);
        println!("nodes size: {}Mb", node_size2 as f32 * 0.000001);
        // println!("nodes size: {}Mb", nodes.len() as f32 * node_size as f32 * 0.000001);
        // land points: 79600971, its: 34.1225%
        // size of node: 32b
        // size of whole nodes: 7464.96Mb
        // nodes size: 24
        // nodes size: 5598.72


        Self {
            nodes,
        }
    }

    pub fn get_neighbours(&self, node: &Node) -> Vec<Rc<Node>> {
        let mut neighbours = vec![];

        for (dx, dy) in SIGNS {
            if node.index.idx_lat as i32 + dy < 0 || node.index.idx_lon as i32 + dx < 0 {
                continue;
            }
            let idx = Index::new((node.index.idx_lat as i32 + dy) as u32, (node.index.idx_lon as i32 + dx) as u32);
            let n = self.get_node(&idx);

            if self.valid(&idx) && !n.visited {
                neighbours.push(n);
            }
        }

        neighbours
    }

    pub fn get_node(&self, idx: &Index) -> Rc<Node> {
        Rc::clone(&self.nodes[(idx.idx_lon * N_LAT + idx.idx_lat) as usize])
    }

    pub fn is_obstacle(&self, idx: &Index) -> bool {
        self.get_node(idx).obstacle
    }

    pub fn valid(&self, idx: &Index) -> bool {
        self.inside_map(idx) && !self.is_obstacle(idx)
    }

    pub fn inside_map(&self, idx: &Index) -> bool {
        return idx.idx_lat > 0 && idx.idx_lon > 0 && idx.idx_lat < N_LAT - 1 && idx.idx_lon < N_LON - 1
    }
}