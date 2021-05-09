use super::{buffer::Buffer, filter::Filter, types::Dimension, utils::WhiteBlackType, zero_one::ZeroOne};

pub trait Sobel: Filter + ZeroOne {
    fn sobel(&mut self, r: u32);
}

type SobelFilterType = i8;

const SOBEL_FILTER_SIZE: usize = 9;

const SOBEL_FILTER: [SobelFilterType; SOBEL_FILTER_SIZE] = [1, 2, 1, 0, 0, 0, -1, -2, -1];

const SOBEL_FILTER_T: [SobelFilterType; SOBEL_FILTER_SIZE] = [1, 0, -1, 2, 0, -2, 1, 0, -1];

fn mul_array(
    buffer: &Buffer,
    sobel_filter: [SobelFilterType; SOBEL_FILTER_SIZE],
) -> i32 {
    buffer
        .buffer
        .iter()
        .take(SOBEL_FILTER_SIZE)
        .zip(sobel_filter.iter())
        .map(|(&x, &y)| x as i32 * y as i32)
        .sum()
}

fn sobel_filter(buffer: Buffer) -> WhiteBlackType {
    let gx = mul_array(&buffer, SOBEL_FILTER);
    let gy = mul_array(&buffer, SOBEL_FILTER_T);

    let hypos = ((gx.pow(2) + gy.pow(2)) as f64).sqrt();

    hypos as u8
}

impl Sobel for Buffer {
    fn sobel(&mut self, r: u32) {
        let sum: u128 =self.buffer.iter()
        .map(|&x| x as u128)
        .sum();

        let len = self.buffer.len() as u128;

        let median =
            (sum / len) as u8;

        let max = *self.buffer.iter().max().unwrap();

        self.into_zo(median);

        self.filter(r, sobel_filter);

        self.from_zo(max);
    }
}
