use crate::app::App;

mod app;
mod images;

fn main() {
    let mut app = App::new();

    app.parse_args();

    app.open_image();

    app.process_image();

    app.save_image();
}
