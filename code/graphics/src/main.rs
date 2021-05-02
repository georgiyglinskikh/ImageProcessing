mod images;

use crate::images::utils::white_black::WhiteBlackTypes;
use crate::images::white_black::WhiteBlack;

fn main() {
    let image =
        image::open("/home/geo/PycharmProjects/ImageProcessing/images/falls.bmp").expect("Cannot open image");

    let mut buf = image.to_rgba8();

    buf.white_black(WhiteBlackTypes::Smooth1);

    buf.save("./a.png").expect("Cannot save image to this file");
}
