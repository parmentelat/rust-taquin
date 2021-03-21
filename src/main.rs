// use std::env;

use taquin::graph::Graph;
//use taquin::dijkstra::Dijkstra;

fn main() {
    let full_graph = Graph::full_build();
    println!("{}", full_graph);
}


// use taquin::board::Board;
// use taquin::graph::Graph;

// fn main() {
//     let argv: Vec<String> = env::args().collect();
//     println!("{:?}", argv);

//     let b = Board::zero();
//     print!("zero is\n{}", b);

//     for n in b.neighbours().iter() {
//         print!("neighbour\n{}", n);
//     }

//     for (index, filename) in argv.iter().enumerate() {
//         if index == 0 {
//             continue
//         }
//         let board = Board::from_file(filename);
//         print!("from file {} is\n{}", filename, board);

//         let solvable = board.solvable();
//         println!("solvable ? {}", solvable);

//         use std::collections::HashMap;

//         let mut g = Graph::new();

//         for next in board.neighbours().iter() {
//             g.add_edge(board, *next, 1);
//         }
//         println!("{}", g)
//     }
// }
