use std::{cell::Cell, path::Path};

use sdl2::{
    render::Canvas,
    video::{Window, WindowBuildError, WindowSurfaceRef},
    IntegerOrSdlError, rect::Rect, surface::Surface, rwops::RWops, image::ImageRWops, EventPump, event::EventPollIterator,
};

pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
pub use sdl2::pixels::Color;

#[macro_export]
macro_rules! cloned {
    ($thing:ident => $e:expr) => {
        let $thing = $thing.clone();
        $e
    };
    ($($thing:ident),* => $e:expr) => {
        $( let $thing = $thing.clone(); )*
        $e
    }
}

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

pub struct Events {
    pump: EventPump
}

impl AsRef<EventPump> for Events {
    fn as_ref(&self) -> &EventPump {
        &self.pump
    }
}

impl AsMut<EventPump> for Events {
    fn as_mut(&mut self) -> &mut EventPump {
        &mut self.pump
    }
}

impl Iterator for Events {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        self.pump.poll_event()
    }
}

pub struct Sprite {
    rect: Rect,
    surf: Surface<'static>,
}

impl Sprite {
    pub fn new<P: AsRef<Path>>(path: P, x: i32, y: i32) -> Result<Self> {
        let ops = RWops::from_file(path, "r")?;
        let surf = ops.load()?; 

        let srect = surf.rect();
        let dest_rect: Rect = Rect::from_center((x, y), srect.width(), srect.height());

        Ok(Self {
            rect: dest_rect,
            surf,
        })
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, events: &Events) -> Result<()> {
        canvas.fill_rect(None)?;
        canvas.clear();

        let mut surface = canvas.window().surface(events.as_ref())?;

        self.surf.blit(None, &mut *surface, self.rect)?;

        surface.finish()?;

        Ok(())
    }

    pub fn translate(&mut self, position: (i32, i32)) {
        let new_x = self.rect.x() - position.0;
        let new_y = self.rect.y() - position.1;

        self.rect.set_x(new_x);
        self.rect.set_y(new_y);
    }
}

pub struct Game {
    pub title: String,
    pub width: u32,
    pub height: u32,
    stopped: Cell<bool>,
}

impl Game {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            stopped: Cell::new(false),
        }
    }

    pub fn run<F: FnMut(&mut Canvas<Window>, &mut Events)>(&self, mut func: F) -> Result<()> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(&self.title, self.width, self.height)
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().build()?;

        let event_pump = sdl_context.event_pump()?;

        let mut events = Events {
            pump: event_pump
        };


        loop {
            if self.stopped.get() {
                break;
            }
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            func(&mut canvas, &mut events);
            // canvas.present();
        }

        Ok(())
    }

    pub fn terminate(&self) {
        self.stopped.set(true);
    }
}
