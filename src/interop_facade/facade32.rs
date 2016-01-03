use super::*;
use vector_types:: {
		DataVectorDomain,
		DataVector,
		GenericVectorOperations,
		RealVectorOperations,
		ComplexVectorOperations,
		TimeDomainOperations,
		FrequencyDomainOperations,
		DataVector32, 
        Statistics};
use num::complex::Complex32;
use std::slice;

#[no_mangle]
pub extern fn delete_vector32(vector: Box<DataVector32>) {
    drop(vector);
}
 
#[no_mangle]
pub extern fn new32(is_complex: i32, domain: i32, init_value: f32, length: usize, delta: f32) -> Box<DataVector32> {
    let domain = if domain == 0 {
            DataVectorDomain::Time
        }
        else {
            DataVectorDomain::Frequency
        };
        
	let vector = Box::new(DataVector32::new(is_complex != 0, domain, init_value, length, delta));
    vector
}

#[no_mangle]
pub extern fn get_value32(vector: &DataVector32, index: usize) -> f32 {
    vector[index]
}

#[no_mangle]
pub extern fn set_value32(vector: &mut DataVector32, index: usize, value : f32) {
    vector[index] = value;
}

#[no_mangle]
pub extern fn is_complex32(vector: &DataVector32) -> i32 {
    if vector.is_complex() {
        1
    } 
    else {
        0
    }
}

/// Returns the vector domain as integer:
/// 0 for time domain
/// 1 for frequency domain
/// if the function returns another value then please report a bug.
#[no_mangle]
pub extern fn get_domain32(vector: &DataVector32) -> i32 {
    match vector.domain() {
        DataVectorDomain::Time => 0,
        DataVectorDomain::Frequency => 1,
    }
}

#[no_mangle]
pub extern fn get_len32(vector: &DataVector32) -> usize {
    vector.len()
}

#[no_mangle]
pub extern fn get_points32(vector: &DataVector32) -> usize {
    vector.points()
}

#[no_mangle]
pub extern fn get_delta32(vector: &DataVector32) -> f32 {
    vector.delta()
}

#[no_mangle]
pub extern fn get_allocated_len32(vector: &DataVector32) -> usize {
    vector.allocated_len()
}

#[no_mangle]
pub extern fn add_vector32(vector: Box<DataVector32>, operand: &DataVector32) -> VectorResult<DataVector32> {
    convert_vec!(vector.add_vector(operand))
}

#[no_mangle]
pub extern fn subtract_vector32(vector: Box<DataVector32>, operand: &DataVector32) -> VectorResult<DataVector32> {
    convert_vec!(vector.subtract_vector(operand))
}

#[no_mangle]
pub extern fn divide_vector32(vector: Box<DataVector32>, operand: &DataVector32) -> VectorResult<DataVector32> {
    convert_vec!(vector.divide_vector(operand))
}

#[no_mangle]
pub extern fn multiply_vector32(vector: Box<DataVector32>, operand: &DataVector32) -> VectorResult<DataVector32> {
    convert_vec!(vector.multiply_vector(operand))
}

#[no_mangle]
pub extern fn real_dot_product32(vector: &DataVector32, operand: &DataVector32) -> ScalarResult<f32> {
    convert_scalar!(vector.real_dot_product(operand), 0.0)
}

#[no_mangle]
pub extern fn complex_dot_product32(vector: &DataVector32, operand: &DataVector32) -> ScalarResult<Complex32> {
    convert_scalar!(vector.complex_dot_product(operand), Complex32::new(0.0, 0.0))
}

#[no_mangle]
pub extern fn real_statistics32(vector: &DataVector32) -> Statistics<f32> {
    vector.real_statistics()
}

#[no_mangle]
pub extern fn complex_statistics32(vector: &DataVector32) -> Statistics<Complex32> {
    vector.complex_statistics()
}

#[no_mangle]
pub extern fn zero_pad32(vector: Box<DataVector32>, points: usize) -> VectorResult<DataVector32> {
    convert_vec!(vector.zero_pad(points))
}

#[no_mangle]
pub extern fn zero_interleave32(vector: Box<DataVector32>, factor: i32) -> VectorResult<DataVector32> {
    convert_vec!(vector.zero_interleave(factor as u32))
}

#[no_mangle]
pub extern fn diff32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.diff())
}

#[no_mangle]
pub extern fn diff_with_start32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.diff_with_start())
}

#[no_mangle]
pub extern fn cum_sum32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.cum_sum())
}

#[no_mangle]
pub extern fn real_offset32(vector: Box<DataVector32>, value: f32) -> VectorResult<DataVector32> {
    convert_vec!(vector.real_offset(value))
}

#[no_mangle]
pub extern fn real_scale32(vector: Box<DataVector32>, value: f32) -> VectorResult<DataVector32> {
    convert_vec!(vector.real_scale(value))
}

#[no_mangle]
pub extern fn abs32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.abs())
}

#[no_mangle]
pub extern fn sqrt32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.sqrt())
}

#[no_mangle]
pub extern fn square32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.square())
}

#[no_mangle]
pub extern fn root32(vector: Box<DataVector32>, value: f32) -> VectorResult<DataVector32> {
    convert_vec!(vector.root(value))
}

#[no_mangle]
pub extern fn power32(vector: Box<DataVector32>, value: f32) -> VectorResult<DataVector32> {
    convert_vec!(vector.power(value))
}

#[no_mangle]
pub extern fn logn32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.logn())
}

#[no_mangle]
pub extern fn expn32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.expn())
}

#[no_mangle]
pub extern fn log_base32(vector: Box<DataVector32>, value: f32) -> VectorResult<DataVector32> {
    convert_vec!(vector.log_base(value))
}

#[no_mangle]
pub extern fn exp_base32(vector: Box<DataVector32>, value: f32) -> VectorResult<DataVector32> {
    convert_vec!(vector.exp_base(value))
}

#[no_mangle]
pub extern fn to_complex32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.to_complex())
}

#[no_mangle]
pub extern fn sin32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.sin())
}

#[no_mangle]
pub extern fn cos32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.cos())
}

#[no_mangle]
pub extern fn tan32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.tan())
}

#[no_mangle]
pub extern fn asin32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.asin())
}

#[no_mangle]
pub extern fn acos32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.acos())
}

#[no_mangle]
pub extern fn atan32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.tan())
}

#[no_mangle]
pub extern fn sinh32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.sinh())
}
#[no_mangle]
pub extern fn cosh32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.cosh())
}

#[no_mangle]
pub extern fn tanh32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.tanh())
}

#[no_mangle]
pub extern fn asinh32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.asinh())
}

#[no_mangle]
pub extern fn acosh32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.acosh())
}

#[no_mangle]
pub extern fn atanh32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.atanh())
}

#[no_mangle]
pub extern fn wrap32(vector: Box<DataVector32>, value: f32) -> VectorResult<DataVector32> {
    convert_vec!(vector.wrap(value))
}

#[no_mangle]
pub extern fn unwrap32(vector: Box<DataVector32>, value: f32) -> VectorResult<DataVector32> {
    convert_vec!(vector.unwrap(value))
}

#[no_mangle]
pub extern fn swap_halves32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.swap_halves())
}

#[no_mangle]
pub extern fn complex_offset32(vector: Box<DataVector32>, real: f32, imag: f32) -> VectorResult<DataVector32> {
    convert_vec!(vector.complex_offset(Complex32::new(real, imag)))
}

#[no_mangle]
pub extern fn complex_scale32(vector: Box<DataVector32>, real: f32, imag: f32) -> VectorResult<DataVector32> {
    convert_vec!(vector.complex_scale(Complex32::new(real, imag)))
}

#[no_mangle]
pub extern fn complex_divide32(vector: Box<DataVector32>, real: f32, imag: f32) -> VectorResult<DataVector32> {
    convert_vec!(vector.complex_scale(Complex32::new(1.0, 0.0) / Complex32::new(real, imag)))
}

#[no_mangle]
pub extern fn magnitude32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.magnitude())
}

#[no_mangle]
pub extern fn get_magnitude32(vector: Box<DataVector32>, destination: &mut DataVector32) -> i32 {
    convert_void!(vector.get_magnitude(destination))
}

#[no_mangle]
pub extern fn magnitude_squared32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.magnitude_squared())
}

#[no_mangle]
pub extern fn complex_conj32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.complex_conj())
}

#[no_mangle]
pub extern fn to_real32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.to_real())
}

#[no_mangle]
pub extern fn to_imag32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.to_imag())
}

#[no_mangle]
pub extern fn get_real32(vector: Box<DataVector32>, destination: &mut DataVector32) -> i32 {
    convert_void!(vector.get_real(destination))
}

#[no_mangle]
pub extern fn get_imag32(vector: Box<DataVector32>, destination: &mut DataVector32) -> i32 {
    convert_void!(vector.get_imag(destination))
}

#[no_mangle]
pub extern fn phase32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.phase())
}

#[no_mangle]
pub extern fn get_phase32(vector: Box<DataVector32>, destination: &mut DataVector32) -> i32 {
    convert_void!(vector.get_phase(destination))
}

#[no_mangle]
pub extern fn plain_fft32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.plain_fft())
}

#[no_mangle]
pub extern fn plain_ifft32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.plain_ifft())
}

#[no_mangle]
pub extern fn clone32(vector: Box<DataVector32>) -> Box<DataVector32> {
    vector.clone()
}

#[no_mangle]
pub extern fn multiply_complex_exponential32(vector: Box<DataVector32>, a: f32, b: f32) -> VectorResult<DataVector32> {
    convert_vec!(vector.multiply_complex_exponential(a, b))
}

#[no_mangle]
pub extern fn add_smaller_vector32(vector: Box<DataVector32>, operand: &DataVector32) -> VectorResult<DataVector32> {
    convert_vec!(vector.add_smaller_vector(operand))
}

#[no_mangle]
pub extern fn subtract_smaller_vector32(vector: Box<DataVector32>, operand: &DataVector32) -> VectorResult<DataVector32> {
    convert_vec!(vector.subtract_smaller_vector(operand))
}

#[no_mangle]
pub extern fn divide_smaller_vector32(vector: Box<DataVector32>, operand: &DataVector32) -> VectorResult<DataVector32> {
    convert_vec!(vector.divide_smaller_vector(operand))
}

#[no_mangle]
pub extern fn multiply_smaller_vector32(vector: Box<DataVector32>, operand: &DataVector32) -> VectorResult<DataVector32> {
    convert_vec!(vector.multiply_smaller_vector(operand))
}

#[no_mangle]
pub extern fn get_real_imag32(vector: Box<DataVector32>, real: &mut DataVector32, imag: &mut DataVector32) -> i32 {
    convert_void!(vector.get_real_imag(real, imag))
}

#[no_mangle]
pub extern fn get_mag_phase32(vector: Box<DataVector32>, mag: &mut DataVector32, phase: &mut DataVector32) -> i32 {
    convert_void!(vector.get_mag_phase(mag, phase))
}

#[no_mangle]
pub extern fn set_real_imag32(vector: Box<DataVector32>, real: &DataVector32, imag: &DataVector32) -> VectorResult<DataVector32> {
    convert_vec!(vector.set_real_imag(real, imag))
}

#[no_mangle]
pub extern fn set_mag_phase32(vector: Box<DataVector32>, mag: &DataVector32, phase: &DataVector32) -> VectorResult<DataVector32> {
    convert_vec!(vector.set_mag_phase(mag, phase))
}

#[no_mangle]
pub extern fn split_into32(vector: Box<DataVector32>, targets: &mut [Box<DataVector32>]) -> i32 {
    convert_void!(vector.split_into(targets))
}

#[no_mangle]
pub extern fn merge32(vector: Box<DataVector32>, sources: &[Box<DataVector32>]) -> VectorResult<DataVector32> {
    convert_vec!(vector.merge(sources))
}

#[no_mangle]
pub extern fn override_data32(vector: Box<DataVector32>, data: *const f32, len: usize) -> VectorResult<DataVector32> {
    let data = unsafe { slice::from_raw_parts(data, len) };
    convert_vec!(vector.override_data(data))
}

#[no_mangle]
pub extern fn real_statistics_splitted32(vector: &DataVector32, data: *mut Statistics<f32>, len: usize) -> i32 {
    let mut data = unsafe { slice::from_raw_parts_mut(data, len) };
    let stats = vector.real_statistics_splitted(data.len());
    for i in 0..stats.len() {
        data[i] = stats[i];
    }
    
    0
}

#[no_mangle]
pub extern fn complex_statistics_splitted32(vector: &DataVector32, data: *mut Statistics<Complex32>, len: usize) -> i32 {
    let mut data = unsafe { slice::from_raw_parts_mut(data, len) };
    let stats = vector.complex_statistics_splitted(data.len());
    for i in 0..stats.len() {
        data[i] = stats[i];
    }
    
    0
}

#[no_mangle]
pub extern fn fft32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.fft())
}

#[no_mangle]
pub extern fn ifft32(vector: Box<DataVector32>) -> VectorResult<DataVector32> {
    convert_vec!(vector.ifft())
}

#[no_mangle]
pub extern fn plain_sifft32(vector: Box<DataVector32>, even_odd: i32) -> VectorResult<DataVector32> {
    let even_odd = translate_to_even_odd(even_odd);
    convert_vec!(vector.plain_sifft(even_odd))
}

#[no_mangle]
pub extern fn sifft32(vector: Box<DataVector32>, even_odd: i32) -> VectorResult<DataVector32> {
    let even_odd = translate_to_even_odd(even_odd);
    convert_vec!(vector.sifft(even_odd))
}

#[no_mangle]
pub extern fn mirror32(vector: Box<DataVector32>, even_odd: i32) -> VectorResult<DataVector32> {
    let even_odd = translate_to_even_odd(even_odd);
    convert_vec!(vector.mirror(even_odd))
}

#[no_mangle]
pub extern fn apply_window32(vector: Box<DataVector32>, window: i32) -> VectorResult<DataVector32> {
    let window = translate_to_window_function(window);
    convert_vec!(vector.apply_window(window.as_ref()))
}

#[no_mangle]
pub extern fn unapply_window32(vector: Box<DataVector32>, window: i32) -> VectorResult<DataVector32> {
    let window = translate_to_window_function(window);
    convert_vec!(vector.unapply_window(window.as_ref()))
}


#[no_mangle]
pub extern fn windowed_fft32(vector: Box<DataVector32>, window: i32) -> VectorResult<DataVector32> {
    let window = translate_to_window_function(window);
    convert_vec!(vector.windowed_fft(window.as_ref()))
}

#[no_mangle]
pub extern fn windowed_ifft32(vector: Box<DataVector32>, window: i32) -> VectorResult<DataVector32> {
    let window = translate_to_window_function(window);
    convert_vec!(vector.windowed_ifft(window.as_ref()))
}

#[no_mangle]
pub extern fn windowed_sifft32(vector: Box<DataVector32>, even_odd: i32, window: i32) -> VectorResult<DataVector32> {
    let even_odd = translate_to_even_odd(even_odd);
    let window = translate_to_window_function(window);
    convert_vec!(vector.windowed_sifft(even_odd, window.as_ref()))
}