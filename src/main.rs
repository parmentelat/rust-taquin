use std::env;

use taquin::board::Board;
use taquin::graph::{Graph, histogram};


fn main() {

    let argv: Vec<String> = env::args().collect();

    let full_graph = Graph::full_build();
    println!("{}", full_graph);

    let zero = Board::zero();
    let solutions = full_graph.dijkstra(&zero);
    let max = solutions.len();

    // sort the histogram by distance
    let histo = histogram(&solutions);
    let mut items: Vec<_> = histo.into_iter().collect();
    items.sort_by(|x, y| x.0.cmp(&y.0));
    print!("full histogram:");
    for (distance, occurrences) in items.iter() {
        print!(" {} : {},", distance, occurrences)
    }
    println!();

    for (index, filename) in argv.iter().enumerate() {
        if index == 0 {
            continue;
        }
        println!("{}", filename);
        let board = Board::from_file(filename);
        if let Some(path) = solutions.get(&board) {
            println!("file {}: {} - found board {} -> {}", 
                     index, filename, board, path);
            path.unroll();
        } else {
            println!("file {}: {} - unreachable board {}", index, filename, board);
        }
    }
}
