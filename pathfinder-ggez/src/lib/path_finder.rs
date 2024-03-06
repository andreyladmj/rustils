use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;
use crate::lib::{MinHeap, Node, Point};
use crate::lib::grid::Grid;
use crate::lib::nodes::NodesMap;


const HEAP_SIZE: usize = 10000000;
const MAX_ITERATIONS: u32 = 10000000;

struct PathFinder<'a> {
    start_pos: Option<Point>,
    end_pos: Option<Point>,
    start_node: Option<Rc<Node>>,
    end_node: Option<Rc<Node>>,
    current_node: Option<Rc<Node>>,
    found: bool,
    iterations: u32,
    k: f32,
    vessel_speed_knots: f32,
    candidate_weights: HashMap<Rc<Node>, f32>,
    list_not_tested_nodes: MinHeap,
    nodes_map: &'a NodesMap,
    grid: &'a Grid,
}

impl PathFinder<'_> {
    pub fn new(nodes_map: &NodesMap, grid: &Grid) -> Self {

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

    // fn find_path(&mut self) {
    //     self.start_node = Some(self.nodes_map.get_node(&self.grid.get_index(self.start_pos.as_ref().unwrap())));
    //     self.end_node = Some(self.nodes_map.get_node(&self.grid.get_index(self.end_pos.as_ref().unwrap())));
    //     self.found = false;
    //     self.iterations = 0;
    //
    //     self.start_node.as_mut().unwrap().gscore = self.cost_fn(&self.start_node.as_ref().unwrap(), &self.end_node.as_ref().unwrap());
    //     self.start_node.as_mut().unwrap().fscore = 0.0;
    //
    //     self.end_node.as_mut().unwrap().parent = Some(Rc::copy(self.end_node.as_ref().unwrap()));
    //     self.start_node.as_mut().unwrap().parent = Some(Rc::copy(self.start_node.as_ref().unwrap()));
    //
    //     self.list_not_tested_nodes.clear();
    //     self.list_not_tested_nodes.insert(self.start_node.as_ref().unwrap());
    //
    //     self.candidate_weights.clear();
    //
    //     let start = Instant::now();
    //     self.search_end_node();
    //     let duration = start.elapsed();
    //     println!("Time elapsed in expensive_function() is: {:?}", duration);
    //
    // }
    //
    // fn iteration(&mut self) {
    //     self.iterations += 1;
    //
    //     if self.found {
    //         return;
    //     }
    //
    //     self.current_node = self.list_not_tested_nodes.get_min();
    //
    //     if self.current_node.as_ref().unwrap().visited {
    //         return;
    //     }
    //
    //     self.current_node.as_mut().unwrap().visited = true;
    //     let parent = self.current_node.as_ref().unwrap().parent.clone();
    //
    //
    //     println!("coparing ref: {}", self.current_node == parent);
    //     let neighbours = self.nodes_map.get_neighbours(self.current_node.as_ref().unwrap());
    //
    //     for neighbour in &neighbours {
    //         if neighbour.parent.is_some() && neighbour.parent == parent {
    //
    //         } else if self.has_candidate(neighbour) {
    //             let candidate_weight = self.candidate_weights[neighbour];
    //
    //             if candidate_weight != f32::INFINITY {
    //                 let (new_weight, update_parent) = self.check_availability_to_move(parent, self.current_node, neighbour);
    //
    //                 if candidate_weight > new_weight {
    //                     self.update_vertex(self.current_node, neighbour, new_weight, update_parent);
    //                 }
    //             }
    //         } else {
    //             if self.is_nodes_visible(self.current_node, neighbour) {
    //                 let (new_weight, update_parent) = self.check_availability_to_move(parent, self.current_node, neighbour);
    //                 self.update_vertex(self.current_node, neighbour, new_weight, update_parent);
    //             }
    //         }
    //
    //         if self.is_search_node_found(neighbour) {
    //             neighbour.parent = self.current_node.clone();
    //             self.end_node.as_mut().unwrap().parent = neighbour;
    //             self.found = true;
    //             return;
    //         }
    //     }
    // }
    //
    // fn search_end_node(&mut self) {
    //     while !self.list_not_tested_nodes.empty() && !self.found {
    //         self.iteration();
    //
    //         if self.iterations > MAX_ITERATIONS {
    //             println!("WARN: exceed length of algorithm");
    //             return;
    //         }
    //     }
    // }
    //
    // fn cost_fn(&self, node1: &Node, node2: &Node) -> f32 {
    //
    // }
    //
    // fn update_vertex(&mut self, current: Rc<Node>, neighbour: Rc<Node>, weight: f32, update_parent: bool) {
    //     self.update_weight(neighbour, weight);
    //
    //     if update_parent {
    //         neighbour.parent = current;
    //     } else {
    //         neighbour.parent = current.parent;
    //     }
    //
    //     neighbour.gscore = weight;
    //     neighbour.fscore = weight + self.k * self.cost_fn(&neighbour, &self.end_node);
    //
    //     self.list_not_tested_nodes.insert(neighbour);
    // }
    //
    // fn check_availability_to_move(&mut self, parent: &Node, current: &Node, neighbour: &Node) -> (f32, bool) {
    //     let mut new_weight: f32;
    //     let mut update_parent: bool;
    //
    //     if self.is_nodes_visible(parent, neighbour) {
    //         new_weight = parent.gscore + self.cost_fn(parent, neighbour);
    //         update_parent = false;
    //     } else {
    //         new_weight = parent.gscore + self.cost_fn(current, neighbour);
    //         update_parent = true;
    //     }
    //
    //     (new_weight, update_parent)
    // }
    //
    // fn update_weight(&mut self, node: &Node, weight: f32) {
    //     self.candidate_weights.insert(node, weight);
    // }
    //
    // fn has_candidate(&self, node: &Node) -> bool {
    //     self.candidate_weights.contains_key(node)
    // }
    //
    // fn is_search_node_found(&self, node: &Node) -> bool {
    //     self.is_nodes_visible(node, self.end_node);
    // }

}