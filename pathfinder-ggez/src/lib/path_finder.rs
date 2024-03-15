use std::collections::HashMap;
use std::time::Instant;
use crate::lib::{haversine_rad, Index, is_visible, MinHeap, Node, Point, RCNode};
use crate::lib::grid::Grid;
use crate::lib::nodes::NodesMap;


const HEAP_SIZE: usize = 10000000;
const MAX_ITERATIONS: u32 = 1000000;

pub struct PathFinder<'a> {
    start_pos: Option<Point>,
    end_pos: Option<Point>,
    start_node: Option<RCNode>,
    end_node: Option<RCNode>,
    pub current_node: Option<RCNode>,
    found: bool,
    iterations: u32,
    k: f32,
    vessel_speed_knots: f32,
    candidate_weights: HashMap<u32, f32>,
    list_not_tested_nodes: MinHeap,
    nodes_map: &'a mut NodesMap<'a>,
    grid: &'a Grid,
}

impl<'a> PathFinder<'a> {
    pub fn new(nodes_map: &'a mut NodesMap<'a>, grid: &'a Grid) -> Self {

        Self {
            start_pos: None,
            end_pos: None,
            start_node: None,
            end_node: None,
            current_node: None,
            found: false,
            iterations: 0,
            k: 1.0,
            vessel_speed_knots: 7.0,
            candidate_weights: HashMap::new(),
            list_not_tested_nodes: MinHeap::new(HEAP_SIZE),
            nodes_map: nodes_map,
            grid: grid,
        }
    }

    pub fn find(&mut self, start_point: Point, end_point: Point) {
        self.start_pos = Some(start_point);
        self.end_pos = Some(end_point);
        self.find_path();
    }

    fn find_path(&mut self) {
        self.start_node = Some(self.nodes_map.get_node(&self.grid.get_index(self.start_pos.as_ref().unwrap())));
        self.end_node = Some(self.nodes_map.get_node(&self.grid.get_index(self.end_pos.as_ref().unwrap())));
        self.found = false;
        self.iterations = 0;

        self.start_node.as_ref().unwrap().borrow_mut().gscore = self.cost_fn(self.start_node.as_ref().unwrap(), self.end_node.as_ref().unwrap());
        self.start_node.as_mut().unwrap().borrow_mut().fscore = 0.0;

        self.end_node.as_mut().unwrap().borrow_mut().parent = Some(self.end_node.as_ref().unwrap().clone());
        self.start_node.as_mut().unwrap().borrow_mut().parent = Some(self.start_node.as_ref().unwrap().clone());

        self.list_not_tested_nodes.clear();
        self.list_not_tested_nodes.insert(self.start_node.as_ref().unwrap().clone());

        self.candidate_weights.clear();

        let start = Instant::now();
        self.search_end_node();
        let duration = start.elapsed();
        println!("Time elapsed in find path is: {:?} at {} iterations", duration, self.iterations);

    }

    fn iteration(&mut self) {
        self.iterations += 1;

        if self.found {
            return;
        }

        self.current_node = self.list_not_tested_nodes.get_min();

        if self.current_node.as_ref().unwrap().borrow().visited {
            return;
        }

        self.current_node.as_mut().unwrap().borrow_mut().visited = true;
        let parent = self.current_node.as_ref().unwrap().borrow().parent.clone();


        let neighbours = self.nodes_map.get_neighbours(self.current_node.as_ref().unwrap());

        for neighbour in &neighbours {
            if neighbour.borrow().parent.is_some() && neighbour.borrow().parent == parent {

            } else if self.has_candidate(neighbour) {
                let candidate_weight = self.candidate_weights[&neighbour.as_ref().borrow().hash_idx()];

                if candidate_weight != f32::INFINITY {
                    let (new_weight, update_parent) = self.check_availability_to_move(parent.as_ref().unwrap(), self.current_node.as_ref().unwrap(), neighbour);

                    if candidate_weight > new_weight {
                        let n = self.current_node.clone().unwrap();
                        self.update_vertex(&n, neighbour, new_weight, update_parent);
                    }
                }
            } else {
                if self.is_nodes_visible(self.current_node.as_ref().unwrap(), neighbour) {
                    let (new_weight, update_parent) = self.check_availability_to_move(parent.as_ref().unwrap(), self.current_node.as_ref().unwrap(), neighbour);
                    let n = self.current_node.clone().unwrap();
                    self.update_vertex(&n, neighbour, new_weight, update_parent);
                }
            }

            if self.is_search_node_found(neighbour) {
                neighbour.borrow_mut().parent = self.current_node.clone();
                self.end_node.as_ref().unwrap().borrow_mut().parent = Some(neighbour.clone());
                self.found = true;
                return;
            }
        }
    }

    fn search_end_node(&mut self) {
        while !self.list_not_tested_nodes.empty() && !self.found {
            self.iteration();

            if self.iterations > MAX_ITERATIONS {
                println!("WARN: exceed length of algorithm");
                return;
            }
        }
    }

    fn cost_fn(&self, node1: &RCNode, node2: &RCNode) -> f32 {
        let p1 = self.grid.get_point(&node1.as_ref().borrow().index);
        let p2 = self.grid.get_point(&node2.as_ref().borrow().index);
        haversine_rad(p1.lat, p1.lon, p2.lat, p2.lon)
    }

    fn update_vertex(&mut self, current: &RCNode, neighbour: &RCNode, weight: f32, update_parent: bool) {
        self.update_weight(neighbour, weight);

        if update_parent {
            neighbour.as_ref().borrow_mut().parent = Some(current.clone());
        } else {
            neighbour.as_ref().borrow_mut().parent = current.as_ref().borrow().parent.clone();
        }

        neighbour.as_ref().borrow_mut().gscore = weight;
        neighbour.as_ref().borrow_mut().fscore = weight + self.k * self.cost_fn(&neighbour, &self.end_node.as_ref().unwrap());

        self.list_not_tested_nodes.insert(neighbour.clone());
    }

    fn check_availability_to_move(&self, parent: &RCNode, current: &RCNode, neighbour: &RCNode) -> (f32, bool) {
        let mut new_weight: f32;
        let mut update_parent: bool;

        if self.is_nodes_visible(parent, neighbour) {
            new_weight = parent.as_ref().borrow().gscore + self.cost_fn(parent, neighbour);
            update_parent = false;
        } else {
            new_weight = parent.as_ref().borrow().gscore + self.cost_fn(current, neighbour);
            update_parent = true;
        }

        (new_weight, update_parent)
    }

    fn update_weight(&mut self, node: &RCNode, weight: f32) {
        self.candidate_weights.insert(node.as_ref().borrow().hash_idx(), weight);
    }

    fn has_candidate(&self, node: &RCNode) -> bool {
        self.candidate_weights.contains_key(&node.as_ref().borrow().hash_idx())
    }

    fn is_search_node_found(&self, node: &RCNode) -> bool {
        self.is_nodes_visible(node, self.end_node.as_ref().unwrap())
    }

    fn is_nodes_visible(&self, node1: &RCNode, node2: &RCNode) -> bool {
        is_visible(&node1.as_ref().borrow().index, &node2.as_ref().borrow().index, self.grid)
    }

    pub fn get_path(&self) -> Vec<Index> {
        let mut path = vec![];
        let mut current_node = self.current_node.clone();
        while current_node.is_some() && current_node.as_ref().unwrap().borrow().parent != current_node {
            let node = current_node.as_ref().unwrap().clone();
            path.push(node.borrow().index.clone());
            current_node = current_node.clone().as_ref().unwrap().borrow().parent.clone();
        }
        let node = current_node.as_ref().unwrap().clone();
        path.push(node.borrow().index.clone());
        path
    }
}