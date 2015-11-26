use rand::*;
use basic_dsp::DataVector;
use std::ops::Range;
use std::thread;

pub fn assert_vector_eq_with_reason(left: &[f32], right: &[f32], reason: &str) {
    assert_vector_eq_with_reason_and_tolerance(left, right, 1e-6, reason);
}

pub fn assert_vector_eq_with_reason_and_tolerance(left: &[f32], right: &[f32], tolerance: f32, reason: &str) {
    let mut errors = Vec::new();
    if reason.len() > 0
    {
        errors.push(format!("{}:\n", reason));
    }
    
    if left.len() != right.len()
    {
        errors.push(format!("Size difference {} != {}", left.len(), right.len()));
    }
    
    let len = if left.len() < right.len() { left.len() } else { right.len() };
    let mut differences = 0;
    let mut first_difference = false;
    for i in 0 .. len {
        if (left[i] - right[i]).abs() > tolerance
        {
            differences += 1;
            if !first_difference
            {
                errors.push(format!("First difference at index {}, left: {} != right: {}", i, left[i], right[i]));
                first_difference = true;
            }
        }
    }
    
    if differences > 0
    {
        errors.push(format!("Total number of differences: {}/{}={}%", differences, len, differences*100/len));
    }
    
    if differences > 0
    {
        let all_errors = errors.join("\n");
        let header = "-----------------------".to_owned();
        let full_text = format!("\n{}\n{}\n{}\n", header, all_errors, header);
        panic!(full_text);
    }
}
    
pub fn assert_vector_eq(left: &[f32], right: &[f32]) {
    assert_vector_eq_with_reason(left, right, "");
}

pub fn create_data(seed: usize, iteration: usize, from: usize, to: usize) -> Vec<f32>
{
    let len_seed: &[_] = &[seed, iteration];
    let mut rng: StdRng = SeedableRng::from_seed(len_seed);
    let len = rng.gen_range(from, to);
    create_data_with_len(seed, iteration, len)
}

pub fn create_data_even(seed: usize, iteration: usize, from: usize, to: usize) -> Vec<f32>
{
    let len_seed: &[_] = &[seed, iteration];
    let mut rng: StdRng = SeedableRng::from_seed(len_seed);
    let len = rng.gen_range(from, to);
    let len = len + len % 2;
    create_data_with_len(seed, iteration, len)
}

pub fn create_data_with_len(seed: usize, iteration: usize, len: usize) -> Vec<f32>
{
    let seed: &[_] = &[seed, iteration];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut data = vec![0.0; len];
    for i in 0..len {
        data[i] = rng.gen_range(-10.0, 10.0);
    }
    data
}

pub fn create_data_even_in_range(seed: usize, iteration: usize, from: usize, to: usize, range_start: f32, range_end: f32) -> Vec<f32>
{
    let len_seed: &[_] = &[seed, iteration];
    let mut rng: StdRng = SeedableRng::from_seed(len_seed);
    let len = rng.gen_range(from, to);
    let len = len + len % 2;
    create_data_in_range_with_len(seed, iteration, len, range_start, range_end)
}

pub fn create_data_in_range_with_len(seed: usize, iteration: usize, len: usize, range_start: f32, range_end: f32) -> Vec<f32>
{
    let seed: &[_] = &[seed, iteration];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut data = vec![0.0; len];
    for i in 0..len {
        data[i] = rng.gen_range(range_start, range_end);
    }
    data
}

pub fn create_delta(seed: usize, iteration: usize)
    -> f32
{
    let seed: &[_] = &[seed, iteration];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    rng.gen_range(-10.0, 10.0)
}

use std::sync::{Arc, Mutex};

pub const RANGE_SINGLE_CORE: Range<usize> = Range { start: 10000, end: 1000000 };
pub const RANGE_MULTI_CORE: Range<usize> = Range { start: 1000001, end: 2000000 };

pub fn parameterized_vector_test<F>(test_code: F)
    where F: Fn(usize, Range<usize>) + Send + 'static + Sync
{
    let mut test_errors = Vec::new();
    
    // I don't know if there is a good reason for this, but Rust
    // requires us to lock the test_code function, 
    // does catch_panic spawn really a new thread?
    let test_code = Arc::new(Mutex::new(test_code));
    for iteration in 0 .. 10 {
        if test_errors.len() > 0 {
            break; // Stop on the first error to speed up things
        }
        
        let small_range = RANGE_SINGLE_CORE;
        let test_code = test_code.clone();
        let result = thread::catch_panic(move|| {
            let test_code = test_code.lock().unwrap();
            test_code(iteration, small_range);
        });
        
        match result {
            Ok(_) => (),
            Err(e) => {
                if let Some(e) = e.downcast_ref::<&'static str>() {
                    test_errors.push(format!("\nSingle threaded execution path failed on iteration {}\nFailure: {}", iteration, e));
                }
                else if let Some(e) = e.downcast_ref::<String>() {
                    test_errors.push(format!("\nSingle threaded execution path failed on iteration {}\nFailure: {}", iteration, e));
                }
                else {
                    test_errors.push(format!("\nSingle threaded execution path failed on iteration {}\nGot an unknown error: {:?}", iteration, e));
                }
            }
        }
    }
    
    for iteration in 0 .. 3 {
        if test_errors.len() > 0 {
            break; // Stop on the first error to speed up things
        }
        
        let large_range = RANGE_MULTI_CORE;
        let test_code = test_code.clone();
        let result = thread::catch_panic(move|| {
            let test_code = test_code.lock().unwrap();
            test_code(iteration, large_range);
        });
        
        match result {
            Ok(_) => (),
            Err(e) => {
                if let Some(e) = e.downcast_ref::<&'static str>() {
                    test_errors.push(format!("\nMulti threaded execution path failed on iteration {}\nFailure: {}", iteration, e));
                } 
                else if let Some(e) = e.downcast_ref::<String>() {
                    test_errors.push(format!("\nMulti threaded execution path failed on iteration {}\nFailure: {}", iteration, e));
                }
                else {
                    test_errors.push(format!("\nMulti threaded execution path failed on iteration {}\nGot an unknown error: {:?}", iteration, e));
                }
            }
        }
    }
    
    if test_errors.len() > 0 {
        let error_messages = test_errors.join("\n");
        panic!(error_messages);
    }
}