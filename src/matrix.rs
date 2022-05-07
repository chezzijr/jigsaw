use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    pub buf: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Default + Clone,
{
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            buf: vec![vec![T::default(); cols]; rows],
        }
    }

    pub fn bound(&self) -> (usize, usize) {
        (self.buf.len(), self.buf[0].len())
    }

    pub fn transpose(&mut self) -> &mut Self {
        let tmp = self.buf.clone();
        let (new_cols, new_rows) = self.bound();
        self.buf = vec![vec![T::default(); new_cols]; new_rows];
        for i in 0..new_rows {
            for j in 0..new_cols {
                self.buf[i][j] = tmp[j][i].clone();
            }
        }
        self
    }

    pub fn flip_hz(&mut self) -> &mut Self {
        self.buf.reverse();
        self
    }

    pub fn flip_vt(&mut self) -> &mut Self {
        self.buf.iter_mut().for_each(|s| s.reverse());
        self
    }

    pub fn rot90(&mut self) -> &mut Self {
        self.transpose().flip_vt()
    }

    pub fn rot180(&mut self) -> &mut Self {
        self.flip_hz().flip_vt()
    }
}

impl<T> Display for Matrix<T>
where
    T: Display + Into<bool> + Clone + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (_, col) = self.bound();
        let it = self.buf.iter().map(|c| {
            c.iter()
                .map(|e| if e.clone().into() { '🟫' } else { '🟨' })
                .fold(String::with_capacity(col), |mut acc, chr| {
                    acc.push(chr);
                    acc
                })
        });
        for i in it {
            writeln!(f, "\t{}", i)?;
        }

        Ok(())
    }
}
