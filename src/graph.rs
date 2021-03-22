use std::fmt;
use std::cmp::Ordering;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

use std::time::SystemTime;

use crate::board::Board;

type Distance = i16;

type Fifo = VecDeque<Board>;
type Adjacencies = HashMap<Board, Distance>;


// this is what will go into the BinaryHeap
#[derive(Debug, Clone)]
pub struct Path {
    previous: Option<Box<Path>>,
    board: Board,
    distance: Distance,
}
impl Path {
    pub fn unroll(&self) {
        let mut distance = 0;
        let mut nav = self;
        println!("{:02}-> {}", distance, self.board);
        while let Some(prev) = &nav.previous {
            distance += 1;
            print!("{:02}", distance);
            for _ in 0..distance {
                print!(" ");
            }
            if let Some(themove) = prev.board.move_from(&nav.board) {
                println!("-> {} ({:?})", prev.board, themove);
            }
            nav = &prev;
        }
    }
}
// so it fits the BinaryHeap requirements
impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}
impl Eq for Path {}
impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Path for board {} is at distance {}", self.board, self.distance)
    }
}


pub type Solutions = HashMap<Board, Path>;


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
    pub fn add_edge(&mut self, b1: Board, b2: Board, distance: Distance) {
        let adjacencies = self.vertices.entry(b1).or_insert(Adjacencies::new());
        let previous = adjacencies.entry(b2).or_insert(0);
        *previous += distance;
    }
    pub fn has_board(&self, board: &Board) -> bool {
        return self.vertices.contains_key(board);
    }
    // build the complete graph with 9!/2 vertices
    pub fn full_build() -> Graph {
        let mut graph = Graph::new();
        let mut fifo = Fifo::new();
        fifo.push_back(Board::zero());
        let mut proceed = true;
        while proceed {
            if let Some(scanned) = fifo.pop_front() {
                if graph.has_board(&scanned) {
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

    pub fn dijkstra(&self, start: &Board) -> Solutions {

        let clap = SystemTime::now();

        // the result is a hashmap board => Path
        let mut solutions = Solutions::new();
        // the queue of all edges between visited and not visited vertices
        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(
            Reverse(Path { previous: None, board: *start, distance: 0, }));

        let mut counter = 0;
        let mut proceed = true;
        while proceed {
            if let Some(Reverse(path)) = priority_queue.pop() {
                // make sure this edge is still relevant, i.e. that it corresponds
                // to an edge from a visited to an unvisited; the former is always true
                let visiting = path.board;
                if solutions.contains_key(&visiting) {
                    continue
                }
                // we can mark this board then
                solutions.insert(visiting, path.clone());
                counter += 1;
                // add all outgoing edges in queue
                // avoid the ones going back to visited nodes
                for neighbour in visiting.neighbours().iter() {
                    // do not bother if neighbour has already been marked
                    if solutions.contains_key(neighbour) {
                        continue
                    }
                    // xxx here the +1 should be found in the graph
                    // but our graph only has 1s as distances
                    let new_path = Path{
                        previous: Some(Box::new(path.clone())),
                        board: *neighbour,
                        distance: path.distance+1};
                    priority_queue.push(Reverse(new_path));
                }
            } else {
                proceed = false
            }
        }
        solutions
    }
}
impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "graph with {} vertices and {} edges",
            self.nb_vertices(), self.nb_edges())
    }
}

////////////////////////////////////////
#[cfg(test)]
mod test {
    use super::Graph;
    use crate::board::Board;

    #[test]
    fn test_basics() {
        let mut g = Graph::new();
        let start = Board::zero();
        for neighbour in start.neighbours().iter() {
            g.add_edge(start, *neighbour, 1);
        }
        assert_eq!(g.nb_vertices(), 1);
        assert_eq!(g.nb_edges(), 2);
    }

    #[test]
    fn test_full_build() {
        let full_graph = Graph::full_build();
        assert_eq!(full_graph.nb_vertices(), 181440);
        assert_eq!(full_graph.nb_edges(), 564480);
    }
}