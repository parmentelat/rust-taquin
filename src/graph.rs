use std::fmt;
use std::cmp::Ordering;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::BinaryHeap;

use crate::board::Board;


type Fifo = VecDeque<Board>;
type Adjacencies = HashMap<Board, i8>;



pub struct Graph {
    pub vertices: HashMap<Board, Adjacencies>,
}
impl Graph {
    pub fn new() -> Graph {
        return Graph{
            vertices: HashMap::new(),
        }
    }
    pub fn nb_vertices(&self) -> usize {
        return self.vertices.len()
    }
    pub fn nb_edges(&self) -> usize {
        let mut count = 0;
        for (_board, adjacencies) in self.vertices.iter() {
            count += adjacencies.len()
        }
        count
    }
    pub fn add_edge(&mut self, b1: Board, b2: Board, distance: i8) {
        let adjacencies = self.vertices.entry(b1).or_insert(Adjacencies::new());
        let previous = adjacencies.entry(b2).or_insert(0);
        *previous += distance;
    }
    pub fn is_visited(&self, board: &Board) -> bool {
        return self.vertices.contains_key(board);
    }
    // build the complete graph with 9!/2 vertices
    pub fn build() -> Graph {
        let mut graph = Graph::new();
        let mut fifo = Fifo::new();
        fifo.push_back(Board::zero());
        let mut proceed = true;
        while proceed {
            if let Some(scanned) = fifo.pop_front() {
                if graph.is_visited(&scanned) {
                    continue
                }
                for neighbour in scanned.neighbours() {
                    fifo.push_back(neighbour);
                    graph.add_edge(scanned, neighbour, 1);
                }
            } else {
                proceed = false
            }
        }
        graph
    }
}
impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "graph with {} vertices and {} edges",
            self.nb_vertices(), self.nb_edges())
    }
}
