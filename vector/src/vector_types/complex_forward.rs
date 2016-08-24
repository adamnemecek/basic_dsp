macro_rules! define_complex_operations_forward {
    (from: $name:ident, to: $gen_type:ident, complex: $complex_type:ident, real_partner: $real_partner:ident, $($data_type:ident),*)
     =>
     { 
        $(
            impl ComplexVectorOps<$data_type> for $name<$data_type>
            {
                type RealPartner = $real_partner<$data_type>;
                
                fn complex_data(&self) -> &[Complex<$data_type>] {
                    self.to_gen_borrow().complex_data()
                }
                
                fn complex_offset(self, offset: Complex<$data_type>) -> TransRes<Self>
                {
                    Self::from_genres(self.to_gen().complex_offset(offset))
                }
                    
                fn complex_scale(self, factor: Complex<$data_type>) -> TransRes<Self>
                {
                    Self::from_genres(self.to_gen().complex_scale(factor))
                }
                
                fn multiply_complex_exponential(self, a: $data_type, b: $data_type) -> TransRes<Self> {
                    Self::from_genres(self.to_gen().multiply_complex_exponential(a, b))
                }
                
                fn magnitude(self) -> TransRes<Self::RealPartner>
                {
                    Self::RealPartner::from_genres(self.to_gen().magnitude())
                }
                
                fn get_magnitude(&self, destination: &mut Self::RealPartner) -> VoidResult
                {
                    self.to_gen_borrow().get_magnitude(destination.to_gen_mut_borrow())
                }
                
                fn magnitude_squared(self) -> TransRes<Self::RealPartner>
                {
                    Self::RealPartner::from_genres(self.to_gen().magnitude_squared())
                }
                
                fn conj(self) -> TransRes<Self>
                {
                    Self::from_genres(self.to_gen().conj())
                }
                
                fn to_real(self) -> TransRes<Self::RealPartner>
                {
                    Self::RealPartner::from_genres(self.to_gen().to_real())
                }
        
                fn to_imag(self) -> TransRes<Self::RealPartner>
                {
                    Self::RealPartner::from_genres(self.to_gen().to_imag())
                }    
                        
                fn get_real(&self, destination: &mut Self::RealPartner) -> VoidResult
                {
                    self.to_gen_borrow().get_real(destination.to_gen_mut_borrow())
                }
                
                fn get_imag(&self, destination: &mut Self::RealPartner) -> VoidResult
                {
                    self.to_gen_borrow().get_imag(destination.to_gen_mut_borrow())
                }
                
                fn phase(self) -> TransRes<Self::RealPartner>
                {
                    Self::RealPartner::from_genres(self.to_gen().phase())
                }
                
                fn get_phase(&self, destination: &mut Self::RealPartner) -> VoidResult
                {
                    self.to_gen_borrow().get_phase(destination.to_gen_mut_borrow())
                }
                
                fn map_inplace_complex<A, F>(self, argument: A, f: F) -> TransRes<Self>
                    where A: Sync + Copy + Send,
                          F: Fn(Complex<$data_type>, usize, A) -> Complex<$data_type> + 'static + Sync {
                    Self::from_genres(self.to_gen().map_inplace_complex(argument, f))
                }
                
                fn map_aggregate_complex<A, FMap, FAggr, R>(
                    &self, 
                    argument: A, 
                    map: FMap,
                    aggregate: FAggr) -> ScalarResult<R>
                        where A: Sync + Copy + Send,
                              FMap: Fn(Complex<$data_type>, usize, A) -> R + 'static + Sync,
                              FAggr: Fn(R, R) -> R + 'static + Sync + Send,
                              R: Send {
                    self.to_gen_borrow().map_aggregate_complex(argument, map, aggregate) 
                }
                
                fn complex_dot_product(&self, factor: &Self) -> ScalarResult<Complex<$data_type>>
                {
                    self.to_gen_borrow().complex_dot_product(&factor.to_gen_borrow())
                }
                
                fn complex_statistics(&self) -> Statistics<Complex<$data_type>> {
                    self.to_gen_borrow().complex_statistics()
                }
                
                fn complex_statistics_splitted(&self, len: usize) -> Vec<Statistics<Complex<$data_type>>> {
                    self.to_gen_borrow().complex_statistics_splitted(len)
                }
                
                fn complex_sum(&self) -> Complex<$data_type> {
                    self.to_gen_borrow().complex_sum()
                }
                
                fn complex_sum_sq(&self) -> Complex<$data_type> {
                    self.to_gen_borrow().complex_sum_sq()
                }
                
                fn get_real_imag(&self, real: &mut Self::RealPartner, imag: &mut Self::RealPartner) -> VoidResult {
                    self.to_gen_borrow().get_real_imag(real.to_gen_mut_borrow(), imag.to_gen_mut_borrow())
                }
                
                fn get_mag_phase(&self, mag: &mut Self::RealPartner, phase: &mut Self::RealPartner) -> VoidResult {
                    self.to_gen_borrow().get_mag_phase(mag.to_gen_mut_borrow(), phase.to_gen_mut_borrow())
                }
                
                fn set_real_imag(self, real: &Self::RealPartner, imag: &Self::RealPartner) -> TransRes<Self> {
                    Self::from_genres(self.to_gen().set_real_imag(real.to_gen_borrow(), imag.to_gen_borrow()))
                }
                
                fn set_mag_phase(self, mag: &Self::RealPartner, phase: &Self::RealPartner) -> TransRes<Self> {
                    Self::from_genres(self.to_gen().set_mag_phase(mag.to_gen_borrow(), phase.to_gen_borrow()))
                }
            }
            
            impl $name<$data_type>
            {
                fn to_gen(self) -> $gen_type<$data_type>
                {
                    unsafe { mem::transmute(self) }
                }
                
                fn to_gen_borrow(&self) -> &$gen_type<$data_type>
                {
                    unsafe { mem::transmute(self) }
                }
                
                fn from_gen(other: $gen_type<$data_type>) -> Self
                {
                    unsafe { mem::transmute(other) }
                }
                
                fn from_genres(other: TransRes<$gen_type<$data_type>>) -> TransRes<Self>
                {
                    match other {
                        Ok(v) => Ok($name::<$data_type>::from_gen(v)),
                        Err((r, v)) => Err((r, $name::<$data_type>::from_gen(v)))
                    }
                }
            }
            
            impl ScaleOps<Complex<$data_type>> for $name<$data_type> {
                fn scale(self, offset: Complex<$data_type>) -> TransRes<Self> {
                    self.complex_scale(offset)
                }
            }
            
            impl OffsetOps<Complex<$data_type>> for $name<$data_type> {
                fn offset(self, offset: Complex<$data_type>) -> TransRes<Self> {
                    self.complex_offset(offset)
                }
            }
            
            impl DotProductOps<Complex<$data_type>> for $name<$data_type> {
                fn dot_product(&self, factor: &Self) -> ScalarResult<Complex<$data_type>> {
                    self.complex_dot_product(factor)
                }
            }
            
            impl StatisticsOps<Complex<$data_type>> for $name<$data_type> {
                fn statistics(&self) -> Statistics<Complex<$data_type>> {
                    self.complex_statistics()
                }
                
                fn statistics_splitted(&self, len: usize) -> Vec<Statistics<Complex<$data_type>>> {
                    self.complex_statistics_splitted(len)
                }
                
                fn sum(&self) -> Complex<$data_type> {
                    self.complex_sum()
                }
                
                fn sum_sq(&self) -> Complex<$data_type> {
                    self.complex_sum_sq()
                }
            }
            
            impl VectorIter<Complex<$data_type>> for $name<$data_type> {
                fn map_inplace<A, F>(self, argument: A, map: F) -> TransRes<Self>
                    where A: Sync + Copy + Send,
                          F: Fn(Complex<$data_type>, usize, A) -> Complex<$data_type> + 'static + Sync {
                    self.map_inplace_complex(argument, map)
                }
                
                fn map_aggregate<A, FMap, FAggr, R>(
                    &self, 
                    argument: A, 
                    map: FMap,
                    aggregate: FAggr) -> ScalarResult<R>
                where A: Sync + Copy + Send,
                      FMap: Fn(Complex<$data_type>, usize, A) -> R + 'static + Sync,
                      FAggr: Fn(R, R) -> R + 'static + Sync + Send,
                      R: Send {
                    self.map_aggregate_complex(argument, map, aggregate)  
                }
            }
        )*
     }
}