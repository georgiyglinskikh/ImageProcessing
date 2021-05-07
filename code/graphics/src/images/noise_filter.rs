use crate::images::buffer::Buffer;
use crate::images::filter::Filter;
use crate::images::utils::WhiteBlackType;

pub trait NoiseFilter: Filter {
    fn noise_filter(&mut self, r: u32);
}

impl NoiseFilter for Buffer {
    fn noise_filter(&mut self, r: u32) {
        Filter::filter(self, r, noise_filterer)
    }
}

fn noise_filterer(buffer: Buffer) -> WhiteBlackType {
    let sum: isize = buffer.buffer.iter().map(|x| *x as isize).sum();

    let number = buffer.size.width * buffer.size.height;

    (sum / number) as u8
}
