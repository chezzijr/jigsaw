mod board;
mod matrix;
mod polyomino;

use board::Board;
use polyomino::Polyomino;

fn main() {
    let polyominoes = vec![
        Polyomino::new("I", vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]),
        Polyomino::new("X", vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        Polyomino::new("F", vec![(0, 0), (1, 0), (1, 1), (1, 2), (2, 1)]),
        Polyomino::new("U", vec![(0, 0), (1, 0), (0, 1), (0, 2), (1, 2)]),
        Polyomino::new("T", vec![(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)]),
        Polyomino::new("Y", vec![(0, 0), (1, 0), (2, 0), (3, 0), (2, 1)]),
        Polyomino::new("W", vec![(0, 0), (1, 0), (1, 1), (2, 1), (2, 2)]),
        Polyomino::new("L", vec![(0, 0), (1, 0), (2, 0), (3, 0), (3, 1)]),
        Polyomino::new("V", vec![(0, 0), (0, 1), (0, 2), (1, 0), (2, 0)]),
        Polyomino::new("Z", vec![(0, 0), (0, 1), (1, 1), (2, 1), (2, 2)]),
        Polyomino::new("P", vec![(0, 0), (0, 1), (0, 2), (1, 1), (1, 2)]),
        Polyomino::new("N", vec![(0, 0), (0, 1), (0, 2), (1, 2), (1, 3)]),
    ];

    let mut b = Board::new((10, 6), polyominoes);
    match b.find_solution() {
        Some(_) => b
            .write_solutions_to_file("solution.txt")
            .expect("Deez nuts"),
        None => (),
    };
}
