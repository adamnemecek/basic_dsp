//! Fundamental math operations
use RealNumber;
use multicore_support::*;
use simd_extensions::*;
use num::Complex;
use std::ops::*;
use super::super::{array_to_complex, array_to_complex_mut, ErrorReason, VoidResult, Vector,
                   DspVec, ToSliceMut, MetaData, Domain, NumberSpace, ComplexNumberSpace};

/// An operation which multiplies each vector element with a constant
pub trait ScaleOps<T>: Sized
    where T: Sized
{
    /// Multiplies the vector element with a scalar.
    ///
    /// # Failures
    ///
    /// `self.len()` to `0` if the vector isn't in the complex number space but
    /// `factor` is complex.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::*;
    /// # fn main() {
    /// let mut vector = vec!(1.0, 2.0).to_real_time_vec();
    /// vector.scale(2.0);
    /// assert_eq!([2.0, 4.0], vector[0..]);
    /// # }
    /// ```
    fn scale(&mut self, factor: T);
}

/// An operation which adds a constant to each vector element
pub trait OffsetOps<T>: Sized
    where T: Sized
{
    /// Adds a scalar to each vector element.
    ///
    /// # Failures
    ///
    /// `self.len()` to `0` if the vector isn't in the complex number space but
    /// `factor` is complex.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::*;
    /// # fn main() {
    /// let mut vector = vec!(1.0, 2.0).to_real_time_vec();
    /// vector.offset(2.0);
    /// assert_eq!([3.0, 4.0], vector[0..]);
    /// # }
    /// ```
    fn offset(&mut self, offset: T);
}

pub trait ElementaryOps<A> {
    /// Calculates the sum of `self + summand`. It consumes self and returns the result.
    /// # Failures
    /// TransRes may report the following `ErrorReason` members:
    ///
    /// 1. `VectorsMustHaveTheSameSize`: `self` and `summand` must have the same size
    /// 2. `VectorMetaDataMustAgree`: `self` and `summand` must be in the same domain
    ///    and number space
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::*;
    /// # fn main() {
    /// let mut vector1 = vec!(1.0, 2.0).to_real_time_vec();
    /// let vector2 = vec!(10.0, 11.0).to_real_time_vec();
    /// vector1.add(&vector2).expect("Ignoring error handling in examples");
    /// assert_eq!([11.0, 13.0], vector1[0..]);
    /// # }
    /// ```
    fn add(&mut self, summand: &A) -> VoidResult;

    /// Calculates the difference of `self - subtrahend`. It consumes self and returns the result.
    /// # Failures
    /// TransRes may report the following `ErrorReason` members:
    ///
    /// 1. `VectorsMustHaveTheSameSize`: `self` and `subtrahend` must have the same size
    /// 2. `VectorMetaDataMustAgree`: `self` and `subtrahend` must be in the same domain
    ///    and number space
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::*;
    /// # fn main() {
    /// let mut vector1 = vec!(1.0, 2.0).to_real_time_vec();
    /// let vector2 = vec!(10.0, 11.0).to_real_time_vec();
    /// vector1.sub(&vector2).expect("Ignoring error handling in examples");
    /// assert_eq!([-9.0, -9.0], vector1[0..]);
    /// # }
    /// ```
    fn sub(&mut self, subtrahend: &A) -> VoidResult;

    /// Calculates the product of `self * factor`. It consumes self and returns the result.
    /// # Failures
    /// TransRes may report the following `ErrorReason` members:
    ///
    /// 1. `VectorsMustHaveTheSameSize`: `self` and `factor` must have the same size
    /// 2. `VectorMetaDataMustAgree`: `self` and `factor` must be in the same domain and
    ///    number space
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::*;
    /// # fn main() {
    /// let mut vector1 = vec!(1.0, 2.0).to_real_time_vec();
    /// let vector2 = vec!(10.0, 11.0).to_real_time_vec();
    /// vector1.mul(&vector2).expect("Ignoring error handling in examples");
    /// assert_eq!([10.0, 22.0], vector1[0..]);
    /// # }
    /// ```
    fn mul(&mut self, factor: &A) -> VoidResult;

    /// Calculates the quotient of `self / summand`. It consumes self and returns the result.
    /// # Failures
    /// TransRes may report the following `ErrorReason` members:
    ///
    /// 1. `VectorsMustHaveTheSameSize`: `self` and `divisor` must have the same size
    /// 2. `VectorMetaDataMustAgree`: `self` and `divisor` must be in the same domain
    ///    and number space
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::*;
    /// # fn main() {
    /// let mut vector1 = vec!(10.0, 22.0).to_real_time_vec();
    /// let vector2 = vec!(2.0, 11.0).to_real_time_vec();
    /// vector1.div(&vector2).expect("Ignoring error handling in examples");
    /// assert_eq!([5.0, 2.0], vector1[0..]);
    /// # }
    /// ```
    fn div(&mut self, divisor: &A) -> VoidResult;
}

pub trait ElementaryWrapAroundOps<A> {
    /// Calculates the sum of `self + summand`. `summand` may be smaller than `self` as long
    /// as `self.len() % summand.len() == 0`. THe result is the same as it would be if
    /// you would repeat `summand` until it has the same length as `self`.
    /// It consumes self and returns the result.
    /// # Failures
    /// TransRes may report the following `ErrorReason` members:
    ///
    /// 1. `InvalidArgumentLength`: `self.points()` isn't dividable by `summand.points()`
    /// 2. `VectorMetaDataMustAgree`: `self` and `summand` must be in the same domain
    ///    and number space
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::*;
    /// # fn main() {
    /// let mut vector1 = vec!(10.0, 11.0, 12.0, 13.0).to_real_time_vec();
    /// let vector2 = vec!(1.0, 2.0).to_real_time_vec();
    /// vector1.add_smaller(&vector2).expect("Ignoring error handling in examples");
    /// assert_eq!([11.0, 13.0, 13.0, 15.0], vector1[0..]);
    /// # }
    /// ```
    fn add_smaller(&mut self, summand: &A) -> VoidResult;

    /// Calculates the sum of `self - subtrahend`. `subtrahend` may be smaller than `self` as long
    /// as `self.len() % subtrahend.len() == 0`. THe result is the same as it would be if
    /// you would repeat `subtrahend` until it has the same length as `self`.
    /// It consumes self and returns the result.
    /// # Failures
    /// TransRes may report the following `ErrorReason` members:
    ///
    /// 1. `InvalidArgumentLength`: `self.points()` isn't dividable by `subtrahend.points()`
    /// 2. `VectorMetaDataMustAgree`: `self` and `subtrahend` must be in the
    ///    same domain and number space
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::*;
    /// # fn main() {
    /// let mut vector1 = vec!(10.0, 11.0, 12.0, 13.0).to_real_time_vec();
    /// let vector2 = vec!(1.0, 2.0).to_real_time_vec();
    /// vector1.sub_smaller(&vector2).expect("Ignoring error handling in examples");
    /// assert_eq!([9.0, 9.0, 11.0, 11.0], vector1[0..]);
    /// # }
    /// ```
    fn sub_smaller(&mut self, summand: &A) -> VoidResult;

    /// Calculates the sum of `self - factor`. `factor` may be smaller than `self` as long
    /// as `self.len() % factor.len() == 0`. THe result is the same as it would be if
    /// you would repeat `factor` until it has the same length as `self`.
    /// It consumes self and returns the result.
    /// # Failures
    /// TransRes may report the following `ErrorReason` members:
    ///
    /// 1. `InvalidArgumentLength`: `self.points()` isn't dividable by `factor.points()`
    /// 2. `VectorMetaDataMustAgree`: `self` and `factor` must be in the same
    ///    domain and number space
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::*;
    /// # fn main() {
    /// let mut vector1 = vec!(10.0, 11.0, 12.0, 13.0).to_real_time_vec();
    /// let vector2 = vec!(1.0, 2.0).to_real_time_vec();
    /// vector1.mul_smaller(&vector2).expect("Ignoring error handling in examples");
    /// assert_eq!([10.0, 22.0, 12.0, 26.0], vector1[0..]);
    /// # }
    /// ```
    fn mul_smaller(&mut self, factor: &A) -> VoidResult;

    /// Calculates the sum of `self - divisor`. `divisor` may be smaller than `self` as long
    /// as `self.len() % divisor.len() == 0`. THe result is the same as it would be if
    /// you would repeat `divisor` until it has the same length as `self`.
    /// It consumes self and returns the result.
    /// # Failures
    /// TransRes may report the following `ErrorReason` members:
    ///
    /// 1. `InvalidArgumentLength`: `self.points()` isn't dividable by `divisor.points()`
    /// 2. `VectorMetaDataMustAgree`: `self` and `divisor` must be in the same domain
    ///    and number space
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::*;
    /// # fn main() {
    /// let mut vector1 = vec!(10.0, 12.0, 12.0, 14.0).to_real_time_vec();
    /// let vector2 = vec!(1.0, 2.0).to_real_time_vec();
    /// vector1.div_smaller(&vector2).expect("Ignoring error handling in examples");
    /// assert_eq!([10.0, 6.0, 12.0, 7.0], vector1[0..]);
    /// # }
    /// ```
    fn div_smaller(&mut self, divisor: &A) -> VoidResult;
}

macro_rules! assert_complex {
    ($self_: ident) => {
        if !$self_.is_complex() {
            $self_.valid_len = 0;
            return;
        }
    }
}

impl<S, T, N, D> OffsetOps<T> for DspVec<S, T, N, D>
    where S: ToSliceMut<T>,
          T: RealNumber,
          N: NumberSpace,
          D: Domain
{
    fn offset(&mut self, offset: T) {
        if self.is_complex() {
            let vector_offset = T::Reg::from_complex(Complex::new(offset, T::zero()));
            self.simd_complex_operation(|x, y| x + y,
                                        |x, y| x + Complex::<T>::new(y.extract(0), y.extract(1)),
                                        vector_offset,
                                        Complexity::Small);
        } else {
            self.simd_real_operation(|x, y| x.add_real(y),
                                     |x, y| x + y,
                                     offset,
                                     Complexity::Small);
        }
    }
}

impl<S, T, N, D> OffsetOps<Complex<T>> for DspVec<S, T, N, D>
    where S: ToSliceMut<T>,
          T: RealNumber,
          N: ComplexNumberSpace,
          D: Domain
{
    fn offset(&mut self, offset: Complex<T>) {
        assert_complex!(self);
        let vector_offset = T::Reg::from_complex(offset);
        self.simd_complex_operation(|x, y| x + y,
                                    |x, y| x + Complex::<T>::new(y.extract(0), y.extract(1)),
                                    vector_offset,
                                    Complexity::Small);
    }
}

impl<S, T, D, N> ScaleOps<T> for DspVec<S, T, N, D>
    where S: ToSliceMut<T>,
          T: RealNumber,
          N: NumberSpace,
          D: Domain
{
    fn scale(&mut self, factor: T) {
        self.simd_real_operation(|x, y| x.scale_real(y),
                                 |x, y| x * y,
                                 factor,
                                 Complexity::Small);
    }
}

impl<S, T, D, N> ScaleOps<Complex<T>> for DspVec<S, T, N, D>
    where S: ToSliceMut<T>,
          T: RealNumber,
          N: ComplexNumberSpace,
          D: Domain
{
    fn scale(&mut self, factor: Complex<T>) {
        assert_complex!(self);
        if factor.im == T::zero() {
            self.simd_real_operation(|x, y| x.scale_real(y),
                                     |x, y| x * y,
                                     factor.re,
                                     Complexity::Small);
        } else {
            self.simd_complex_operation(|x, y| x.scale_complex(y),
                                        |x, y| x * y,
                                        factor,
                                        Complexity::Small);
        }
    }
}

macro_rules! reject_if {
    ($self_: ident, $condition: expr, $message: expr) => {
        if $condition {
            return Err($message);
        }
    }
}

macro_rules! assert_meta_data {
    ($self_: ident, $other: ident) => {
         {
            let delta_ratio = $self_.delta / $other.delta;
            if $self_.is_complex() != $other.is_complex() ||
                $self_.domain != $other.domain ||
                delta_ratio > T::from(1.1).unwrap() || delta_ratio < T::from(0.9).unwrap() {
                return Err(ErrorReason::InputMetaDataMustAgree);
            }
         }
    }
}

macro_rules! impl_binary_vector_operation {
    (fn $method: ident, $arg_name: ident, $simd_op: ident, $scal_op: ident) => {
        fn $method(&mut self, $arg_name: &Self) -> VoidResult
        {
            {
                let len = self.len();
                reject_if!(self, len != $arg_name.len(), ErrorReason::InputMustHaveTheSameSize);
                assert_meta_data!(self, $arg_name);

                let data_length = self.len();
                let mut array = self.data.to_slice_mut();
                let (scalar_left, scalar_right, vectorization_length) =
                    T::Reg::calc_data_alignment_reqs(&array[0..data_length]);
                let other = &$arg_name.data.to_slice();
                if vectorization_length > 0 {
                    Chunk::from_src_to_dest(
                        Complexity::Small, &self.multicore_settings,
                        &other[scalar_left..vectorization_length], T::Reg::len(),
                        &mut array[scalar_left..vectorization_length], T::Reg::len(), (),
                        |original, range, target, _arg| {
                            let original =
                                T::Reg::array_to_regs(&original[range.start .. range.end]);
                            let mut target =
                                T::Reg::array_to_regs_mut(&mut target[range.start .. range.end]);
                            for (dst, src) in &mut target.iter_mut().zip(original) {
                                *dst = dst.$simd_op(*src);
                            }
                    });
                }
                for i in 0..scalar_left {
                    array[i] = array[i].$scal_op(other[i]);
                }

                for i in scalar_right..data_length {
                    array[i] = array[i].$scal_op(other[i]);
                }
            }

            Ok(())
        }
    }
}

macro_rules! impl_binary_complex_vector_operation {
    (fn $method: ident, $arg_name: ident, $simd_op: ident, $scal_op: ident) => {
        fn $method(&mut self, $arg_name: &Self) -> VoidResult
        {
            {
                let len = self.len();
                reject_if!(self, len != $arg_name.len(), ErrorReason::InputMustHaveTheSameSize);
                assert_meta_data!(self, $arg_name);

                let data_length = self.len();
                let mut array = self.data.to_slice_mut();
                let (scalar_left, scalar_right, vectorization_length) =
                    T::Reg::calc_data_alignment_reqs(&array[0..data_length]);
                let other = &$arg_name.data.to_slice();
                if vectorization_length > 0 {
                    Chunk::from_src_to_dest(
                        Complexity::Small, &self.multicore_settings,
                        &other[scalar_left..vectorization_length], T::Reg::len(),
                        &mut array[scalar_left..vectorization_length], T::Reg::len(), (),
                        |original, range, target, _arg| {
                            let original =
                                T::Reg::array_to_regs(&original[range.start .. range.end]);
                            let target =
                                T::Reg::array_to_regs_mut(&mut target[range.start .. range.end]);
                            for (dst, src) in target.iter_mut().zip(original) {
                                *dst = dst.$simd_op(*src);
                            }
                    });
                }
                let mut i = 0;
                while i < scalar_left {
                    let complex1 = Complex::<T>::new(array[i], array[i + 1]);
                    let complex2 = Complex::<T>::new(other[i], other[i + 1]);
                    let result = complex1.$scal_op(complex2);
                    array[i] = result.re;
                    array[i + 1] = result.im;
                    i += 2;
                }

                let mut i = scalar_right;
                while i < data_length {
                    let complex1 = Complex::<T>::new(array[i], array[i + 1]);
                    let complex2 = Complex::<T>::new(other[i], other[i + 1]);
                    let result = complex1.$scal_op(complex2);
                    array[i] = result.re;
                    array[i + 1] = result.im;
                    i += 2;
                }
            }

            Ok(())
        }
    }
}

macro_rules! impl_binary_smaller_vector_operation {
    (fn $method: ident, $arg_name: ident, $simd_op: ident, $scal_op: ident) => {
        fn $method(&mut self, $arg_name: &Self) -> VoidResult
        {
            {
                let len = self.len();
                reject_if!(self, len % $arg_name.len() != 0, ErrorReason::InvalidArgumentLength);
                assert_meta_data!(self, $arg_name);

                let data_length = self.len();
                let mut array = self.data.to_slice_mut();
                let other = $arg_name.data.to_slice();
                Chunk::from_src_to_dest(
                    Complexity::Small, &self.multicore_settings,
                    &other, T::Reg::len(),
                    &mut array[0..data_length], 1, (),
                    |operand, range, target, _arg| {
                        let mut i = range.start;
                        for n in &mut target[..] {
                            *n = n.$scal_op(operand[i % operand.len()]);
                            i += 1;
                        }
                });
            }

            Ok(())
        }
    }
}

macro_rules! impl_binary_smaller_complex_vector_ops {
    (fn $method: ident, $arg_name: ident, $simd_op: ident, $scal_op: ident) => {
        fn $method(&mut self, $arg_name: &Self) -> VoidResult
        {
            {
                let len = self.len();
                reject_if!(self, len % $arg_name.len() != 0, ErrorReason::InvalidArgumentLength);
                assert_meta_data!(self, $arg_name);

                let data_length = self.len();
                let mut array = self.data.to_slice_mut();
                let other = $arg_name.data.to_slice();
                Chunk::from_src_to_dest(
                    Complexity::Small, &self.multicore_settings,
                    &other, T::Reg::len(),
                    &mut array[0..data_length], 2, (),
                    |operand, range, target, _arg| {
                        let mut target = array_to_complex_mut(&mut target[..]);
                        let operand = array_to_complex(&operand[..]);
                        let mut i = range.start;
                        for n in &mut target[..] {
                            *n = n.$scal_op(operand[i % operand.len()]);
                            i += 1;
                        }
                });
            }

            Ok(())
        }
    }
}

impl<S, T, N, D> DspVec<S, T, N, D>
    where S: ToSliceMut<T>,
          T: RealNumber,
          N: NumberSpace,
          D: Domain
{
    impl_binary_complex_vector_operation!(fn mul_complex, factor, mul_complex, mul);
    impl_binary_smaller_complex_vector_ops!(fn mul_smaller_complex, factor, mul_complex, mul);
    impl_binary_vector_operation!(fn mul_real, factor, mul, mul);
    impl_binary_smaller_vector_operation!(fn mul_smaller_real, factor, mul, mul);
    impl_binary_complex_vector_operation!(fn div_complex, divisor, div_complex, div);
    impl_binary_smaller_complex_vector_ops!(fn div_smaller_complex, divisor, div_complex, div);
    impl_binary_vector_operation!(fn div_real, divisor, div, div);
    impl_binary_smaller_vector_operation!(fn div_smaller_real, divisor, div, div);
}

impl<S, T, N, D> ElementaryOps<DspVec<S, T, N, D>> for DspVec<S, T, N, D>
    where S: ToSliceMut<T>,
          T: RealNumber,
          N: NumberSpace,
          D: Domain
{
    impl_binary_vector_operation!(fn add, summand, add, add);
    impl_binary_vector_operation!(fn sub, summand, sub, sub);

    fn mul(&mut self, factor: &Self) -> VoidResult {
        let len = self.len();
        reject_if!(self, len != factor.len(), ErrorReason::InputMustHaveTheSameSize);
        assert_meta_data!(self, factor);

        if self.is_complex() {
            self.mul_complex(factor)
        } else {
            self.mul_real(factor)
        }
    }

    fn div(&mut self, divisor: &Self) -> VoidResult {
        let len = self.len();
        reject_if!(self, len != divisor.len(), ErrorReason::InputMustHaveTheSameSize);
        assert_meta_data!(self, divisor);

        if self.is_complex() {
            self.div_complex(divisor)
        } else {
            self.div_real(divisor)
        }
    }
}

impl<S, T, N, D> ElementaryWrapAroundOps<DspVec<S, T, N, D>> for DspVec<S, T, N, D>
    where S: ToSliceMut<T>,
          T: RealNumber,
          N: NumberSpace,
          D: Domain
{
    impl_binary_smaller_vector_operation!(fn add_smaller, summand, add, add);
    impl_binary_smaller_vector_operation!(fn sub_smaller, summand, sub, sub);

    fn mul_smaller(&mut self, factor: &Self) -> VoidResult {
        let len = self.len();
        reject_if!(self, len % factor.len() != 0, ErrorReason::InvalidArgumentLength);
        assert_meta_data!(self, factor);

        if self.is_complex() {
            self.mul_smaller_complex(factor)
        } else {
            self.mul_smaller_real(factor)
        }
    }

    fn div_smaller(&mut self, divisor: &Self) -> VoidResult {
        let len = self.len();
        reject_if!(self, len % divisor.len() != 0, ErrorReason::InvalidArgumentLength);
        assert_meta_data!(self, divisor);

        if self.is_complex() {
            self.div_smaller_complex(divisor)
        } else {
            self.div_smaller_real(divisor)
        }
    }
}