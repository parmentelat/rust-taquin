use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;

struct Board {
    implem: [i8; 9],
}

impl Board {
    fn zero() -> Board {
        return Board{
            implem: [1, 2, 3, 4, 5, 6, 7, 8, 0],
        }
    }

    fn i_to_xy(index: i8) -> [i8; 2] {
        return [index % 3, index / 3]
    }
    fn xy_to_i(x: i8, y:i8) -> i8 {
        return 3*y + x
    }
    fn hole(&self) -> i8 {
        for i in 0..9 {
            if self.implem[i] == 0 {
                return i as i8
            }
        }
        return -1 as i8
    }

    fn swap(&self, i: i8, j: i8) -> Board {
        let mut implem2 = self.implem.clone();
        implem2[i as usize] = self.implem[j as usize];
        implem2[j as usize] = self.implem[i as usize];
        return Board{implem: implem2};
    }

    fn from_file(filename: &str) -> Board {
        const RADIX: u32 = 10;
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut implem: [i8; 9] = [0; 9];
        let mut counter: usize = 0;
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            for char in line.chars() {
                if '0' <= char && char <= '8' {
                    implem[counter] = char.to_digit(RADIX).unwrap() as i8;
                    counter += 1;
                }
            }

        }
        return Board{implem: implem};
    }

    fn neighbours(&self) -> Vec<Board> {
        let hole = self.hole();
        let [hx, hy] = Board::i_to_xy(self.hole());
        let mut result: Vec<Board> = Vec::new();
        for dx in [-1, 0, 1].iter() {
            for dy in [-1, 0, 1].iter() {
                if (dx+dy == 1) || (dx+dy == -1) {
                    // potential new hole: 4 directions
                    let nhole = Board::xy_to_i(hx+dx, hy+dy);
                    // may still be outside
                    if 0 <= nhole && nhole < 9 {
                        result.push(self.swap(hole, nhole));
                    }
                }
            }
        }
        return result;
    }

    fn solvable(&self) -> bool {
        let [hx, hy] = Board::i_to_xy(self.hole());
        let expected_parity = ((hx + hy) % 2) == 0;

        let size = 9;
        let mut swaps = 0;
        for i in 0..size {
            for j in i+1..size {
                if self.implem[j] < self.implem[j] {
                    swaps += 1;
                }
            }
        }
        let actual_parity = (swaps % 2) == 0;

        return expected_parity == actual_parity;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
/*        for l in 0..2 {
            for c in 0..2 {
                write!(f, "{} ", self.implem[3*l+c])
            }
        }
*/
        write!(f, "{} {} {}\n{} {} {}\n{} {} {}\n",
            self.implem[0], self.implem[1], self.implem[2],
            self.implem[3], self.implem[4], self.implem[5],
            self.implem[6], self.implem[7], self.implem[8])
    }
}

use std::env;

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