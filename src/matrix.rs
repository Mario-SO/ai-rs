use rand::{thread_rng, Rng};

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn rand_matrix(rows: usize, cols: usize) -> Matrix {
        let mut rng = thread_rng();

        let mut res: Matrix = Matrix::zeros(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                res.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }

        res
    }

    pub fn multiply(&mut self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("Columns of A must match rows of B, otherwiase they can't be multiplied");
        }

        let mut res: Matrix = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }
                res.data[i][j] = sum;
            }
        }
        res
    }

    pub fn add(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Columns and rows of A must match columns and rows of B, otherwiase they can't be added");
        }

        let mut res: Matrix = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        res
    }

    pub fn dot_mul(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Columns and rows of A must match columns and rows of B, otherwiase they can't be dot multiplied");
        }

        let mut res: Matrix = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }
        res
    }

    pub fn substract(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Columns and rows of A must match columns and rows of B, otherwiase they can't be added");
        }

        let mut res: Matrix = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        res
    }
}
