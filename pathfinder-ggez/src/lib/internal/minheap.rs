use std::rc::Rc;
use crate::lib::Node;

#[derive(Debug)]
pub struct MinHeap {
    nodes: Vec<Option<Rc<Node>>>,
    capacity: usize,
    heap_size: usize
}

impl MinHeap {
    pub fn new(cap: usize) -> Self {
        Self {
            nodes: vec![None; cap],
            capacity: cap,
            heap_size: 0,
        }
    }

    fn parent(&self, i: usize) -> usize {
        return (i-1) / 2;
    }
    fn left(&self, i: usize) -> usize {
        return 2*i + 1;
    }
    fn right(&self, i: usize) -> usize {
        return 2*i + 2;
    }
    fn swap(&mut self, i1: usize, i2: usize) {
        self.nodes.swap(i1, i2);
    }
    fn heapify(&mut self, i: usize) {
        let l = self.left(i);
        let r = self.right(i);
        let mut smallest = i;

        if l < self.heap_size && self.nodes[l].as_ref().unwrap().fscore < self.nodes[i].as_ref().unwrap().fscore {
            smallest = l;
        }
        if r < self.heap_size && self.nodes[r].as_ref().unwrap().fscore < self.nodes[smallest].as_ref().unwrap().fscore {
            smallest = r;
        }

        if smallest != i {
            self.swap(i, smallest);
            self.heapify(smallest);
        }
    }
    pub fn empty(&self) -> bool {
        self.heap_size == 0
    }
    pub fn clear(&mut self) {
        self.heap_size = 0
    }
    fn front() {

    }

    pub fn insert(&mut self, node: &Rc<Node>) {
        if self.heap_size == self.capacity {
            self.heap_size -= 1;
        }

        self.heap_size += 1;
        let mut i = self.heap_size - 1;
        self.nodes[i] = Some(Rc::copy(node));

        while i != 0 && self.nodes[self.parent(i)].as_ref().unwrap().fscore > self.nodes[i].as_ref().unwrap().fscore {
            self.swap(i, self.parent(i));
            i = self.parent(i);
        }
    }

    pub fn get_min(&mut self) -> Option<Rc<Node>> {
        if self.heap_size == 0 {
            return None
        }

        if self.heap_size == 1 {
            self.heap_size -= 1;
            return self.nodes[0].clone()
        }

        let root = self.nodes[0].clone();
        self.nodes.swap(0, self.heap_size - 1);
        // self.nodes[0] = self.nodes[self.heap_size - 1];
        self.heap_size -= 1;
        self.heapify(0);
        return root
    }
}