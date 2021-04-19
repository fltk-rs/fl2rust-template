use fltk::{prelude::*, *};
mod mainview;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut ui = mainview::UserInterface::make_window();
    let mut frame = ui.frame.clone();
    ui.but.set_callback(move |_| {
        frame.set_label("Hello World!");
    });
    app.run().unwrap();
}
