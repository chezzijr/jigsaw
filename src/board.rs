use crate::{matrix::Matrix, polyomino::Polyomino};
use std::{fs, path::Path, time::Instant};
#[derive(Debug)]
pub struct Board {
    pub polyominoes: Vec<Polyomino>,
    pub size: (usize, usize),

    // The board represent in 2d vector
    // true = occupied
    pub b: Matrix<bool>,
    pub solution_found: bool,
    pub call_stack: Vec<Polyomino>,
    pub min_poliomino_size: usize,
    pub solution: Option<Vec<Polyomino>>,
}

impl Board {
    pub fn new(size: (usize, usize), polyominoes: Vec<Polyomino>) -> Self {
        let (rows, cols) = size;
        let min_size = polyominoes
            .iter()
            .min_by(|x, y| x.size.cmp(&y.size))
            .unwrap()
            .size;
        Self {
            call_stack: Vec::with_capacity(polyominoes.len()),
            size,
            polyominoes,
            b: Matrix::new(rows, cols),
            solution_found: false,
            min_poliomino_size: min_size,
            solution: None,
        }
    }

    pub fn placeable(&self, pos: (usize, usize), pol: &mut Polyomino) -> bool {
        let (row, col) = pos;
        let (brows, bcols) = self.size;
        let (prows, pcols) = pol.shape.bound();
        if row + prows > brows || col + pcols > bcols {
            return false;
        }

        for r in 0..prows {
            for c in 0..pcols {
                if self.b.buf[row + r][col + c] && pol.shape.buf[r][c] {
                    return false;
                }
            }
        }

        true
    }

    pub fn place(&mut self, pos: (usize, usize), pol: &mut Polyomino) {
        let (row, col) = pos;
        let (prows, pcols) = pol.shape.bound();
        pol.start = pos;
        for r in 0..prows {
            for c in 0..pcols {
                self.b.buf[row + r][col + c] = self.b.buf[row + r][col + c] || pol.shape.buf[r][c]
            }
        }
    }

    pub fn unplace(&mut self, pol: &mut Polyomino) {
        let (row, col) = pol.start;
        let (prows, pcols) = pol.shape.bound();
        pol.start = (usize::MAX, usize::MAX);
        for r in 0..prows {
            for c in 0..pcols {
                if pol.shape.buf[r][c] {
                    self.b.buf[row + r][col + c] = false;
                }
            }
        }
    }

    // If the board was divided into many empty areas
    // There may exists an area less than the area of the smallest polyomino
    // Which results in non-placeable area
    pub fn has_unfillable_space(&self) -> bool {
        let mut mask = self.b.buf.clone();
        for y in 0..self.size.0 {
            for x in 0..self.size.1 {
                if !mask[y][x] {
                    let mut blocked_of_region_size: usize = 1;
                    mask[y][x] = true;
                    let mut to_check = Vec::new();

                    for (dy, dx) in [(1, 0), (-1i32, 0), (0, 1), (0, -1i32)] {
                        let new_y = y as i32 + dy;
                        let new_x = x as i32 + dx;
                        if (0 <= new_y && new_y < self.size.0 as i32)
                            && (0 <= new_x && new_x < self.size.1 as i32)
                        {
                            if !mask[new_y as usize][new_x as usize] {
                                mask[new_y as usize][new_x as usize] = true;
                                to_check.push((new_y as usize, new_x as usize));
                                blocked_of_region_size += 1;
                            }
                        }
                    }

                    while to_check.len() != 0 {
                        let mut new_to_check = Vec::new();
                        for (sy, sx) in to_check {
                            for (dy, dx) in [(1, 0), (-1i32, 0), (0, 1), (0, -1i32)] {
                                let new_y = sy as i32 + dy;
                                let new_x = sx as i32 + dx;
                                if (0 <= new_y && new_y < self.size.0 as i32)
                                    && (0 <= new_x && new_x < self.size.1 as i32)
                                {
                                    if !mask[new_y as usize][new_x as usize] {
                                        mask[new_y as usize][new_x as usize] = true;
                                        new_to_check.push((new_y as usize, new_x as usize));
                                        blocked_of_region_size += 1;
                                    }
                                }
                            }
                        }

                        to_check = new_to_check;
                    }

                    if blocked_of_region_size < self.min_poliomino_size {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn check(&self) -> bool {
        for r in self.b.buf.iter() {
            for e in r.iter() {
                if !e {
                    return false;
                }
            }
        }
        true
    }

    pub fn find_solution(&mut self) -> Option<&Vec<Polyomino>> {
        let now = Instant::now();
        self.solve(0);
        let elapsed_time = now.elapsed();

        match self.solution {
            Some(_) => println!("Solution found after {}s", elapsed_time.as_secs_f32()),
            None => println!("No solution found"),
        };

        self.solution.as_ref()
    }

    pub fn solve(&mut self, index: usize) {
        if self.check() || self.call_stack.len() == self.polyominoes.len() {
            self.solution_found = true;
            self.solution = Some(self.call_stack.clone());
            return;
        }

        if !self.solution_found {
            if !self.has_unfillable_space() {
                let mut fo = self.polyominoes[index].feasible_orientations();
                for mut orientation in fo.iter_mut() {
                    for row in 0..self.size.0 {
                        for col in 0..self.size.1 {
                            if self.placeable((row, col), &mut orientation) {
                                self.place((row, col), orientation);
                                self.call_stack.push(orientation.clone());
                                self.solve(index + 1);
                                self.call_stack.pop();
                                self.unplace(orientation);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn write_solutions_to_file<T>(&self, filename: T) -> Result<(), std::io::Error>
    where
        T: AsRef<Path>,
    {
        match self.solution.as_ref() {
            Some(val) => {
                let content = val
                    .iter()
                    .map(|i| format!("{}", i))
                    .reduce(|acc, i| acc + i.as_ref())
                    .unwrap();
                fs::write(filename, content)
            }
            None => panic!("Cannot write to file since there is no solution has been found"),
        }
    }
}
