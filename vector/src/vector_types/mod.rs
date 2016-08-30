macro_rules! reject_if {
    ($self_: ident, $condition: expr, $message: expr) => {
        if $condition {
            return Err(($message, $self_));
        }
    }
}

macro_rules! assert_meta_data {
    ($self_: ident, $other: ident) => {
         {
            let delta_ratio = $self_.delta / $other.delta;
            if $self_.is_complex != $other.is_complex ||
                $self_.domain != $other.domain ||
                delta_ratio > 1.1 || delta_ratio < 0.9 {
                return Err((ErrorReason::InputMetaDataMustAgree, $self_));
            }
         }
    }
}

macro_rules! assert_real {
    ($self_: ident) => {
        if $self_.is_complex {
            return Err((ErrorReason::InputMustBeReal, $self_));
        }
    }
}

macro_rules! assert_complex {
    ($self_: ident) => {
        if !$self_.is_complex {
            return Err((ErrorReason::InputMustBeComplex, $self_));
        }
    }
}

macro_rules! assert_time {
    ($self_: ident) => {
        if $self_.domain != DataVecDomain::Time {
            return Err((ErrorReason::InputMustBeInTimeDomain, $self_));
        }
    }
}

macro_rules! assert_freq {
    ($self_: ident) => {
        if $self_.domain != DataVecDomain::Frequency {
            return Err((ErrorReason::InputMustBeInFrquencyDomain, $self_));
        }
    }
}

macro_rules! temp_mut {
    ($self_: ident, $len: expr) => {
        if $self_.temp.len() < $len {
            $self_.temp = Vec::with_capacity(round_len($len));
            unsafe { $self_.temp.set_len($len); }
            &mut $self_.temp
        }
        else {
            &mut $self_.temp
        }
    }
}

#[macro_use]
mod struct_macros;
#[macro_use]
mod real_forward;
#[macro_use]
mod complex_forward;
#[macro_use]
mod general_forward;
#[macro_use]
mod basic_functions;
pub mod definitions;
pub mod general_impl;
pub mod real_impl;
pub mod complex_impl;
pub mod time_freq_impl;
pub mod convolution_impl;
pub mod correlation_impl;
pub mod interpolation_impl;
mod rededicate_impl;
mod stats_impl;
pub mod multi_ops;
pub mod operations_enum;

pub use vector_types::definitions::{
        DataVecDomain,
        DataVec,
        TransRes,
        VoidResult,
        ScalarResult,
        ErrorReason,
        GenericVectorOps,
        RealVectorOps,
        ComplexVectorOps,
        Statistics,
        RededicateOps,
        ScaleOps,
        OffsetOps,
        DotProductOps,
        StatisticsOps,
        PaddingOption,
        VectorIter,
        ComplexIndex,
        ComplexIndexMut,
        InterleavedIndex,
        InterleavedIndexMut,
        RealIndex,
        RealIndexMut
    };
pub use vector_types::time_freq_impl::{
        TimeDomainOperations,
        FrequencyDomainOperations,
        SymmetricFrequencyDomainOperations,
        SymmetricTimeDomainOperations
    };
pub use vector_types::convolution_impl::{
    Convolution,
    ConvolutionOps,
    FrequencyMultiplication};
pub use vector_types::correlation_impl::CrossCorrelationOps;
pub use vector_types::interpolation_impl::{
    InterpolationOps,
    RealInterpolationOps};
use num::complex::Complex;
use RealNumber;
use multicore_support::{Chunk, Complexity, MultiCoreSettings};
use std::mem;
use std::ptr;
use simd_extensions::*;
use std::ops::{Index, IndexMut, Range, RangeTo, RangeFrom, RangeFull};
use num::traits::Zero;
use std::ops::Mul;
use std::fmt::{Display, Debug};

fn round_len(len: usize) -> usize {
    ((len + Reg32::len() - 1) / Reg32::len()) * Reg32::len()
}

define_vector_struct!(struct GenericDataVec);
add_basic_private_impl!(f32, Reg32; f64, Reg64);
add_real_accessors!(GenericDataVec);
add_complex_accessors!(GenericDataVec);

define_vector_struct!(struct RealTimeVector);
define_real_basic_struct_members!(impl RealTimeVector, DataVecDomain::Time);
add_real_accessors!(RealTimeVector);
define_generic_operations_forward!(from: RealTimeVector, to: GenericDataVec, f32, f64);
define_real_operations_forward!(from: RealTimeVector, to: GenericDataVec, complex_partner: ComplexTimeVector, f32, f64);

define_vector_struct!(struct RealFreqVector);
define_real_basic_struct_members!(impl RealFreqVector, DataVecDomain::Frequency);
add_real_accessors!(RealFreqVector);
define_generic_operations_forward!(from: RealFreqVector, to: GenericDataVec, f32, f64);
define_real_operations_forward!(from: RealFreqVector, to: GenericDataVec, complex_partner: ComplexFreqVector, f32, f64);

define_vector_struct!(struct ComplexTimeVector);
add_complex_accessors!(ComplexTimeVector);
define_complex_basic_struct_members!(impl ComplexTimeVector, DataVecDomain::Time);
define_generic_operations_forward!(from: ComplexTimeVector, to: GenericDataVec, f32, f64);
define_complex_operations_forward!(from: ComplexTimeVector, to: GenericDataVec, complex: Complex, real_partner: RealTimeVector, f32, f64);

define_vector_struct!(struct ComplexFreqVector);
add_complex_accessors!(ComplexFreqVector);
define_complex_basic_struct_members!(impl ComplexFreqVector, DataVecDomain::Frequency);
define_generic_operations_forward!(from: ComplexFreqVector, to: GenericDataVec, f32, f64);
define_complex_operations_forward!(from: ComplexFreqVector, to: GenericDataVec, complex: Complex, real_partner: RealTimeVector, f32, f64);

define_vector_struct_type_alias!(struct DataVec32, based_on: GenericDataVec, f32);
define_vector_struct_type_alias!(struct RealTimeVector32, based_on: RealTimeVector, f32);
define_vector_struct_type_alias!(struct RealFreqVector32, based_on: RealFreqVector, f32);
define_vector_struct_type_alias!(struct ComplexTimeVector32, based_on: ComplexTimeVector, f32);
define_vector_struct_type_alias!(struct ComplexFreqVector32, based_on: ComplexFreqVector, f32);
define_vector_struct_type_alias!(struct DataVec64, based_on: GenericDataVec, f64);
define_vector_struct_type_alias!(struct RealTimeVector64, based_on: RealTimeVector, f64);
define_vector_struct_type_alias!(struct RealFreqVector64, based_on: RealFreqVector, f64);
define_vector_struct_type_alias!(struct ComplexTimeVector64, based_on: ComplexTimeVector, f64);
define_vector_struct_type_alias!(struct ComplexFreqVector64, based_on: ComplexFreqVector, f64);

impl<T> GenericDataVec<T>
    where T: RealNumber
{
    fn swap_data_temp(mut self) -> Self
    {
        let temp = self.temp;
        self.temp = self.data;
        self.data = temp;
        self
    }
}

pub fn array_to_complex<T>(array: &[T]) -> &[Complex<T>] {
    unsafe {
        let len = array.len();
        if len % 2 != 0 {
            panic!("Argument must have an even length");
        }
        let trans: &[Complex<T>] = mem::transmute(array);
        &trans[0 .. len / 2]
    }
}

pub fn array_to_complex_mut<T>(array: &mut [T]) -> &mut [Complex<T>] {
    unsafe {
        let len = array.len();
        if len % 2 != 0 {
            panic!("Argument must have an even length");
        }
        let trans: &mut [Complex<T>] = mem::transmute(array);
        &mut trans[0 .. len / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::complex::Complex32;

    #[test]
    fn construct_real_time_vector_32_test()
    {
        let array = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let vector = RealTimeVector32::from_array(&array);
        assert_eq!(vector.data, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        assert_eq!(vector.delta(), 1.0);
        assert_eq!(vector.domain(), DataVecDomain::Time);
    }

    #[test]
    fn construct_complex_time_vector_32_test()
    {
        let array = [Complex32::new(1.0, 2.0), Complex32::new(3.0, 4.0), Complex32::new(5.0, 6.0), Complex32::new(7.0, 8.0)];
        let vector = ComplexTimeVector32::from_complex(&array);
        assert_eq!(vector.data, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        assert_eq!(vector.complex(0..), &array);
    }

    #[test]
    fn add_real_one_32_test()
    {
        let mut data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let vector = RealTimeVector32::from_array(&mut data);
        let result = vector.real_offset(1.0).unwrap();
        let expected = [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        assert_eq!(result.data, expected);
        assert_eq!(result.delta, 1.0);
    }

    #[test]
    fn add_real_two_32_test()
    {
        // Test also that vectors may be passed to from_array
        let data = vec!(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0);
        let result = RealTimeVector32::from_array(&data);
        let result = result.real_offset(2.0).unwrap();
        assert_eq!(result.data, [3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
        assert_eq!(result.delta, 1.0);
    }

    #[test]
    fn add_complex_32_test()
    {
        let data = vec!(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let result = ComplexTimeVector32::from_interleaved(&data);
        let result = result.complex_offset(Complex32::new(1.0, -1.0)).unwrap();
        assert_eq!(result.data, [2.0, 1.0, 4.0, 3.0, 6.0, 5.0, 8.0, 7.0]);
        assert_eq!(result.delta, 1.0);
    }

    #[test]
    fn multiply_real_two_32_test()
    {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let result = RealTimeVector32::from_array(&data);
        let result = result.real_scale(2.0).unwrap();
        let expected = [2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0];
        assert_eq!(result.data, expected);
        assert_eq!(result.delta, 1.0);
    }

    #[test]
    fn multiply_complex_32_test()
    {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let result = ComplexTimeVector32::from_interleaved(&data);
        let result = result.complex_scale(Complex32::new(2.0, -3.0)).unwrap();
        let expected = [8.0, 1.0, 18.0, -1.0, 28.0, -3.0, 38.0, -5.0];
        assert_eq!(result.data, expected);
        assert_eq!(result.delta, 1.0);
    }

    #[test]
    fn abs_real_32_test()
    {
        let data = [-1.0, 2.0, -3.0, 4.0, -5.0, -6.0, 7.0, -8.0];
        let result = RealTimeVector32::from_array(&data);
        let result = result.abs().unwrap();
        let expected = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        assert_eq!(result.data, expected);
        assert_eq!(result.delta, 1.0);
    }

    #[test]
    fn abs_complex_32_test()
    {
        let data = [3.0, 4.0, -3.0, 4.0, 3.0, -4.0, -3.0, -4.0];
        let result = ComplexTimeVector32::from_interleaved(&data);
        let result = result.magnitude().unwrap();
        let expected = [5.0, 5.0, 5.0, 5.0];
        assert_eq!(result.real(0..), expected);
        assert_eq!(result.delta, 1.0);
    }

    #[test]
    fn abs_complex_squared_32_test()
    {
        let data = [-1.0, 2.0, -3.0, 4.0, -5.0, -6.0, 7.0, -8.0, 9.0, 10.0];
        let result = ComplexTimeVector32::from_interleaved(&data);
        let result = result.magnitude_squared().unwrap();
        let expected = [5.0, 25.0, 61.0, 113.0, 181.0];
        assert_eq!(result.real(0..), expected);
        assert_eq!(result.delta, 1.0);
    }

    #[test]
    fn indexer_test()
    {
        let data = [1.0, 2.0, 3.0, 4.0];
        let mut result = ComplexTimeVector32::from_interleaved(&data);
        assert_eq!(result[0], 1.0);
        result[0] = 5.0;
        assert_eq!(result[0], 5.0);
        let expected = [5.0, 2.0, 3.0, 4.0];
        assert_eq!(result.interleaved(0..), expected);
    }

    #[test]
    fn add_test()
    {
        let data1 = [1.0, 2.0, 3.0, 4.0];
        let vector1 = ComplexTimeVector32::from_interleaved(&data1);
        let data2 = [5.0, 7.0, 9.0, 11.0];
        let vector2 = ComplexTimeVector32::from_interleaved(&data2);
        let result = vector1.add(&vector2).unwrap();
        let expected = [6.0, 9.0, 12.0, 15.0];
        assert_eq!(result.interleaved(0..), expected);
    }

    #[test]
    fn multiply_complex_vector_32_test()
    {
        let a = ComplexTimeVector32::from_interleaved(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        let b = ComplexTimeVector32::from_interleaved(&[2.0, -3.0, -2.0, 3.0, 2.0, -3.0, -2.0, 3.0]);
        let result = a.mul(&b).unwrap();
        let expected = [8.0, 1.0, -18.0, 1.0, 28.0, -3.0, -38.0, 5.0];
        assert_eq!(result.data, expected);
        assert_eq!(result.delta, 1.0);
    }

    #[test]
    fn divide_complex_vector_32_test()
    {
        let a = ComplexTimeVector32::from_interleaved(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = ComplexTimeVector32::from_interleaved(&[-1.0, 0.0, 0.0, 1.0, 2.0, -3.0]);
        let result = a.div(&b).unwrap();
        let expected = [-1.0, -2.0, 4.0, -3.0, -8.0/13.0, 27.0/13.0];
        assert_eq!(result.data, expected);
        assert_eq!(result.delta, 1.0);
    }

    #[test]
    fn array_to_complex_test()
    {
        let a = [1.0; 10];
        let c = super::array_to_complex(&a);
        let expected = [Complex32::new(1.0, 1.0); 5];
        assert_eq!(&expected, c);
    }

    #[test]
    fn array_to_complex_mut_test()
    {
        let mut a = [1.0; 10];
        let c = super::array_to_complex_mut(&mut a);
        let expected = [Complex32::new(1.0, 1.0); 5];
        assert_eq!(&expected, c);
    }

    #[test]
    fn swap_halves_real_even_test()
    {
        let mut a = [1.0, 2.0, 3.0, 4.0];
        let c = RealTimeVector32::from_array(&mut a);
        let r = c.swap_halves().unwrap();
        assert_eq!(r.real(0..), &[3.0, 4.0, 1.0, 2.0]);
    }

    #[test]
    fn swap_halves_real_odd_test()
    {
        let mut a = [1.0, 2.0, 3.0, 4.0, 5.0];
        let c = RealTimeVector32::from_array(&mut a);
        let r = c.swap_halves().unwrap();
        assert_eq!(r.real(0..), &[4.0, 5.0, 1.0, 2.0, 3.0]);
    }

    #[test]
    fn swap_halves_complex_even_test()
    {
        let mut a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let c = ComplexTimeVector32::from_interleaved(&mut a);
        let r = c.swap_halves().unwrap();
        assert_eq!(r.interleaved(0..), &[5.0, 6.0, 7.0, 8.0, 1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn swap_halves_complex_odd_test()
    {
        let mut a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let c = ComplexTimeVector32::from_interleaved(&mut a);
        let r = c.swap_halves().unwrap();
        assert_eq!(r.interleaved(0..), &[7.0, 8.0, 9.0, 10.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn zero_pad_end_test()
    {
        let mut a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let c = ComplexTimeVector32::from_interleaved(&mut a);
        let r = c.zero_pad(9, PaddingOption::End).unwrap();
        let expected =
            [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0,
             0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert_eq!(r.interleaved(0..), &expected);
    }

    #[test]
    fn zero_pad_surround_test()
    {
        let mut a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let c = ComplexTimeVector32::from_interleaved(&mut a);
        let r = c.zero_pad(10, PaddingOption::Surround).unwrap();
        let expected =
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
             1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0,
             0.0, 0.0, 0.0, 0.0];
        assert_eq!(r.interleaved(0..), &expected);
    }

    #[test]
    fn zero_pad_center_test()
    {
        let mut a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let c = ComplexFreqVector32::from_interleaved(&mut a);
        let r = c.zero_pad(10, PaddingOption::Center).unwrap();
        let r = r.fft_shift().unwrap();
        let expected =
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
             7.0, 8.0, 9.0, 10.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0,
             0.0, 0.0, 0.0, 0.0];
        assert_eq!(r.interleaved(0..), &expected);
    }

    #[test]
    fn complex_conj_test()
    {
        let mut a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let c = ComplexFreqVector32::from_interleaved(&mut a);
        let r = c.conj().unwrap();
        let expected =
            [1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0, 9.0, -10.0];
        assert_eq!(r.interleaved(0..), &expected);
    }
}
