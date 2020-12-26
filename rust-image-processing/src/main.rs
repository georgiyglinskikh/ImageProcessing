use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let image_path = Path::new(&args[1]);

    let mut image =
        image::open(image_path.to_str().unwrap())
        .expect("cannot find image");

    image.save(
        format!(
                "{}/1_{}",
                image_path.parent().unwrap().to_str().unwrap(),
                image_path.file_name().unwrap().to_str().unwrap()
            )
        ).expect("cannot save picture");
}
