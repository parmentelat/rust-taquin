use std::fmt;
use std::collections::HashSet;

use crate::board::Board;
use crate::graph::Graph;

pub struct Dijkstra {
    graph: Graph,
    visited: HashSet<Board>,
}

impl Dijkstra {

    pub fn new() -> Dijkstra {
        return Dijkstra {
            graph: Graph::new(),
            visited: HashSet::new(),
        }
    }

    pub fn scan(&mut self) {

        let mut counter = 0;

        fn _scan(board: Board, graph: &mut Graph,
                 visited: &mut HashSet<Board>, mut counter: usize) {
            counter += 1;
            println!("scanning {}-th board\n{}", counter, board);
            visited.insert(board);
            for next in board.neighbours() {
                if visited.contains(&next) {
                    continue
                }
                graph.add_edge(board, next, 1);
                _scan(next, graph, visited, counter);
            }
        }
        let start = Board::zero();
        _scan(start, &mut self.graph, &mut self.visited, counter);
    }
}

impl std::fmt::Display for Dijkstra {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nb_vertices = self.visited.len();
        write!(f, "Full graph has {} boards", nb_vertices)
    }
}
