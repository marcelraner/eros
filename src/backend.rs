use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas, EventPump, Sdl, VideoSubsystem, ttf::Sdl2TtfContext};

pub trait Backend {
    fn wait_for_events(&mut self) -> Result<WaitForEventsReturnCode, String>;
    fn clear_canvas(&mut self);
    fn present_canvas_backbuffer(&mut self);
    fn render_box(&mut self, x: i32, y: i32, w: u32, h: u32);
    fn get_canvas_size(&self) -> Result<(u32, u32), String>;
    fn render_text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<(), String>;
}

#[derive(PartialEq)]
pub enum WaitForEventsReturnCode {
    Ok,
    EndApplication,
}

pub struct SDL2Backend {
    _sdl_context: Sdl,
    _video_subsystem: VideoSubsystem,
    canvas: WindowCanvas,
    ttf_context: Sdl2TtfContext,
    event_pump: EventPump,
}

impl SDL2Backend {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let event_pump = sdl_context.event_pump()?;

        Ok(Self {
            _sdl_context: sdl_context,
            _video_subsystem: video_subsystem,
            canvas,
            ttf_context: ttf_context,
            event_pump,
        })
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Backend for SDL2Backend {
    fn wait_for_events(&mut self) -> Result<WaitForEventsReturnCode, String> {
        let event = self.event_pump.wait_event();
        match event {
            sdl2::event::Event::Quit { .. }
            | sdl2::event::Event::KeyDown {
                keycode: Some(sdl2::keyboard::Keycode::Escape),
                ..
            } => {
                return Ok(WaitForEventsReturnCode::EndApplication);
            }
            _ => {
                //println!("Unhandled event: {:?}", event);
            }
        }
        return Ok(WaitForEventsReturnCode::Ok);
    }

    fn clear_canvas(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    fn present_canvas_backbuffer(&mut self) {
        self.canvas.present();
    }

    fn render_box(&mut self, x: i32, y: i32, width: u32, height: u32) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.draw_rect(Rect::new(x, y, width, height));
    }

    fn get_canvas_size(&self) -> Result<(u32, u32), String> {
        self.canvas.output_size()
    }

    fn render_text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        let previous_color = self.canvas.draw_color();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        if !text.is_empty() {
            let mut font = self.ttf_context.load_font("test_font.ttf", 128)?;
            let surface = font
                .render(text)
                .blended(Color::RGBA(128, 255, 128, 255))
                .map_err(|e| e.to_string())?;

            let texture_creator = self.canvas.texture_creator();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let target = Rect::new(x, y, width, height);

            self.canvas.copy(&texture, None, Some(target))?;
        }
        Ok(())
    }
}
