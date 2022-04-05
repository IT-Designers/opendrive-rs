pub trait NotNan {
    fn not_nan_f32(&mut self) -> arbitrary::Result<f32>;
    fn not_nan_f64(&mut self) -> arbitrary::Result<f64>;
}

impl<'a> NotNan for arbitrary::Unstructured<'a> {
    fn not_nan_f32(&mut self) -> arbitrary::Result<f32> {
        Ok(f32::from_bits(
            self.arbitrary::<f32>()?.to_bits() & !f32::NAN.to_bits(),
        ))
    }

    fn not_nan_f64(&mut self) -> arbitrary::Result<f64> {
        Ok(f64::from_bits(
            self.arbitrary::<f64>()?.to_bits() & !f64::NAN.to_bits(),
        ))
    }
}
