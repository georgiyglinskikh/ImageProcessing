pub struct ImageArgs {
    pub width: usize,
    pub height: usize
}

impl ImageArgs {
    pub fn new(width: usize, height: usize) -> ImageArgs {
        ImageArgs {
            width,
            height
        }
    }

    pub fn parse_args() -> Option<Vec<ImageArgs>> {
        None
    }
}
