use args::ImageArgs;
use image::open;
use ndarray::{ArrayView, Axis, ShapeBuilder};

mod args;

fn main() {
    // TODO: ndarray working

    let ImageArgs {
        width,
        height,
        path,
    } = ImageArgs::parse_args();

    let image = open(path).expect("cannot open image");

    let color_bytes = ArrayView::from_shape(
        ShapeBuilder::strides((width, height), (1, 1)),
        image.as_bytes(),
    )
    .unwrap();

    println!("{}", color_bytes.len_of(Axis(0)))
}
