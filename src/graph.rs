pub mod graph {

    use std::collections::HashMap;
    use std::fmt;

    use crate::board::board::Board;

    // non generic for now
    // here <T> will be Board
    // it will thus require the Hash and Eq traits

    pub struct Adjacencies {
        pub hash: HashMap<Board, i8>,
    }
    impl Adjacencies {
        fn new() -> Adjacencies {
            return Adjacencies{hash: HashMap::new(),}
        }
    }

    pub struct Graph {
        pub vertices: HashMap<Board, Adjacencies>,
    }

    impl Graph {
        pub fn add_edge(&mut self, b1: Board, b2: Board, distance: i8) {
            let adjacencies = self.vertices.entry(b1).or_insert(Adjacencies::new());
            let previous = adjacencies.hash.entry(b2).or_insert(0);
            *previous += distance;
        }
    }

    impl fmt::Display for Graph {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "graph with {} vertices", self.vertices.len())
        }
    }

}
