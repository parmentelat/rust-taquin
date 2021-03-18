use std::env;

use taquin::Board;

fn main() {
    let argv: Vec<String> = env::args().collect();
    println!("{:?}", argv);

    let b = Board::zero();
    print!("zero is\n{}", b);

    for n in b.neighbours().iter() {
        print!("neighbour\n{}", n);
    }

    for (index, filename) in argv.iter().enumerate() {
        if index == 0 {
            continue
        }
        let parsed = Board::from_file(filename);
        print!("from file {} is\n{}", filename, parsed);

        let solvable = parsed.solvable();
        println!("solvable ? {}", solvable);
    }
}