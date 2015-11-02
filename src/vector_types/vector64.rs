use multicore_support::Chunk;
use super::general::{DataVector,DataVectorDomain};
use super::vector32::DataVector32;
use databuffer::DataBuffer;
use simd::f32x4;
use simd::x86::sse3::Sse3F32x4;
use simd_extensions::SimdExtensions32;
use num::complex::{Complex32,Complex64};
use std::mem;
use num::traits::Float;

define_vector_struct!(struct DataVector64, f64);
define_real_basic_struct_members!(impl DataVector64, DataVectorDomain::Time);
define_complex_basic_struct_members!(impl DataVector64, DataVectorDomain::Frequency);

define_vector_struct!(struct RealTimeVector64, f64);
define_real_basic_struct_members!(impl RealTimeVector64, DataVectorDomain::Time);
// define_generic_operations_forward!(from: RealTimeVector64, to: DataVector64);
define_real_operations_forward!(from: RealTimeVector64, to: DataVector64);

define_vector_struct!(struct RealFreqVector64, f64);
define_real_basic_struct_members!(impl RealFreqVector64, DataVectorDomain::Frequency);
// define_generic_operations_forward!(from: RealFreqVector64, to: DataVector64);
define_real_operations_forward!(from: RealFreqVector64, to: DataVector64);

define_vector_struct!(struct ComplexTimeVector64, f64);
define_complex_basic_struct_members!(impl ComplexTimeVector64, DataVectorDomain::Time);
// define_generic_operations_forward!(from: ComplexTimeVector64, to: DataVector64);
define_complex_operations_forward!(from: ComplexTimeVector64, to: DataVector64, complex: Complex64);

define_vector_struct!(struct ComplexFreqVector64, f64);
define_complex_basic_struct_members!(impl ComplexFreqVector64, DataVectorDomain::Frequency);
// define_generic_operations_forward!(from: ComplexFreqVector64, to: DataVector64);
define_complex_operations_forward!(from: ComplexFreqVector64, to: DataVector64, complex: Complex64);

const DEFAULT_GRANUALRITY: usize = 4;

#[inline]
impl<'a> DataVector64<'a>
{
	fn perfom_pending_operations(&mut self, buffer: &mut DataBuffer)
		-> DataVector64
	{
		panic!("Unimplemented");
		//DataVector64 { data: self.data, .. *self }
	}

	pub fn inplace_complex_offset(&mut self, offset: Complex64, buffer: &mut DataBuffer) 
	{
		panic!("Unimplemented");
		// self.inplace_offset(&[offset.re, offset.im], buffer);
	}
	
	pub fn inplace_real_offset(&mut self, offset: f64, buffer: &mut DataBuffer) 
	{
		panic!("Unimplemented");
		// self.inplace_offset(&[offset, offset], buffer);
	}
	/*
	fn inplace_offset(&mut self, offset: &[f64; 2], buffer: &mut DataBuffer) 
	{
		let data_length = self.len();
		let mut array = &mut self.data;
		Chunk::execute_partial_with_arguments(&mut array, data_length, 1, buffer, DataVector64::inplace_offset_parallel, offset);
	}
		
	fn inplace_offset_parallel(array: &mut [f64], increment_vector: &[f64;2])
	{
		let mut i = 0;
		while i < array.len()
		{ 
			array[i] = array[i] + increment_vector[i % 2];
			i += 1;
		}
	}*/

	pub fn inplace_real_scale(&mut self, factor: f64, buffer: &mut DataBuffer) 
	{
		panic!("Unimplemented");
		/*let data_length = self.len();
		let mut array = &mut self.data;
		Chunk::execute_partial_with_arguments(&mut array, data_length, 1, buffer, DataVector64::inplace_real_scale_par, factor);*/
	}
	/*
	fn inplace_real_scale_par(array: &mut [f64], factor: f64)
	{
		let mut i = 0;
		while i < array.len()
		{ 
			array[i] = array[i] * factor;
			i += 1;
		}
	}*/
	
	pub fn inplace_complex_scale(&mut self, factor: Complex64, buffer: &mut DataBuffer) 
	{
		panic!("Unimplemented");
		/*
		let data_length = self.len();
		let mut array = &mut self.data;
		Chunk::execute_partial_with_arguments(&mut array, data_length, 1, buffer, DataVector64::inplace_complex_scale_par, factor);*/
	}
	/*
	fn inplace_complex_scale_par(array: &mut [f64], factor: Complex64)
	{
		let mut i = 0;
		while i < array.len()
		{
			let real = array[i];
			let imag = array[i + 1];
			array[i] = real * factor.re - imag * factor.im;
			array[i + 1] = real * factor.im + imag * factor.re;
			i += 2;
		}
	}*/
		
	pub fn inplace_real_abs(&mut self, buffer: &mut DataBuffer)
	{
		panic!("Unimplemented");
		/*let mut array = &mut self.data;
		let length = array.len();
		Chunk::execute_partial(&mut array, length, 1, buffer, DataVector32::inplace_abs_real_par);*/
	}
	
	pub fn inplace_complex_abs(&mut self, buffer: &mut DataBuffer)
	{
		panic!("Unimplemented");
		/*
		let data_length = self.len();
		let mut array = &mut self.data;
		Chunk::execute_partial(&mut array, data_length, 1, buffer, DataVector64::inplace_complex_abs_par);*/
	}
	/*
	fn inplace_complex_abs_par(array: &mut [f64])
	{
		let mut i = 0;
		while i < array.len()
		{ 
			let real = array[i];
			let imag = array[i + 1];
			array[i / 2] = (real * real + imag * imag).sqrt();
			i += 2;
		}
	}*/
	
	pub fn inplace_complex_abs_squared(&mut self, buffer: &mut DataBuffer)
	{
		panic!("Unimplemented");
		/*
		let data_length = self.len();
		let mut array = &mut self.data;
		Chunk::execute_partial(&mut array, data_length, 1, buffer, DataVector64::inplace_complex_abs_squared_par);*/
	}
	
	/*
	fn inplace_complex_abs_squared_par(array: &mut [f64])
	{
		let mut i = 0;
		while i < array.len()
		{ 
			let real = array[i];
			let imag = array[i + 1];
			array[i / 2] = real * real + imag * imag;
			i += 2;
		}
	}*/
}
/*
#[test]
fn add_real_one_64_test()
{
	let mut data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
	let mut result = DataVector64::from_array(&mut data);
	let mut buffer = DataBuffer::new("test");
	result.inplace_real_offset(1.0, &mut buffer);
	let expected = [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
	assert_eq!(result.data, expected);
	assert_eq!(result.delta, 1.0);
}

#[test]
fn multiply_real_two_64_test()
{
	let mut data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
	let mut result = DataVector64::from_array(&mut data);
	let mut buffer = DataBuffer::new("test");
	result.inplace_real_scale(2.0, &mut buffer);
	let expected = [2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0];
	assert_eq!(result.data, expected);
	assert_eq!(result.delta, 1.0);
}

#[test]
fn multiply_complex_64_test()
{
	let mut data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
	let mut result = DataVector64::from_interleaved(&mut data);
	let mut buffer = DataBuffer::new("test");
	result.inplace_complex_scale(Complex64::new(2.0, -3.0), &mut buffer);
	let expected = [8.0, 1.0, 18.0, -1.0, 28.0, -3.0, 38.0, -5.0];
	assert_eq!(result.data, expected);
	assert_eq!(result.delta, 1.0);
}*/