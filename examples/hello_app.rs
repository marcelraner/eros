use eros::{view::Text, App};

fn main() {
    let main_view = Text::new("Hello").boxed();
    let mut app = App::new().with_main_view(main_view);
    app.run().unwrap();
}
