use std::fmt::{self, Display, Formatter};

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
// `Vec` will allocate memory through `GLOBAL` above

/// Value type for matrix
type Value = i64;

#[derive(Debug, PartialEq)]
pub(crate) struct Matrix {
    row: usize,
    col: usize,

    inner: Vec<Value>,
}

impl Matrix {
    pub(crate) fn new(row: usize, col: usize) -> Self {
        Matrix {
            row,
            col,
            // fill with 0
            inner: vec![Value::default(); row * col],
        }
    }
    /// for debug
    pub(crate) fn serial_new(row: usize, col: usize) -> Self {
        let mut matrix = Self::new(row, col);

        let mut num: Value = 0;
        for j in 0..col {
            for i in 0..row {
                matrix.insert(i, j, num);
                num += 1;
            }
        }

        matrix
    }
    /// for benchmark
    pub(crate) fn random_new(row: usize, col: usize) -> Self {
        Matrix {
            row,
            col,
            inner: fake::vec![Value; row * col],
        }
    }

    #[inline]
    pub(crate) fn get(&self, i: usize, j: usize) -> Value {
        self.inner[j * self.row + i]
    }
    #[inline]
    pub(crate) fn get_ref_mut(&mut self, i: usize, j: usize) -> &mut Value {
        &mut self.inner[j * self.row + i]
    }
    #[inline]
    pub(crate) fn insert(&mut self, i: usize, j: usize, val: Value) {
        self.inner[j * self.row + i] = val;
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for j in 0..self.col {
            for i in 0..self.row {
                write!(f, "{} ", self.get(i, j))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
