use num_cpus;
use std::slice::ChunksMut;
use num::traits::Float;
use simple_parallel::Pool;
use std::ops::Range;

/// Indicates how complex an operation is and determines how many cores 
/// will be used since operations with smaller complexity are memory bus bound
/// and not CPU bound
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Complexity {
	Small,
    Medium,
    Large
}

/// Contains logic which helps to perform an operation
/// in parallel by dividing an array into chunks.
pub struct Chunk;
#[allow(dead_code)]
impl Chunk
{
    /// Gives access to the thread pool singleton
	fn get_static_pool() -> &'static mut Pool
	{
		use std::mem::transmute;
		use std::sync::{Once, ONCE_INIT};
		unsafe
		{
			static mut pool: *mut Pool = 0 as *mut Pool;
			static mut ONCE: Once = ONCE_INIT;
			ONCE.call_once(||
			{
				pool = transmute::<Box<Pool>, *mut Pool>(Box::new(Pool::new(num_cpus::get())));
			});
			
			let mut static_pool = &mut *pool;
			static_pool
		}
	}

    /// Figures out how many threads make use for the an operation with the given complexity on 
    /// an array with the given size. 
    ///
    /// This method tries to balance the expected performance gain vs. CPU utilization since there is in most cases
    /// no point to keep all CPU cores busy only to get 5 to 10% performance gain.
    /// The expected performance gain is roughly estimated based on three factors:
    /// 1. More cores improves the calculation speed according to Amdahl's law (`https://en.wikipedia.org/wiki/Amdahl's_law`)
    /// 2. Spawning a thread consumes time and so the array length must be large enough to so that the expected performance
    ///    gain justifies the effort to spawn a thread/invoke the thread pool. 
    /// 3. The CPU is not the only resource and if one of the other resources is a bottleneck then Amdahl's law won't be applicable.
    ///    The memory bus speed is limited (20GB/s in case of a typical 2015 consumer laptop in the price range of $1000) and
    ///    for operations which only require a few CPU cycles already one or two cores will process data faster than the 
    ///    memory bus is able to provide and to transport back. Using more cores then only creates heat but no performance benefit.
	#[inline]
	fn determine_number_of_chunks(array_length: usize, complexity: Complexity) -> usize
	{
        let cores = num_cpus::get();
        if complexity == Complexity::Large || cores == 1 {
            cores
        }
        else if complexity == Complexity::Small  {
            if array_length < 500000 {
                1
            }
            else {
                2
            }
        }
        else { // complexity == medium
            if array_length < 10000 {
                1
            }
            else if array_length < 50000 {
                2
            }
            else {
                cores
            }
        }
	}
	
    /// Partitions an array into the given number of chunks. It makes sure that all chunks have the same size
    /// and so it will happen that some elements at the end of the array are not part of any chunk. 
	#[inline]
	fn partition<T>(array: &mut [T], array_length: usize, step_size: usize, number_of_chunks: usize) -> ChunksMut<T>
		where T : Float + Copy + Clone + Send
	{
		let chunk_size = Chunk::calc_chunk_size(array_length, step_size, number_of_chunks);
		array[0 .. array_length].chunks_mut(chunk_size)
	}
	
	#[inline]
	fn calc_chunk_size(array_length: usize, step_size: usize, number_of_chunks: usize) -> usize
	{
		let mut chunk_size = (array_length as f64/ number_of_chunks as f64).ceil() as usize;
		let remainder = chunk_size % step_size;
		if remainder > 0
		{
			chunk_size += step_size - chunk_size % step_size;
		}
		
		chunk_size
	}
	
    /// This function returns the ranges which correspond to the chunks generated by `partition_in_number`. 
	#[inline]
	fn partition_in_ranges(array_length: usize, step_size: usize, number_of_chunks: usize) -> Vec<Range<usize>>
	{
		let chunk_size = Chunk::calc_chunk_size(array_length, step_size, number_of_chunks);
		let mut ranges = Vec::with_capacity(number_of_chunks);
		let mut sum = 0;
		for i in 0..number_of_chunks {
			let new_sum = if i < number_of_chunks - 1 { sum + chunk_size } else { array_length };
			ranges.push(Range { start: sum, end: new_sum - 1 });
			sum = new_sum;
		} 
		
		ranges
	}
		
    /// Executes the given function on the all elements of the array in parallel.
	#[inline]
	pub fn execute<F, T>(complexity: Complexity, array: &mut [T], step_size: usize, function: F)
		where F: Fn(&mut [T]) + 'static + Sync,
			  T : Float + Copy + Clone + Send + Sync
	{
		let array_length = array.len();
		Chunk::execute_partial(complexity, array, array_length, step_size, function);
	}
	
    /// Executes the given function on the first `array_length` elements of the given array in parallel.
	#[inline]
	pub fn execute_partial<F, T>(complexity: Complexity, array: &mut [T], array_length: usize, step_size: usize, function: F)
		where F: Fn(&mut [T]) + 'static + Sync,
			  T : Float + Copy + Clone + Send + Sync
	{
        let number_of_chunks = Chunk::determine_number_of_chunks(array_length, complexity);
		if number_of_chunks > 1
		{
			let chunks = Chunk::partition(array, array_length, step_size, number_of_chunks);
			let ref mut pool = Chunk::get_static_pool();
			pool.for_(chunks, |chunk|
				{
					function(chunk);
				});
		}
		else
		{
			function(&mut array[0..array_length]);
		}
	}
	
    /// Executes the given function on the first `array_length` elements of the given array in parallel and passes
    /// the argument to all function calls.
	#[inline]
	pub fn execute_partial_with_arguments<T,S,F>(
            complexity: Complexity, 
            array: &mut [T], array_length: usize, step_size: usize, 
            arguments:S, function: F)
		where F: Fn(&mut [T], S) + 'static + Sync, 
			  T: Float + Copy + Clone + Send + Sync,
			  S: Sync + Copy
	{
		let number_of_chunks = Chunk::determine_number_of_chunks(array_length, complexity);
		if number_of_chunks > 1
		{
			let chunks = Chunk::partition(array, array_length, step_size, number_of_chunks);
			let ref mut pool = Chunk::get_static_pool();
			pool.for_(chunks, |chunk|
				{
					function(chunk, arguments);
				});
		}
		else
		{
			function(&mut array[0..array_length], arguments);
		}
	}
	
    /// Executes the given function on the all elements of the array in parallel. Results are intended to be stored in the target array.
	#[inline]
	pub fn execute_original_to_target<F, T>(
            complexity: Complexity, 
            original: &[T], original_length: usize, original_step: usize, 
            target: &mut [T], target_length: usize, target_step: usize, 
            function: F)
		where F: Fn(&[T], Range<usize>, &mut [T]) + 'static + Sync,
			  T : Float + Copy + Clone + Send + Sync
	{
		let number_of_chunks = Chunk::determine_number_of_chunks(original_length, complexity);
		if number_of_chunks > 1
		{
			let chunks = Chunk::partition(target, target_length, target_step, number_of_chunks);
			let ranges = Chunk::partition_in_ranges(original_length, original_step, chunks.len());
			let ref mut pool = Chunk::get_static_pool();
			pool.for_(chunks.zip(ranges), |chunk|
				{
					function(original, chunk.1, chunk.0);
				});
		}
		else
		{
			function(original, Range { start: 0, end: original_length }, &mut target[0..target_length]);
		}
	}
	
    /// Executes the given function on the all elements of the array in parallel and passes
    /// the argument to all function calls.. Results are intended to be stored in the target array.
	#[inline]
	pub fn execute_original_to_target_with_arguments<T,S,F>(
            complexity: Complexity, 
            original: &[T], original_length: usize, original_step: usize, 
            target: &mut [T], target_length: usize, target_step: usize, 
            arguments: S, function: F)
		where F: Fn(&[T], Range<usize>, &mut [T], S) + 'static + Sync,
			  T : Float + Copy + Clone + Send + Sync,
			  S: Sync + Copy
	{
		let number_of_chunks = Chunk::determine_number_of_chunks(original_length, complexity);
		if number_of_chunks > 1
		{
			let chunks = Chunk::partition(target, target_length, target_step, number_of_chunks);
			let ranges = Chunk::partition_in_ranges(original_length, original_step, chunks.len());
			let ref mut pool = Chunk::get_static_pool();
			pool.for_(chunks.zip(ranges), |chunk|
				{
					function(original, chunk.1, chunk.0, arguments);
				});
		}
		else
		{
			function(original, Range { start: 0, end: original_length }, &mut target[0..target_length], arguments);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::ops::Range;
	
	#[test]
	fn partition_array()
	{
		let mut array = [0.0; 256];
		let chunks = Chunk::partition(&mut array, 256, 4, 2);
		assert_eq!(chunks.len(), 2);
		for chunk in chunks
		{
			assert_eq!(chunk.len(), 128);
		}
	}
	
	#[test]
	fn partition_array_8_cores()
	{
		let mut array = [0.0; 1023];
		let chunks = Chunk::partition(&mut array, 1023, 4, 8);
		assert_eq!(chunks.len(), 8);
		let mut i = 0;
		for chunk in chunks
		{
			let expected = if i >= 7 { 127 } else { 128 };
			assert_eq!(chunk.len(), expected);
			i += 1;
		}
	}
	
	#[test]
	fn partitionin_ranges()
	{
		let ranges = Chunk::partition_in_ranges(1023, 4, 2);
		assert_eq!(ranges.len(), 2);
		assert_eq!(ranges[0], Range { start: 0, end: 511 });
		assert_eq!(ranges[1], Range { start: 512, end: 1022 });
	}
}