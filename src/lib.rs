mod backend;
pub mod view;

use backend::{Backend, SDL2Backend, WaitForEventsReturnCode};
use view::View;

use crate::view::render_util::Area;

pub struct App {
    main_view: Option<Box<dyn View>>,
    backend: Box<dyn Backend>,
}

impl App {
    pub fn new() -> Self {
        Self {
            main_view: None,
            backend: SDL2Backend::new().unwrap().boxed(),
        }
    }

    pub fn with_main_view(mut self, view: Box<dyn View>) -> Self {
        self.main_view = Some(view);
        self
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.render()?;
        'run: loop {
            if self.backend.wait_for_events()? == WaitForEventsReturnCode::EndApplication {
                break 'run;
            }
            self.render()?;
        }
        Ok(())
    }

    fn render(&mut self) -> Result<(), String> {
        println!("App::render()");
        let canvas_size = self.backend.get_canvas_size()?;
        self.backend.clear_canvas();
        if let Some(main_view) = &self.main_view {
            main_view.render(
                &mut self.backend,
                Area::new(0, 0, canvas_size.0, canvas_size.1),
            )?;
        }
        self.backend.present_canvas_backbuffer();
        Ok(())
    }
}
