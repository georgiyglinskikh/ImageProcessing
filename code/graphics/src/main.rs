mod images;
mod app;

use crate::app::App;

fn main() {
    let mut app = App::new();

    app.parse_args();

    app.open_image();

    app.process_image();

    app.save_image();
}
