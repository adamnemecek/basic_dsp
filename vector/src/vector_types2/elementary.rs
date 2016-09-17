//! Fundamental math operations
use RealNumber;
use multicore_support::*;
use simd_extensions::*;
use num::Complex;
use std::ops::*;
use super::{
    ErrorReason, VoidResult,
    DspVec, ToSliceMut,
    Domain, RealNumberSpace, ComplexNumberSpace
};


/// An operation which multiplies each vector element with a constant
pub trait ScaleOps<T> : Sized
    where T: Sized {
    /// Multiplies the vector element with a scalar.
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::vector_types2::*;
    /// # fn main() {
    /// let mut vector = vec!(1.0, 2.0).to_real_time_vec();
    /// vector.scale(2.0).expect("Ignoring error handling in examples");
    /// assert_eq!([2.0, 4.0], vector[0..]);
    /// # }
    /// ```
    fn scale(&mut self, factor: T) -> VoidResult;
}

/// An operation which adds a constant to each vector element
pub trait OffsetOps<T> : Sized
    where T: Sized {
    /// Adds a scalar to each vector element.
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::vector_types2::*;
    /// # fn main() {
    /// let mut vector = vec!(1.0, 2.0).to_real_time_vec();
    /// vector.offset(2.0).expect("Ignoring error handling in examples");
    /// assert_eq!([3.0, 4.0], vector[0..]);
    /// # }
    /// ```
    fn offset(&mut self, offset: T) -> VoidResult;
}

pub trait ElementaryOps {
    /// Calculates the sum of `self + summand`. It consumes self and returns the result.
    /// # Failures
    /// TransRes may report the following `ErrorReason` members:
    ///
    /// 1. `VectorsMustHaveTheSameSize`: `self` and `summand` must have the same size
    /// 2. `VectorMetaDataMustAgree`: `self` and `summand` must be in the same domain and number space
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::vector_types2::*;
    /// # fn main() {
    /// let mut vector1 = vec!(1.0, 2.0).to_real_time_vec();
    /// let vector2 = vec!(10.0, 11.0).to_real_time_vec();
    /// vector1.add(&vector2).expect("Ignoring error handling in examples");
    /// assert_eq!([11.0, 13.0], vector1[0..]);
    /// # }
    /// ```
    fn add(&mut self, summand: &Self) -> VoidResult;

    /// Calculates the sum of `self + summand`. `summand` may be smaller than `self` as long
    /// as `self.len() % summand.len() == 0`. THe result is the same as it would be if
    /// you would repeat `summand` until it has the same length as `self`.
    /// It consumes self and returns the result.
    /// # Failures
    /// TransRes may report the following `ErrorReason` members:
    ///
    /// 1. `InvalidArgumentLength`: `self.points()` isn't dividable by `summand.points()`
    /// 2. `VectorMetaDataMustAgree`: `self` and `summand` must be in the same domain and number space
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::vector_types2::*;
    /// # fn main() {
    /// let mut vector1 = vec!(10.0, 11.0, 12.0, 13.0).to_real_time_vec();
    /// let vector2 = vec!(1.0, 2.0).to_real_time_vec();
    /// vector1.add_smaller(&vector2).expect("Ignoring error handling in examples");
    /// assert_eq!([11.0, 13.0, 13.0, 15.0], vector1[0..]);
    /// # }
    /// ```
    fn add_smaller(&mut self, summand: &Self) -> VoidResult;

    /// Calculates the difference of `self - subtrahend`. It consumes self and returns the result.
    /// # Failures
    /// TransRes may report the following `ErrorReason` members:
    ///
    /// 1. `VectorsMustHaveTheSameSize`: `self` and `subtrahend` must have the same size
    /// 2. `VectorMetaDataMustAgree`: `self` and `subtrahend` must be in the same domain and number space
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::vector_types2::*;
    /// # fn main() {
    /// let mut vector1 = vec!(1.0, 2.0).to_real_time_vec();
    /// let vector2 = vec!(10.0, 11.0).to_real_time_vec();
    /// vector1.sub(&vector2).expect("Ignoring error handling in examples");
    /// assert_eq!([-9.0, -9.0], vector1[0..]);
    /// # }
    /// ```
    fn sub(&mut self, subtrahend: &Self) -> VoidResult;

    /// Calculates the sum of `self - subtrahend`. `subtrahend` may be smaller than `self` as long
    /// as `self.len() % subtrahend.len() == 0`. THe result is the same as it would be if
    /// you would repeat `subtrahend` until it has the same length as `self`.
    /// It consumes self and returns the result.
    /// # Failures
    /// TransRes may report the following `ErrorReason` members:
    ///
    /// 1. `InvalidArgumentLength`: `self.points()` isn't dividable by `subtrahend.points()`
    /// 2. `VectorMetaDataMustAgree`: `self` and `subtrahend` must be in the same domain and number space
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::vector_types2::*;
    /// # fn main() {
    /// let mut vector1 = vec!(10.0, 11.0, 12.0, 13.0).to_real_time_vec();
    /// let vector2 = vec!(1.0, 2.0).to_real_time_vec();
    /// vector1.sub_smaller(&vector2).expect("Ignoring error handling in examples");
    /// assert_eq!([9.0, 9.0, 11.0, 11.0], vector1[0..]);
    /// # }
    /// ```
    fn sub_smaller(&mut self, summand: &Self) -> VoidResult;

    /// Calculates the product of `self * factor`. It consumes self and returns the result.
    /// # Failures
    /// TransRes may report the following `ErrorReason` members:
    ///
    /// 1. `VectorsMustHaveTheSameSize`: `self` and `factor` must have the same size
    /// 2. `VectorMetaDataMustAgree`: `self` and `factor` must be in the same domain and number space
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::vector_types2::*;
    /// # fn main() {
    /// let mut vector1 = vec!(1.0, 2.0).to_real_time_vec();
    /// let vector2 = vec!(10.0, 11.0).to_real_time_vec();
    /// vector1.mul(&vector2).expect("Ignoring error handling in examples");
    /// assert_eq!([10.0, 22.0], vector1[0..]);
    /// # }
    /// ```
    fn mul(&mut self, factor: &Self) -> VoidResult;

    /// Calculates the sum of `self - factor`. `factor` may be smaller than `self` as long
    /// as `self.len() % factor.len() == 0`. THe result is the same as it would be if
    /// you would repeat `factor` until it has the same length as `self`.
    /// It consumes self and returns the result.
    /// # Failures
    /// TransRes may report the following `ErrorReason` members:
    ///
    /// 1. `InvalidArgumentLength`: `self.points()` isn't dividable by `factor.points()`
    /// 2. `VectorMetaDataMustAgree`: `self` and `factor` must be in the same domain and number space
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::vector_types2::*;
    /// # fn main() {
    /// let mut vector1 = vec!(10.0, 11.0, 12.0, 13.0).to_real_time_vec();
    /// let vector2 = vec!(1.0, 2.0).to_real_time_vec();
    /// vector1.mul_smaller(&vector2).expect("Ignoring error handling in examples");
    /// assert_eq!([10.0, 22.0, 12.0, 26.0], vector1[0..]);
    /// # }
    /// ```
    fn mul_smaller(&mut self, factor: &Self) -> VoidResult;

    /// Calculates the quotient of `self / summand`. It consumes self and returns the result.
    /// # Failures
    /// TransRes may report the following `ErrorReason` members:
    ///
    /// 1. `VectorsMustHaveTheSameSize`: `self` and `divisor` must have the same size
    /// 2. `VectorMetaDataMustAgree`: `self` and `divisor` must be in the same domain and number space
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::vector_types2::*;
    /// # fn main() {
    /// let mut vector1 = vec!(10.0, 22.0).to_real_time_vec();
    /// let vector2 = vec!(2.0, 11.0).to_real_time_vec();
    /// vector1.div(&vector2).expect("Ignoring error handling in examples");
    /// assert_eq!([5.0, 2.0], vector1[0..]);
    /// # }
    /// ```
    fn div(&mut self, divisor: &Self) -> VoidResult;

    /// Calculates the sum of `self - divisor`. `divisor` may be smaller than `self` as long
    /// as `self.len() % divisor.len() == 0`. THe result is the same as it would be if
    /// you would repeat `divisor` until it has the same length as `self`.
    /// It consumes self and returns the result.
    /// # Failures
    /// TransRes may report the following `ErrorReason` members:
    ///
    /// 1. `InvalidArgumentLength`: `self.points()` isn't dividable by `divisor.points()`
    /// 2. `VectorMetaDataMustAgree`: `self` and `divisor` must be in the same domain and number space
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// use basic_dsp_vector::vector_types2::*;
    /// # fn main() {
    /// let mut vector1 = vec!(10.0, 12.0, 12.0, 14.0).to_real_time_vec();
    /// let vector2 = vec!(1.0, 2.0).to_real_time_vec();
    /// vector1.div_smaller(&vector2).expect("Ignoring error handling in examples");
    /// assert_eq!([10.0, 6.0, 12.0, 7.0], vector1[0..]);
    /// # }
    /// ```
    fn div_smaller(&mut self, divisor: &Self) -> VoidResult;
}

macro_rules! assert_real {
    ($self_: ident) => {
        if $self_.is_complex() {
            return Err(ErrorReason::InputMustBeReal);
        }
    }
}

macro_rules! assert_complex {
    ($self_: ident) => {
        if !$self_.is_complex() {
            return Err(ErrorReason::InputMustBeComplex);
        }
    }
}

impl<S, T, N, D> OffsetOps<T> for DspVec<S, T, N, D>
    where S: ToSliceMut<T>,
          T: RealNumber,
          N: RealNumberSpace,
          D: Domain {
    fn offset(&mut self, offset: T) -> VoidResult {
        assert_real!(self);
        self.simd_real_operation(|x, y| x.add_real(y), |x, y| x + y, offset, Complexity::Small);
        Ok(())
    }
}

impl<S, T, N, D> OffsetOps<Complex<T>> for DspVec<S, T, N, D>
    where S: ToSliceMut<T>,
          T: RealNumber,
          N: ComplexNumberSpace,
          D: Domain {
    fn offset(&mut self, offset: Complex<T>) -> VoidResult {
        assert_complex!(self);
        let vector_offset = T::Reg::from_complex(offset);
        self.simd_complex_operation(|x,y| x + y, |x,y| x + Complex::<T>::new(y.extract(0), y.extract(1)), vector_offset, Complexity::Small);
        Ok(())
    }
}

impl<S, T, N, D> ScaleOps<T> for DspVec<S, T, N, D>
    where S: ToSliceMut<T>,
          T: RealNumber,
          N: RealNumberSpace,
          D: Domain {
    fn scale(&mut self, factor: T) -> VoidResult {
        assert_real!(self);
        self.simd_real_operation(|x, y| x.scale_real(y), |x, y| x * y, factor, Complexity::Small);
        Ok(())
    }
}

impl<S, T, N, D> ScaleOps<Complex<T>> for DspVec<S, T, N, D>
    where S: ToSliceMut<T>,
          T: RealNumber,
          N: ComplexNumberSpace,
          D: Domain {
    fn scale(&mut self, factor: Complex<T>) -> VoidResult {
        assert_complex!(self);
        self.simd_complex_operation(|x,y| x.scale_complex(y), |x,y| x * y, factor, Complexity::Small);
        Ok(())
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
                let scalar_length = data_length % T::Reg::len();
                let vectorization_length = data_length - scalar_length;
                let mut array = self.data.to_slice_mut();
                let other = &$arg_name.data.to_slice();
                Chunk::from_src_to_dest(
                    Complexity::Small, &self.multicore_settings,
                    &other[0..vectorization_length], T::Reg::len(),
                    &mut array[0..vectorization_length], T::Reg::len(), (),
                    |original, range, target, _arg| {
                        let mut i = 0;
                        let mut j = range.start;
                        while i < target.len()
                        {
                            let vector1 = T::Reg::load_unchecked(original, j);
                            let vector2 = T::Reg::load_unchecked(target, i);
                            let result = vector2.$simd_op(vector1);
                            result.store_unchecked(target, i);
                            i += T::Reg::len();
                            j += T::Reg::len();
                        }
                });
                let mut i = vectorization_length;
                while i < data_length
                {
                    array[i] = array[i].$scal_op(other[i]);
                    i += 1;
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
                let scalar_length = data_length % T::Reg::len();
                let vectorization_length = data_length - scalar_length;
                let mut array = self.data.to_slice_mut();
                let other = &$arg_name.data.to_slice();
                Chunk::from_src_to_dest(
                    Complexity::Small, &self.multicore_settings,
                    &other[0..vectorization_length], T::Reg::len(),
                    &mut array[0..vectorization_length], T::Reg::len(), (),
                    |original, range, target, _arg| {
                        let mut i = 0;
                        let mut j = range.start;
                        while i < target.len()
                        {
                            let vector1 = T::Reg::load_unchecked(original, j);
                            let vector2 = T::Reg::load_unchecked(target, i);
                            let result = vector2.$simd_op(vector1);
                            result.store_unchecked(target, i);
                            i += T::Reg::len();
                            j += T::Reg::len();
                        }
                });
                let mut i = vectorization_length;
                while i < data_length
                {
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
                let scalar_length = data_length % T::Reg::len();
                let vectorization_length = data_length - scalar_length;
                let mut array = self.data.to_slice_mut();
                let other = &$arg_name.data.to_slice();
                Chunk::from_src_to_dest(
                    Complexity::Small, &self.multicore_settings,
                    &other, T::Reg::len(),
                    &mut array[0..vectorization_length], T::Reg::len(), (),
                    |original, range, target, _arg| {
                        // This parallelization likely doesn't make sense for the use
                        // case which we have in mind with this implementation
                        // so we likely have to revisit this code piece in future
                        let mut i = 0;
                        let mut j = range.start;
                        while i < target.len()
                        {
                            let vector1 =
                                if j + T::Reg::len() < original.len() {
                                    T::Reg::load_unchecked(original, j)
                                } else {
                                    T::Reg::load_wrap_unchecked(original, j)
                                };
                            let vector2 = T::Reg::load_unchecked(target, i);
                            let result = vector2.$simd_op(vector1);
                            result.store_unchecked(target, i);
                            i += T::Reg::len();
                            j = (j + T::Reg::len()) % original.len();
                        }
                });
                let mut i = vectorization_length;
                while i < data_length
                {
                    array[i] = array[i].$scal_op(other[i % $arg_name.len()]);
                    i += 1;
                }
            }

            Ok(())
        }
    }
}

macro_rules! impl_binary_smaller_complex_vector_operation {
    (fn $method: ident, $arg_name: ident, $simd_op: ident, $scal_op: ident) => {
        fn $method(&mut self, $arg_name: &Self) -> VoidResult
        {
            {
                let len = self.len();
                reject_if!(self, len % $arg_name.len() != 0, ErrorReason::InvalidArgumentLength);
                assert_meta_data!(self, $arg_name);

                let data_length = self.len();
                let scalar_length = data_length % T::Reg::len();
                let vectorization_length = data_length - scalar_length;
                let mut array = &mut self.data.to_slice_mut();
                let other = &$arg_name.data.to_slice();
                Chunk::from_src_to_dest(
                    Complexity::Small, &self.multicore_settings,
                    &other, T::Reg::len(),
                    &mut array[0..vectorization_length], T::Reg::len(), (),
                    |original, range, target, _arg| {
                        // This parallelization likely doesn't make sense for the use
                        // case which we have in mind with this implementation
                        // so we likely have to revisit this code piece in future
                        let mut i = 0;
                        let mut j = range.start;
                        while i < target.len()
                        {
                            let vector1 =
                                if j + T::Reg::len() < original.len() {
                                    T::Reg::load_unchecked(original, j)
                                } else {
                                    T::Reg::load_wrap_unchecked(original, j)
                                };
                            let vector2 = T::Reg::load_unchecked(target, i);
                            let result = vector2.$simd_op(vector1);
                            result.store_unchecked(target, i);
                            i += T::Reg::len();
                            j = (j + T::Reg::len()) % original.len();
                        }
                });
                let mut i = vectorization_length;
                while i < data_length
                {
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

impl<S, T, N, D> DspVec<S, T, N, D>
    where S: ToSliceMut<T>,
          T: RealNumber,
          N: RealNumberSpace,
          D: Domain {
    impl_binary_complex_vector_operation!(fn mul_complex, factor, mul_complex, mul);
    impl_binary_smaller_complex_vector_operation!(fn mul_smaller_complex, factor, mul_complex, mul);
    impl_binary_vector_operation!(fn mul_real, factor, mul, mul);
    impl_binary_smaller_vector_operation!(fn mul_smaller_real, factor, mul, mul);
    impl_binary_complex_vector_operation!(fn div_complex, divisor, div_complex, div);
    impl_binary_smaller_complex_vector_operation!(fn div_smaller_complex, divisor, div_complex, div);
    impl_binary_vector_operation!(fn div_real, divisor, div, div);
    impl_binary_smaller_vector_operation!(fn div_smaller_real, divisor, div, div);
}

impl<S, T, N, D> ElementaryOps for DspVec<S, T, N, D>
    where S: ToSliceMut<T>,
          T: RealNumber,
          N: RealNumberSpace,
          D: Domain {
      impl_binary_vector_operation!(fn add, summand, add, add);
      impl_binary_smaller_vector_operation!(fn add_smaller, summand, add, add);
      impl_binary_vector_operation!(fn sub, summand, sub, sub);
      impl_binary_smaller_vector_operation!(fn sub_smaller, summand, sub, sub);

      fn mul(&mut self, factor: &Self) -> VoidResult {
          let len = self.len();
          reject_if!(self, len != factor.len(), ErrorReason::InputMustHaveTheSameSize);
          assert_meta_data!(self, factor);

          if self.is_complex()
          {
              self.mul_complex(factor)
          }
          else
          {
              self.mul_real(factor)
          }
      }

      fn mul_smaller(&mut self, factor: &Self) -> VoidResult {
          let len = self.len();
          reject_if!(self, len % factor.len() != 0, ErrorReason::InvalidArgumentLength);
          assert_meta_data!(self, factor);

          if self.is_complex()
          {
              self.mul_smaller_complex(factor)
          }
          else
          {
              self.mul_smaller_real(factor)
          }
      }

      fn div(&mut self, divisor: &Self) -> VoidResult {
          let len = self.len();
          reject_if!(self, len != divisor.len(), ErrorReason::InputMustHaveTheSameSize);
          assert_meta_data!(self, divisor);

          if self.is_complex()
          {
              self.div_complex(divisor)
          }
          else
          {
              self.div_real(divisor)
          }
      }

      fn div_smaller(&mut self, divisor: &Self) -> VoidResult {
          let len = self.len();
          reject_if!(self, len % divisor.len() != 0, ErrorReason::InvalidArgumentLength);
          assert_meta_data!(self, divisor);

          if self.is_complex()
          {
              self.div_smaller_complex(divisor)
          }
          else
          {
              self.div_smaller_real(divisor)
          }
      }

}