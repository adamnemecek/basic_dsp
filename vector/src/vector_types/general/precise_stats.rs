use RealNumber;
use num::{Complex, Zero};
use num::complex::{Complex64};
use multicore_support::*;
use super::super::{array_to_complex, Vector, DspVec, ToSlice, Domain, RealNumberSpace,
                   ComplexNumberSpace, Statistics, Stats};
use super::{kahan_sum, kahan_sumb};               

pub trait PreciseStatisticsOps<T>: Sized
    where T: Sized
{
    /// Calculates the statistics of the data contained in the vector using
    /// a more precise but slower algorithm.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// # use num::complex::Complex64;
    /// use basic_dsp_vector::*;
    /// # fn main() {
    /// let vector: Vec<f32> = vec!(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    /// let vector = vector.to_complex_time_vec();
    /// let result = vector.statistics_prec();
    /// assert_eq!(result.sum, Complex64::new(9.0, 12.0));
    /// assert_eq!(result.count, 3);
    /// assert_eq!(result.average, Complex64::new(3.0, 4.0));
    /// assert!((result.rms - Complex64::new(3.4027193, 4.3102784)).norm() < 1e-4);
    /// assert_eq!(result.min, Complex64::new(1.0, 2.0));
    /// assert_eq!(result.min_index, 0);
    /// assert_eq!(result.max, Complex64::new(5.0, 6.0));
    /// assert_eq!(result.max_index, 2);
    /// }
    /// ```
    fn statistics_prec(&self) -> T;

    /// Calculates the statistics of the data contained in the vector as if the vector would
    /// have been split into `len` pieces 
    /// using a more precise but slower algorithm. `self.len` should be dividable by
    /// `len` without a remainder,
    /// but this isn't enforced by the implementation.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// # use num::complex::Complex64;
    /// use basic_dsp_vector::*;
    /// # fn main() {
    /// let vector: Vec<f32> = vec!(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    /// let vector = vector.to_complex_time_vec();
    /// let result = vector.statistics_splitted_prec(2);
    /// assert_eq!(result[0].sum, Complex64::new(6.0, 8.0));
    /// assert_eq!(result[1].sum, Complex64::new(3.0, 4.0));
    /// }
    /// ```
    fn statistics_splitted_prec(&self, len: usize) -> Vec<T>;
}

pub trait PreciseSumOps<T>: Sized
    where T: Sized
{
    /// Calculates the sum of the data contained in the vector 
    /// using a more precise but slower algorithm.
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// # use num::complex::Complex64;
    /// use basic_dsp_vector::*;
    /// # fn main() {
    /// let vector = vec!(1.0, 2.0, 3.0, 4.0, 5.0, 6.0).to_complex_time_vec();
    /// let result = vector.sum_prec();
    /// assert_eq!(result, Complex64::new(9.0, 12.0));
    /// }
    /// ```
    fn sum_prec(&self) -> T;

    /// Calculates the sum of the squared data contained in the vector 
    /// using a more precise but slower algorithm.
    /// # Example
    ///
    /// ```
    /// # extern crate num;
    /// # extern crate basic_dsp_vector;
    /// # use num::complex::Complex64;
    /// use basic_dsp_vector::*;
    /// # fn main() {
    /// let vector = vec!(1.0, 2.0, 3.0, 4.0, 5.0, 6.0).to_complex_time_vec();
    /// let result = vector.sum_sq_prec();
    /// assert_eq!(result, Complex64::new(-21.0, 88.0));
    /// }
    /// ```
    fn sum_sq_prec(&self) -> T;
}

pub trait PreciseStats<T>: Sized {
    fn add_prec(&mut self, elem: T, index: usize, sumc: &mut T, rmsc: &mut T);
}

impl<S, N, D> PreciseStatisticsOps<Statistics<f64>> for DspVec<S, f32, N, D>
    where S: ToSlice<f32>,
          N: RealNumberSpace,
          D: Domain
{
    fn statistics_prec(&self) -> Statistics<f64> {
        let data_length = self.len();
        let array = self.data.to_slice();
        let chunks = Chunk::get_chunked_results(Complexity::Small,
                                                &self.multicore_settings,
                                                &array[0..data_length],
                                                1,
                                                (),
                                                |array, range, _arg| {
            let mut stats = Statistics::empty();
            let mut j = range.start;
            for num in array {
                stats.add(*num as f64, j);
                j += 1;
            }
            stats
        });

        Statistics::merge(&chunks)
    }

    fn statistics_splitted_prec(&self, len: usize) -> Vec<Statistics<f64>> {
        if len == 0 {
            return Vec::new();
        }

        let data_length = self.len();
        let array = self.data.to_slice();
        let chunks = Chunk::get_chunked_results(Complexity::Small,
                                                &self.multicore_settings,
                                                &array[0..data_length],
                                                1,
                                                len,
                                                |array, range, len| {
            let mut results = Statistics::empty_vec(len);
            let mut j = range.start;
            for num in array {
                let stats = &mut results[j % len];
                stats.add(*num as f64, j / len);
                j += 1;
            }

            results
        });

        Statistics::merge_cols(&chunks)
    }
}

impl<S, N, D> PreciseStatisticsOps<Statistics<f64>> for DspVec<S, f64, N, D>
    where S: ToSlice<f64>,
          N: RealNumberSpace,
          D: Domain
{
    fn statistics_prec(&self) -> Statistics<f64> {
        let data_length = self.len();
        let array = self.data.to_slice();
        let chunks = Chunk::get_chunked_results(Complexity::Small,
                                                &self.multicore_settings,
                                                &array[0..data_length],
                                                1,
                                                (),
                                                |array, range, _arg| {
            let mut stats = Statistics::empty();
            let mut j = range.start;
            let mut sumc = 0.0;
            let mut rmsc = 0.0;
            for num in array {
                stats.add_prec(*num, j, &mut sumc, &mut rmsc);
                j += 1;
            }
            stats
        });

        Statistics::merge(&chunks)
    }

    fn statistics_splitted_prec(&self, len: usize) -> Vec<Statistics<f64>> {
        if len == 0 {
            return Vec::new();
        }

        let data_length = self.len();
        let array = self.data.to_slice();
        let chunks = Chunk::get_chunked_results(Complexity::Small,
                                                &self.multicore_settings,
                                                &array[0..data_length],
                                                1,
                                                len,
                                                |array, range, len| {
            let mut results = Statistics::empty_vec(len);
            let mut j = range.start;
            let mut sumc = 0.0;
            let mut rmsc = 0.0;
            for num in array {
                let stats = &mut results[j % len];
                stats.add_prec(*num, j / len, &mut sumc, &mut rmsc);
                j += 1;
            }

            results
        });

        Statistics::merge_cols(&chunks)
    }
}

impl<S, N, D> PreciseSumOps<f64> for DspVec<S, f32, N, D>
    where S: ToSlice<f32>,
          N: RealNumberSpace,
          D: Domain
{
    fn sum_prec(&self) -> f64 {
        let data_length = self.len();
        let array = self.data.to_slice();
        let chunks = Chunk::get_chunked_results(Complexity::Small,
                                                &self.multicore_settings,
                                                &array[0..data_length],
                                                1,
                                                (),
                                                move |array, _, _| {
            let mut sum = 0.0;
            for n in array {
                sum = sum + *n as f64;
            }
            sum
        });
        chunks.iter()
            .fold(0.0, |a, b| a + b)
    }

    fn sum_sq_prec(&self) -> f64 {
        let data_length = self.len();
        let array = self.data.to_slice();
        let chunks = Chunk::get_chunked_results(Complexity::Small,
                                                &self.multicore_settings,
                                                &array[0..data_length],
                                                1,
                                                (),
                                                move |array, _, _| {
            let mut sum = 0.0;
            for n in array {
                let t = *n as f64;
                sum = sum + t * t;
            }
            sum
        });
        chunks.iter()
            .fold(0.0, |a, b| a + b)
    }
}

impl<S, N, D> PreciseSumOps<f64> for DspVec<S, f64, N, D>
    where S: ToSlice<f64>,
          N: RealNumberSpace,
          D: Domain
{
    fn sum_prec(&self) -> f64 {
        let data_length = self.len();
        let array = self.data.to_slice();
        let chunks = Chunk::get_chunked_results(Complexity::Small,
                                                &self.multicore_settings,
                                                &array[0..data_length],
                                                1,
                                                (),
                                                move |array, _, _| {
            kahan_sumb(array.iter())
        });
        chunks.iter()
            .fold(0.0, |a, b| a + b)
    }

    fn sum_sq_prec(&self) -> f64 {
        let data_length = self.len();
        let array = self.data.to_slice();
        let chunks = Chunk::get_chunked_results(Complexity::Small,
                                                &self.multicore_settings,
                                                &array[0..data_length],
                                                1,
                                                (),
                                                move |array, _, _| {
            kahan_sum(array.iter().map(|x|x*x))
        });
        chunks.iter()
            .fold(0.0, |a, b| a + b)
    }
}

impl<S, N, D> PreciseStatisticsOps<Statistics<Complex<f64>>> for DspVec<S, f32, N, D>
    where S: ToSlice<f32>,
          N: ComplexNumberSpace,
          D: Domain
{
    fn statistics_prec(&self) -> Statistics<Complex<f64>> {
        let data_length = self.len();
        let array = self.data.to_slice();
        let chunks = Chunk::get_chunked_results(Complexity::Small,
                                                &self.multicore_settings,
                                                &array[0..data_length],
                                                2,
                                                (),
                                                |array, range, _arg| {
            let mut stat = Statistics::<Complex64>::empty();
            let mut j = range.start / 2;
            let array = array_to_complex(array);
            for num in array {
                stat.add(Complex64::new(num.re as f64, num.im as f64), j);
                j += 1;
            }
            stat
        });

        Statistics::merge(&chunks)
    }

    fn statistics_splitted_prec(&self, len: usize) -> Vec<Statistics<Complex<f64>>> {
        if len == 0 {
            return Vec::new();
        }

        let data_length = self.len();
        let array = self.data.to_slice();
        let chunks = Chunk::get_chunked_results(Complexity::Small,
                                                &self.multicore_settings,
                                                &array[0..data_length],
                                                2,
                                                len,
                                                |array, range, len| {
            let mut results = Statistics::<Complex<f64>>::empty_vec(len);
            let mut j = range.start / 2;
            let array = array_to_complex(array);
            for num in array {
                let stat = &mut results[j % len];
                stat.add(Complex64::new(num.re as f64, num.im as f64), j / len);
                j += 1;
            }

            results
        });

        Statistics::merge_cols(&chunks)
    }
}

impl<S, N, D> PreciseStatisticsOps<Statistics<Complex<f64>>> for DspVec<S, f64, N, D>
    where S: ToSlice<f64>,
          N: ComplexNumberSpace,
          D: Domain
{
    fn statistics_prec(&self) -> Statistics<Complex<f64>> {
        let data_length = self.len();
        let array = self.data.to_slice();
        let chunks = Chunk::get_chunked_results(Complexity::Small,
                                                &self.multicore_settings,
                                                &array[0..data_length],
                                                2,
                                                (),
                                                |array, range, _arg| {
            let mut stat = Statistics::<Complex64>::empty();
            let mut j = range.start / 2;
            let array = array_to_complex(array);
            let mut sumc = Complex64::zero();
            let mut rmsc = Complex64::zero();
            for num in array {
                stat.add_prec(*num, j, &mut sumc, &mut rmsc);
                j += 1;
            }
            stat
        });

        Statistics::merge(&chunks)
    }

    fn statistics_splitted_prec(&self, len: usize) -> Vec<Statistics<Complex<f64>>> {
        if len == 0 {
            return Vec::new();
        }

        let data_length = self.len();
        let array = self.data.to_slice();
        let chunks = Chunk::get_chunked_results(Complexity::Small,
                                                &self.multicore_settings,
                                                &array[0..data_length],
                                                2,
                                                len,
                                                |array, range, len| {
            let mut results = Statistics::<Complex<f64>>::empty_vec(len);
            let mut j = range.start / 2;
            let array = array_to_complex(array);
            let mut sumc = Complex64::zero();
            let mut rmsc = Complex64::zero();
            for num in array {
                let stat = &mut results[j % len];
                stat.add_prec(*num, j / len, &mut sumc, &mut rmsc);
                j += 1;
            }

            results
        });

        Statistics::merge_cols(&chunks)
    }
}

impl<S, N, D> PreciseSumOps<Complex<f64>> for DspVec<S, f32, N, D>
    where S: ToSlice<f32>,
          N: ComplexNumberSpace,
          D: Domain
{
    fn sum_prec(&self) -> Complex<f64> {
        let data_length = self.len();
        let array = self.data.to_slice();
        let chunks = Chunk::get_chunked_results(Complexity::Small,
                                                &self.multicore_settings,
                                                &array[0..data_length],
                                                2,
                                                (),
                                                move |array, _, _| {
            let mut sum = Complex64::zero();
            let array = array_to_complex(&array[0..array.len()]);
            for n in array {
                sum = sum + Complex64::new(n.re as f64, n.im as f64);
            }
            sum
        });
        chunks.iter()
            .fold(Complex64::zero(), |a, b| a + b)
    }

    fn sum_sq_prec(&self) -> Complex<f64> {
        let data_length = self.len();
        let array = self.data.to_slice();
        let chunks = Chunk::get_chunked_results(Complexity::Small,
                                                &self.multicore_settings,
                                                &array[0..data_length],
                                                2,
                                                (),
                                                move |array, _, _| {
            let mut sum = Complex::<f64>::zero();
            let array = array_to_complex(&array[0..array.len()]);
            for n in array {
                let t = Complex64::new(n.re as f64, n.im as f64);
                sum = sum + t * t;
            }
            sum
        });
        chunks.iter()
            .fold(Complex64::zero(), |a, b| a + b)
    }
}

impl<T> PreciseStats<T> for Statistics<T>
    where T: RealNumber
{
    #[inline]
    fn add_prec(&mut self, elem: T, index: usize, sumc: &mut T, rmsc: &mut T) {
        let y = elem - *sumc;
        let t = self.sum + y;
        *sumc = (t - self.sum) - y;
        self.sum = t;
    
        self.count += 1;
        
        let y = elem * elem - *rmsc;
        let t = self.rms + y;
        *rmsc = (t - self.rms) - y;
        self.rms = t;
        
        if elem > self.max {
            self.max = elem;
            self.max_index = index;
        }
        if elem < self.min {
            self.min = elem;
            self.min_index = index;
        }
    }
}

impl<T> PreciseStats<Complex<T>> for Statistics<Complex<T>>
    where T: RealNumber
{
    #[inline]
    fn add_prec(&mut self, elem: Complex<T>, index: usize, sumc: &mut Complex<T>, rmsc: &mut Complex<T>) {
        let y = elem - *sumc;
        let t = self.sum + y;
        *sumc = (t - self.sum) - y;
        self.sum = t;
    
        self.count += 1;
        
        let y = elem * elem - *rmsc;
        let t = self.rms + y;
        *rmsc = (t - self.rms) - y;
        self.rms = t;
        if elem.norm() > self.max.norm() {
            self.max = elem;
            self.max_index = index;
        }
        if elem.norm() < self.min.norm() {
            self.min = elem;
            self.min_index = index;
        }
    }
}