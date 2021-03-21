use std::fmt;
use std::collections::HashMap;
use std::collections::VecDeque;

use crate::board::Board;

const DEBUG: bool = false;
const DEBUG_STEP: i32 = 10_000;

struct Fifo {
    contents: VecDeque<Board>,
}
impl Fifo {
    pub fn new() -> Fifo {
        return Fifo {
            contents: VecDeque::new(),
        }
    }
    pub fn insert(&mut self, b: Board) {
        self.contents.push_back(b)
    }
    pub fn retrieve(&mut self) -> Option<Board> {
        self.contents.pop_front()
    }
    pub fn len(&self) -> usize {
        return self.contents.len()
    }
}

pub struct Adjacencies {
    pub hash: HashMap<Board, i8>,
}
impl Adjacencies {
    fn new() -> Adjacencies {
        return Adjacencies{hash: HashMap::new()}
    }
    pub fn len(&self) -> usize {
        self.hash.len()
    }
}

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
        for (board, adjacencies) in self.vertices.iter() {
            count += adjacencies.len()
        }
        count
    }

    pub fn add_edge(&mut self, b1: Board, b2: Board, distance: i8) {
        let adjacencies = self.vertices.entry(b1).or_insert(Adjacencies::new());
        let previous = adjacencies.hash.entry(b2).or_insert(0);
        *previous += distance;
    }

    pub fn is_visited(&self, board: &Board) -> bool {
        return self.vertices.contains_key(board);
    }

    // build the complete graph with 9!/2 vertices
    pub fn build() -> Graph {
        let mut graph = Graph::new();
        let mut queue = Fifo::new();
        queue.insert(Board::zero());
        // debug only
        let mut debug_counter = 0;
        let mut proceed = true;
        while proceed {
            if let Some(scanned) = queue.retrieve() {
                if graph.is_visited(&scanned) {
                    continue
                }
                for neighbour in scanned.neighbours() {
                    debug_counter += 1;
                    queue.insert(neighbour);
                    graph.add_edge(scanned, neighbour, 1);
                    if DEBUG && (debug_counter % DEBUG_STEP) == 0 {
                        println!("{:?} -> {:?} graph={}v+{}e queue={} counter={}",
                             scanned, neighbour,
                             graph.nb_vertices(), graph.nb_edges(),
                             queue.len(), debug_counter);
                    }
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
