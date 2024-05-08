use anyhow::Result;
use core::fmt;
use std::ops::{Add, AddAssign, Mul};

#[derive(Debug)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: fmt::Debug> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: impl Into<Vec<T>>) -> Self {
        Matrix {
            rows,
            cols,
            data: data.into(),
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // display a 2x3 as {1 2 3, 4 5 6}, 3x2 as {1 2, 3 4, 5 6}
        for i in 0..self.rows {
            write!(f, "{}", if i == 0 { "{" } else { " " })?;
            for j in 0..self.cols {
                write!(
                    f,
                    "{}{}",
                    if j == 0 { "" } else { " " },
                    self.data[i * self.cols + j]
                )?;
            }
            write!(f, "{}", if i == self.rows - 1 { "}" } else { ", " })?;
        }
        Ok(())
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.cols != b.rows {
        return Err(anyhow::anyhow!(
            "Cannot multiply {}x{} with {}x{}",
            a.rows,
            a.cols,
            b.rows,
            b.cols
        ));
    }

    let mut data = vec![T::default(); a.rows * b.cols];
    for i in 0..a.rows {
        for j in 0..b.cols {
            for k in 0..a.cols {
                data[i * b.cols + j] += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
        }
    }

    Ok(Matrix {
        rows: a.rows,
        cols: b.cols,
        data,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let a = Matrix::new(2, 3, [1, 2, 3, 4, 5, 6]);
        let b = Matrix::new(3, 2, [1, 2, 3, 4, 5, 6]);
        let c = multiply(&a, &b)?;
        assert_eq!(c.cols, 2);
        assert_eq!(c.rows, 2);
        assert_eq!(c.data, vec![22, 28, 49, 64]);
        assert_eq!(
            format!("{:?}", c),
            "Matrix { rows: 2, cols: 2, data: [22, 28, 49, 64] }"
        );

        Ok(())
    }

    #[test]
    fn test_matrix_display() -> Result<()> {
        let a = Matrix::new(2, 2, [1, 2, 3, 4]);
        let b = Matrix::new(2, 2, [1, 2, 3, 4]);
        let c = multiply(&a, &b)?;
        assert_eq!(c.data, vec![7, 10, 15, 22]);
        assert_eq!(format!("{}", c), "{7 10,  15 22}");
        Ok(())
    }
}
