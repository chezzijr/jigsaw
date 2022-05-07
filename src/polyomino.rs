use std::fmt::Display;

use crate::matrix::Matrix;

#[derive(Debug, Clone, PartialEq)]
pub struct Polyomino {
    pub name: String,
    pub shape: Matrix<bool>,
    pub size: usize,
    pub start: (usize, usize),
}

impl Polyomino {
    pub fn new<T>(name: T, pos: Vec<(usize, usize)>) -> Self
    where
        T: Into<String>,
    {
        let rows = pos.iter().max_by_key(|e| e.0).unwrap().0 + 1;
        let cols = pos.iter().max_by_key(|e| e.1).unwrap().1 + 1;
        let size = pos.len();
        let mut s: Matrix<bool> = Matrix::new(rows, cols);
        for (i, j) in pos.iter() {
            s.buf[*i][*j] = true;
        }
        Self {
            name: name.into(),
            size,
            shape: s,
            start: (usize::MAX, usize::MAX),
        }
    }

    pub fn feasible_orientations(&mut self) -> Vec<Self> {
        // Rot0
        let origin = self.clone();

        // Rot90
        let mut rot90 = self.clone();
        rot90.rot90();

        // Rot180
        let mut rot180 = self.clone();
        rot180.rot180();

        // Rot270
        let mut rot270 = self.clone();
        rot270.rot270();

        // Flip
        let mut flip = self.clone();
        flip.flip();

        // Flip + Rot90
        let mut flip_rot90 = self.clone();
        flip_rot90.flip().rot90();

        // Flip + Rot180
        let mut flip_rot180 = self.clone();
        flip_rot180.flip().rot180();

        // Flip + Rot270
        let mut flip_rot270 = self.clone();
        flip_rot270.flip().rot270();

        let mut orientations = vec![
            origin,
            rot90,
            rot180,
            rot270,
            flip,
            flip_rot90,
            flip_rot180,
            flip_rot270,
        ];

        // Cleaning up duplicate orientation
        orientations.dedup_by(|a, b| a == b);
        orientations
    }

    pub fn flip(&mut self) -> &mut Self {
        self.shape.flip_hz();
        self
    }

    pub fn rot90(&mut self) -> &mut Self {
        self.shape.rot90();
        self
    }

    pub fn rot180(&mut self) -> &mut Self {
        self.shape.rot180();
        self
    }

    pub fn rot270(&mut self) -> &mut Self {
        self.shape.rot180().rot90();
        self
    }
}

impl Display for Polyomino {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Start: {:?}", self.start)?;
        writeln!(f, "Shape:")?;
        self.shape.fmt(f)
    }
}
