//! Define trait for Hermite matrices

use std::fmt::Debug;
use ndarray::prelude::*;
use ndarray::LinalgScalar;
use num_traits::float::Float;

use matrix::Matrix;
use error::{LinalgError, NotSquareError};
use qr::ImplQR;
use eig::ImplEig;
use svd::ImplSVD;
use norm::ImplNorm;
use solve::ImplSolve;

/// Methods for square matrices
///
/// This trait defines method for square matrices,
/// but does not assure that the matrix is square.
/// If not square, `NotSquareError` will be thrown.
pub trait SquareMatrix: Matrix {
    fn eig(self) -> Result<(Vec<Self::Complex>, Self), LinalgError>;
    /// inverse matrix
    fn inv(self) -> Result<Self, LinalgError>;
    /// trace of matrix
    fn trace(&self) -> Result<Self::Scalar, LinalgError>;
    /// test matrix is square
    fn check_square(&self) -> Result<(), NotSquareError> {
        let (rows, cols) = self.size();
        if rows == cols {
            Ok(())
        } else {
            Err(NotSquareError {
                rows: rows,
                cols: cols,
            })
        }
    }
}

impl<A> SquareMatrix for Array<A, (Ix, Ix)>
    where A: ImplEig + ImplQR + ImplNorm + ImplSVD + ImplSolve + LinalgScalar + Float + Debug
{
    fn eig(self) -> Result<(Vec<Self::Complex>, Self), LinalgError> {
        try!(self.check_square());
        let (n, _) = self.size();
        let (w, vr) = try!(ImplEig::eig(n, self.into_raw_vec()));
        let vm = Array::from_vec(vr).into_shape((n, n)).unwrap();
        Ok((w, vm))
    }
    fn inv(self) -> Result<Self, LinalgError> {
        try!(self.check_square());
        let (n, _) = self.size();
        let is_fortran_align = self.strides()[0] > self.strides()[1];
        let a = try!(ImplSolve::inv(self.layout(), n, self.into_raw_vec()));
        let m = Array::from_vec(a).into_shape((n, n)).unwrap();
        if is_fortran_align {
            Ok(m)
        } else {
            Ok(m.reversed_axes())
        }
    }
    fn trace(&self) -> Result<Self::Scalar, LinalgError> {
        try!(self.check_square());
        let (n, _) = self.size();
        Ok((0..n).fold(A::zero(), |sum, i| sum + self[(i, i)]))
    }
}
