use std::ops::{Index, IndexMut, Mul, Sub};

use approx_eq::ApproxEq;

use crate::tuples::Tuple;

#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        Self { data }
    }

    pub fn zero(size: usize) -> Self {
        Self::new(vec![vec![0.0; size]; size])
    }

    pub fn identity(size: usize) -> Self {
        let mut id = Self::zero(size);
        for i in 0..size {
            id[(i, i)] = 1.0;
        }
        id
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn transpose(&self) -> Self {
        let mut m = Matrix::zero(self.size());
        for row in 0..self.size() {
            for col in 0..self.size() {
                m[(col, row)] = self[(row, col)];
            }
        }
        m
    }

    pub fn determinant(&self) -> f64 {
        if self.size() == 2 {
            self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
        } else {
            let mut det = 0.0;
            for col in 0..self.size() {
                det += self[(0, col)] * self.cofactor(0, col);
            }
            det
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let submatrix_size = self.size() - 1;
        let mut values: Vec<f64> = Vec::new();
        for r in 0..self.size() {
            if r != row {
                for c in 0..self.size() {
                    if c != col {
                        values.push(self[(r, c)]);
                    }
                }
            }
        }
        Matrix::from(values, submatrix_size)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        } else {
            -self.minor(row, col)
        }
    }

    pub fn inverse(&self) -> Option<Matrix> {
        let det = self.determinant();
        if det == 0.0 {
            None
        } else {
            let mut cofactors: Vec<f64> = Vec::new();
            for row in 0..self.size() {
                for col in 0..self.size() {
                    cofactors.push(self.cofactor(row, col) / det);
                }
            }
            let cofactor_matrix = Matrix::from(cofactors, self.size()).transpose();
            Some(cofactor_matrix)
        }
    }

    pub fn from(values: Vec<f64>, size: usize) -> Matrix {
        let data = values
            .chunks(size)
            .map(|r| {
                let mut row = Vec::new();
                for val in r {
                    row.push(*val);
                }
                row
            })
            .collect::<Vec<_>>();
        Matrix::new(data)
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.data.get(index.0).unwrap().get(index.1).unwrap()
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.data
            .get_mut(index.0)
            .unwrap()
            .get_mut(index.1)
            .unwrap()
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..self.data.len() {
            for col in 0..self.data.len() {
                if !self[(row, col)].approx_eq(other[(row, col)]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Sub for &Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut m: Vec<Vec<f64>> = Vec::new();
        let mut r: Vec<f64> = Vec::new();
        for row in 0..self.size() {
            for col in 0..self.size() {
                r.push(self[(row, col)] - rhs[(row, col)]);
            }
            m.push(r.clone());
        }
        Matrix::new(m)
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = Matrix::zero(self.size());
        for row in 0..self.size() {
            for col in 0..self.size() {
                let mut sum = 0.0;
                for i in 0..self.size() {
                    sum += self[(row, i)] * rhs[(i, col)];
                }
                m[(row, col)] = sum;
            }
        }
        m
    }
}

impl<T> Mul<&T> for &Matrix
where
    T: Tuple,
{
    type Output = T;

    fn mul(self, rhs: &T) -> Self::Output {
        Self::Output::new(
            self[(0, 0)] * rhs.x()
                + self[(0, 1)] * rhs.y()
                + self[(0, 2)] * rhs.z()
                + self[(0, 3)] * rhs.w(),
            self[(1, 0)] * rhs.x()
                + self[(1, 1)] * rhs.y()
                + self[(1, 2)] * rhs.z()
                + self[(1, 3)] * rhs.w(),
            self[(2, 0)] * rhs.x()
                + self[(2, 1)] * rhs.y()
                + self[(2, 2)] * rhs.z()
                + self[(2, 3)] * rhs.w(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::points::Point;

    use super::*;

    #[test]
    fn matrix_can_be_built_and_inspected() {
        let matrix = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.6],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(matrix[(0, 0)], 1.0);
        assert_eq!(matrix[(0, 3)], 4.0);
        assert_eq!(matrix[(1, 0)], 5.5);
        assert_eq!(matrix[(1, 2)], 7.5);
        assert_eq!(matrix[(2, 2)], 11.0);
        assert_eq!(matrix[(3, 0)], 13.5);
        assert_eq!(matrix[(3, 2)], 15.5);
    }

    #[test]
    fn a_2x2_matrix_can_be_represented() {
        let matrix = Matrix::new(vec![vec![-3.0, -5.0], vec![1.0, -2.0]]);
        assert_eq!(matrix[(0, 0)], -3.0);
        assert_eq!(matrix[(0, 1)], -5.0);
        assert_eq!(matrix[(1, 0)], 1.0);
        assert_eq!(matrix[(1, 1)], -2.0);
    }

    #[test]
    fn a_3x3_matrix_can_be_represented() {
        let matrix = Matrix::new(vec![
            vec![-3.0, -5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ]);
        assert_eq!(matrix[(0, 0)], -3.0);
        assert_eq!(matrix[(1, 1)], -2.0);
        assert_eq!(matrix[(2, 2)], 1.0);
    }

    #[test]
    fn matrices_can_be_compared() {
        let m1 = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        assert!(m1 == m2);
        let m1 = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::new(vec![
            vec![2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0, 9.0],
            vec![8.0, 7.0, 6.0, 5.0],
            vec![4.0, 3.0, 2.0, 1.0],
        ]);
        assert!(m1 != m2);
    }

    #[test]
    fn matrices_can_be_multiplied() {
        let m1 = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::new(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);
        assert_eq!(
            &m1 * &m2,
            Matrix::new(vec![
                vec![20.0, 22.0, 50.0, 48.0],
                vec![44.0, 54.0, 114.0, 108.0],
                vec![40.0, 58.0, 110.0, 102.0],
                vec![16.0, 26.0, 46.0, 42.0]
            ])
        );
    }

    #[test]
    fn a_matrix_can_be_multiplied_by_a_tuple() {
        let m = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
        let p = Point::new(1.0, 2.0, 3.0);
        assert_eq!(&m * &p, Point::new(18.0, 24.0, 33.0));
    }

    #[test]
    fn multiplying_a_matrix_for_identity_matrix_gives_back_the_same() {
        let m = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(
            &m * &Matrix::identity(4),
            Matrix::new(vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![2.0, 4.0, 4.0, 2.0],
                vec![8.0, 6.0, 4.0, 1.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],)
        );
    }

    #[test]
    fn matrix_can_be_transposed() {
        let m = Matrix::new(vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ]);
        assert_eq!(
            m.transpose(),
            Matrix::new(vec![
                vec![0.0, 9.0, 1.0, 0.0],
                vec![9.0, 8.0, 8.0, 0.0],
                vec![3.0, 0.0, 5.0, 5.0],
                vec![0.0, 8.0, 3.0, 8.0],
            ],)
        );
    }

    #[test]
    fn the_transpose_of_the_identity_matrix_is_the_identity_matrix() {
        let id = Matrix::identity(4);
        assert_eq!(id.transpose(), Matrix::identity(4));
    }

    #[test]
    fn can_calculate_the_determinant_of_a_2x2_matrix() {
        let m = Matrix::new(vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);
        assert_eq!(m.determinant(), 17.0);
    }

    #[test]
    fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let m = Matrix::new(vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        ]);
        assert_eq!(
            m.submatrix(0, 2),
            Matrix::new(vec![vec![-3.0, 2.0], vec![0.0, 6.0]])
        );
    }

    #[test]
    fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let m = Matrix::new(vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ]);
        assert_eq!(
            m.submatrix(2, 1),
            Matrix::new(vec![
                vec![-6.0, 1.0, 6.0],
                vec![-8.0, 8.0, 6.0],
                vec![-7.0, -1.0, 1.0]
            ])
        )
    }

    #[test]
    fn the_minor_of_a_3x3_matrix_can_be_calculated() {
        let m = Matrix::new(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        let s = m.submatrix(1, 0);
        assert_eq!(s.determinant(), 25.0);
        assert_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn the_cofactor_of_a_3x3_matrix_can_be_calculated() {
        let m = Matrix::new(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn can_calculate_the_determinant_of_a_3x3_matrix() {
        let m = Matrix::new(vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ]);
        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.determinant(), -196.0);
    }

    #[test]
    fn can_calculate_the_determinant_of_a_4x4_matrix() {
        let m = Matrix::new(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn testing_the_inverse_of_a_invertible_matrix() {
        let m = Matrix::new(vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![9.0, 1.0, 7.0, -6.0],
        ]);
        assert_eq!(m.determinant(), -2120.0);
        assert!(m.inverse().is_some());
    }

    #[test]
    fn testing_the_inverse_of_a_non_invertible_matrix() {
        let m = Matrix::new(vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(m.determinant(), 0.0);
        assert!(m.inverse().is_none());
    }

    #[test]
    fn can_calculate_the_inverse_of_a_matrix() {
        let m = Matrix::new(vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ]);
        let i = m.inverse();
        assert_eq!(m.determinant(), 532.0);
        assert_eq!(m.cofactor(2, 3), -160.0);
        assert!(i.is_some());
        let i = i.unwrap();
        assert_eq!(i[(3, 2)], -160.0 / 532.0);
        assert_eq!(m.cofactor(3, 2), 105.0);
        assert_eq!(i[(2, 3)], 105.0 / 532.0);
        assert_eq!(
            i,
            Matrix::new(vec![
                vec![0.21805, 0.45113, 0.24060, -0.04511],
                vec![-0.80827, -1.45677, -0.44361, 0.52068],
                vec![-0.07895, -0.22368, -0.05263, 0.19737],
                vec![-0.52256, -0.81391, -0.30075, 0.30639]
            ])
        );
    }

    #[test]
    fn can_calculate_the_inverse_of_another_matrix() {
        let m = Matrix::new(vec![
            vec![8.0, -5.0, 9.0, 2.0],
            vec![7.0, 5.0, 6.0, 1.0],
            vec![-6.0, 0.0, 9.0, 6.0],
            vec![-3.0, 0.0, -9.0, -4.0],
        ]);
        let i = m.inverse();
        assert!(i.is_some());
        let i = i.unwrap();
        assert_eq!(
            i,
            Matrix::new(vec![
                vec![-0.15385, -0.15385, -0.28205, -0.53846],
                vec![-0.07692, 0.12308, 0.02564, 0.03077],
                vec![0.35897, 0.35897, 0.43590, 0.92308],
                vec![-0.69231, -0.69231, -0.76923, -1.92308]
            ])
        )
    }

    #[test]
    fn can_calculate_the_inverse_of_a_third_matrix() {
        let m = Matrix::new(vec![
            vec![9.0, 3.0, 0.0, 9.0],
            vec![-5.0, -2.0, -6.0, -3.0],
            vec![-4.0, 9.0, 6.0, 4.0],
            vec![-7.0, 6.0, 6.0, 2.0],
        ]);
        let i = m.inverse();
        assert!(i.is_some());
        let i = i.unwrap();
        assert_eq!(
            i,
            Matrix::new(vec![
                vec![-0.04074, -0.07778, 0.14444, -0.22222],
                vec![-0.07778, 0.03333, 0.36667, -0.33333],
                vec![-0.02901, -0.14630, -0.10926, 0.12963],
                vec![0.17778, 0.06667, -0.26667, 0.33333],
            ])
        )
    }

    #[test]
    fn multiplying_a_product_by_the_inverse() {
        let m1 = Matrix::new(vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 4.0, 4.0, 1.0],
            vec![-6.0, 5.0, -1.0, 1.0],
        ]);
        let m2 = Matrix::new(vec![
            vec![8.0, 2.0, 2.0, 2.0],
            vec![3.0, -1.0, 7.0, 0.0],
            vec![7.0, 0.0, 5.0, 4.0],
            vec![6.0, -2.0, 0.0, 5.0],
        ]);
        let i2 = m2.inverse();
        let m3 = &m1 * &m2;
        assert!(i2.is_some());
        let i2 = i2.unwrap();
        assert_eq!(&m3 * &i2, m1);
    }
}
