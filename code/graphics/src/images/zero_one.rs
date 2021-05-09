use super::{buffer::Buffer, utils::WhiteBlackType};

pub trait ZeroOne {
    fn transform_zo(&mut self, median: WhiteBlackType);
    fn from_zo(&mut self, max: WhiteBlackType);
}

impl ZeroOne for Buffer {
    fn transform_zo(&mut self, median: WhiteBlackType) {
        self.buffer = self
            .buffer
            .iter()
            .map(|&x| if x < median { 0_u8 } else { 1_u8 })
            .collect()
    }

    fn from_zo(&mut self, max: WhiteBlackType) {
        let cur_max = *self.buffer.iter().max().unwrap();

        let coefficient = max as f64 / cur_max as f64;

        self.buffer = self
            .buffer
            .iter()
            .map(|&x| (x as f64 * coefficient) as WhiteBlackType)
            .collect();
    }
}
