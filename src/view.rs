pub trait View {
    fn render(&self);
}

pub struct Text {
    text: String,
}

impl Text {
    pub fn new() -> Self {
        Self {
            text: "".to_owned(),
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
    fn render(&self) {
        println!("TextView[\"{}\"]", self.text);
    }
}
