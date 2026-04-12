fn main() {
    let a = Matrix::<2, 3>::new();
    let b = Matrix::<3, 4>::new();
    let c = multiply(&a, &b);
    println!("{:?}", c);
}

#[derive(Debug)]
struct Matrix<const ROWS: usize, const COLS: usize> {
    data: [[f64; COLS]; ROWS],
}

impl<const ROWS: usize, const COLS: usize> Matrix<ROWS, COLS> {
    fn new() -> Self {
        Self {
            data: [[0.0; COLS]; ROWS],
        }
    }

    fn transpose(&self) -> Matrix<COLS, ROWS> {
        let mut result = Matrix::<COLS, ROWS>::new();
        for r in 0..ROWS {
            for c in 0..COLS {
                result.data[c][r] = self.data[r][c];
            }
        }
        result
    }
}

fn multiply<const M: usize, const N: usize, const P: usize>(
    a: &Matrix<M, N>,
    b: &Matrix<N, P>, // N must match!
) -> Matrix<M, P> {
    let mut result = Matrix::<M, P>::new();
    for r in 0..M {
        for c in 0..P {
            for k in 0..N {
                result.data[r][c] += a.data[r][k] * b.data[k][c];
            }
        }
    }
    result
}
