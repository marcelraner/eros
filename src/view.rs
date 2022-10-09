use crate::backend::Backend;

use self::render_util::Area;

pub mod render_util {
    pub struct Position {
        pub x: i32,
        pub y: i32,
    }

    pub struct Size {
        pub width: u32,
        pub height: u32,
    }

    pub struct Area {
        pub position: Position,
        pub size: Size,
    }

    impl Area {
        pub fn new(position_x: i32, position_y: i32, size_width: u32, size_height: u32) -> Self {
            Self {
                position: Position {
                    x: position_x,
                    y: position_y,
                },
                size: Size {
                    width: size_width,
                    height: size_height,
                },
            }
        }
    }
}

pub trait View {
    fn render(&self, backend: &mut Box<dyn Backend>, renderable_area: Area) -> Result<(), String>;
}

pub struct Text {
    text: String,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
        }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn with_text(mut self, value: &str) -> Self {
        self.text = value.to_owned();
        self
    }
}

impl View for Text {
    fn render(&self, backend: &mut Box<dyn Backend>, renderable_area: Area) -> Result<(), String> {
        println!("Text::render(\"{}\")", self.text);
        backend.render_box(
            renderable_area.position.x,
            renderable_area.position.y,
            renderable_area.size.width,
            renderable_area.size.height,
        );
        backend.render_text(
            &self.text,
            renderable_area.position.x,
            renderable_area.position.y,
            renderable_area.size.width,
            renderable_area.size.height,
        )?;
        Ok(())
    }
}
