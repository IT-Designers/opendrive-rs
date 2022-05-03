use std::ops::RangeInclusive;

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

pub trait ArbitraryStrings {
    fn arbitrary_string(
        &mut self,
        size: RangeInclusive<usize>,
        characters: &[RangeInclusive<char>],
    ) -> arbitrary::Result<String>;
}

impl<'a> ArbitraryStrings for arbitrary::Unstructured<'a> {
    fn arbitrary_string(
        &mut self,
        size: RangeInclusive<usize>,
        characters: &[RangeInclusive<char>],
    ) -> arbitrary::Result<String> {
        let len = self.arbitrary_len::<String>()?;
        let len = len.min(*size.end()).max(*size.start());
        let mut string = String::with_capacity(len);

        while string.len() < len && !characters.is_empty() {
            let slice = self.int_in_range(0..=characters.len().saturating_sub(1))?;
            let slice = &characters[slice];
            let character = self.int_in_range(*slice.start() as u32..=*slice.end() as u32)?;
            if let Ok(character) = char::try_from(character) {
                string.push(character);
            }
        }

        Ok(string)
    }
}
