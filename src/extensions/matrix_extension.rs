use itertools::Itertools;
use nalgebra::{DMatrix, Matrix};
use prettytable::{Cell, Row, Table};

pub trait MatrixExtension<T, R, C, S> {
    fn printstd(&self);
}

impl<T, R, C, S> MatrixExtension<T, R, C, S> for Matrix<T, R, C, S>
where
    T: std::fmt::Display + std::fmt::Debug + Copy,
    R: nalgebra::Dim,
    C: nalgebra::Dim,
    S: nalgebra::storage::Storage<T, R, C>,
{
    fn printstd(&self) {
        let mut table = Table::new();
        for i in 0..self.nrows() {
            let mut row = vec![];
            for j in 0..self.ncols() {
                let cell = self[(i, j)];
                row.push(Cell::new(
                    &(cell
                        .to_string()
                        .chars()
                        .pad_using(10, |_| ' ')
                        .collect::<String>()),
                ));
            }
            table.add_row(Row::new(row));
        }
        table.printstd();
    }
}

pub trait Purifiable {
    fn purify(&self) -> Self;
}

impl Purifiable for DMatrix<f32> {
    fn purify(&self) -> Self {
        let mut matrix = self.clone();
        for i in 0..matrix.nrows() {
            for j in 0..matrix.ncols() {
                let value = matrix[(i, j)];
                if i / 3 == j / 3 || value.abs() < 0.00001f32 {
                    matrix[(i, j)] = 0.0f32;
                }
            }
        }
        matrix
    }
}
