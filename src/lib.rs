pub mod view;

use view::View;

pub struct App {
    main_view: Option<Box<dyn View>>,
}

impl App {
    pub fn new() -> Self {
        Self { main_view: None }
    }

    pub fn with_main_view(mut self, view: Box<dyn View>) -> Self {
        self.main_view = Some(view);
        self
    }

    pub fn run(&self) -> Result<(), String> {
        self.render();
        Ok(())
    }

    fn render(&self) {
        match &self.main_view {
            Some(view) => {
                println!("App::MainView[");
                view.render();
                println!("]");
            }
            None => {}
        }
    }
}
