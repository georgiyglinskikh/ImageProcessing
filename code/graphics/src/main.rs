fn main() {
    let image =
        image::open("/home/geo/ImageProcessing/images/floor.bmp").expect("Cannot open image");

    let buf = image.as_rgba8().expect("Cannot open image in this format");

    for i in buf.enumerate_pixels() {
        println!("{:?}", i);
    }

    // White and black

}
