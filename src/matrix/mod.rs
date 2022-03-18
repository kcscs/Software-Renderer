pub mod vector;
pub use vector::*;

use num_traits::Num;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

pub trait ValueT: Num + Copy {}

impl ValueT for f32 {}

#[derive(Clone)]
pub struct MatrixNMT<T, const N: usize, const M: usize>
where
    T: ValueT,
{
    m: [[T; M]; N],
}

impl<T, const N: usize, const M: usize> MatrixNMT<T, N, M>
where
    T: ValueT,
{
    pub fn zero() -> Self {
        Self {
            m: [[T::zero(); M]; N],
        }
    }
}

impl<T, const N: usize> MatrixNT<T, N>
where
    T: ValueT,
{
    pub fn identity() -> Self {
        let mut m = Self::zero();
        for i in 0..N {
            m.m[i][i] = T::one();
        }
        m
    }
}

pub type MatrixNT<T, const N: usize> = MatrixNMT<T, N, N>;
pub type Matrix3 = MatrixNT<f32, 3>;
pub type Matrix4 = MatrixNT<f32, 4>;

impl<T, const N: usize, const M: usize> Add<MatrixNMT<T, N, M>> for MatrixNMT<T, N, M>
where
    T: ValueT,
{
    type Output = Self;

    fn add(self, rhs: MatrixNMT<T, N, M>) -> Self::Output {
        let mut res = rhs;
        for i in 0..N {
            for j in 0..M {
                res[i][j] = res[i][j] + self[i][j];
            }
        }
        res
    }
}

impl<T, const N: usize, const M: usize> Sub<MatrixNMT<T, N, M>> for MatrixNMT<T, N, M>
where
    T: ValueT,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = rhs;
        for i in 0..N {
            for j in 0..M {
                res[i][j] = res[i][j] - self[i][j];
            }
        }
        res
    }
}

impl<T, const N: usize, const M: usize, const U: usize> Mul<MatrixNMT<T, M, U>>
    for MatrixNMT<T, N, M>
where
    T: ValueT,
{
    type Output = MatrixNMT<T, N, U>;
    fn mul(self, rhs: MatrixNMT<T, M, U>) -> Self::Output {
        let mut o = Self::Output::zero();
        for i in 0..N {
            for j in 0..U {
                for k in 0..M {
                    o.m[i][j] = o.m[i][j] + self.m[i][k] * rhs.m[k][j];
                }
            }
        }

        o
    }
}

impl<T, const N: usize, const M: usize> Index<usize> for MatrixNMT<T, N, M>
where
    T: ValueT,
{
    type Output = [T; M];
    fn index(&self, i: usize) -> &Self::Output {
        &self.m[i]
    }
}

impl<T, const N: usize, const M: usize> IndexMut<usize> for MatrixNMT<T, N, M>
where
    T: ValueT,
{
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.m[i]
    }
}
