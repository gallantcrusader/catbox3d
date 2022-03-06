use std::cell::Cell;

use sdl2::{video::{WindowBuildError, Window}, IntegerOrSdlError, render::Canvas, EventPump};

pub use sdl2::pixels::Color;
pub use sdl2::keyboard::Keycode;
pub use sdl2::event::Event;


#[derive(Debug)]
pub struct CatboxError(String);

impl From<WindowBuildError> for CatboxError {
    fn from(e: WindowBuildError) -> Self {
        CatboxError(format!("{}", e))
    }
}

impl From<String> for CatboxError {
    fn from(e: String) -> Self {
        CatboxError(e)
    }
}

impl From<IntegerOrSdlError> for CatboxError {
    fn from(e: IntegerOrSdlError) -> Self {
        CatboxError(format!("{}", e))
    }
}

pub type Result<T> = std::result::Result<T, CatboxError>;

pub struct Game {
    pub title: String,
    pub width: u32,
    pub height: u32,
    stopped: Cell<bool>
}

impl Game {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            stopped: Cell::new(false)
        }
    }

    pub fn run<F: Fn(&mut Canvas<Window>, &mut EventPump)>(&self, func: F) -> Result<()> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem.window(&self.title, self.width, self.height)
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().build()?;

        let mut event_pump = sdl_context.event_pump()?;

        loop {
            if self.stopped.get() {
                break;
            }
            func(&mut canvas, &mut event_pump);
            canvas.present();
        }

        Ok(())
    }

    pub fn terminate(&self) {
        self.stopped.set(true);
    }
}