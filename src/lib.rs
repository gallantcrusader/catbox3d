//! Work in progress game engine, inspired by [arcade](arcade.academy/).
//!
//! ```no_run
//! use cat_box::{Event, Game, Keycode, Sprite};
//!
//! fn main() {
//!     let game = Game::new("cat_box demo", 1000, 800);
//!
//!     let mut i = 0.0;
//!     let mut s = Sprite::new("duck.png", 500, 400).unwrap();
//!     game.run(|ctx, event_pump| {
//!         i = (i + 1.0) % 360.0;
//!
//!         let (start_x, start_y) = s.position();
//!         let m = sdl2::mouse::MouseState::new(event_pump.as_ref());
//!         let x_diff = m.x() - start_x;
//!         let y_diff = m.y() - start_y;
//!
//!         let angle = (y_diff as f64).atan2(x_diff as f64);
//!         s.set_angle(angle.to_degrees());
//!
//!        for event in event_pump {
//!             match event {
//!                 Event::Quit { .. }
//!                 | Event::KeyDown {
//!                     keycode: Some(Keycode::Escape),
//!                     ..
//!                 } => game.terminate(),
//!
//!                 Event::KeyDown { keycode, .. } => {
//!                     let offset = match keycode.unwrap() {
//!                         Keycode::W | Keycode::Up => (0, 5),
//!                         Keycode::S | Keycode::Down => (0, -5),
//!                         Keycode::A | Keycode::Left => (-5, 0),
//!                         Keycode::D | Keycode::Right => (5, 0),
//!                         _ => (0, 0),
//!                     };
//!
//!                     s.translate(offset);
//!                 }
//!                 _ => {}
//!             }
//!         }
//!
//!         s.draw(ctx).unwrap();
//!     })
//!     .unwrap();
//! }
//! ```

use std::{cell::Cell, path::Path};

use sdl2::{
    image::ImageRWops,
    rect::Rect,
    render::{Canvas, TextureCreator, TextureValueError},
    rwops::RWops,
    surface::Surface,
    video::{Window, WindowBuildError, WindowContext},
    EventPump, IntegerOrSdlError, ttf::{Sdl2TtfContext, Font, FontError},
};

#[doc(no_inline)]
pub use sdl2::event::Event;
#[doc(no_inline)]
pub use sdl2::keyboard::Keycode;
#[doc(no_inline)]
pub use sdl2::pixels::Color;

/// Utility macro for cloning things into closures.
///
/// Temporary workaround for [Rust RFC 2407](https://github.com/rust-lang/rfcs/issues/2407)
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

impl From<TextureValueError> for CatboxError {
    fn from(e: TextureValueError) -> Self {
        CatboxError(format!("{}", e))
    }
}

impl From<FontError> for CatboxError {
    fn from(e: FontError) -> Self {
        CatboxError(format!("{}", e))
    }
}

pub type Result<T> = std::result::Result<T, CatboxError>;

/// Wrapper type around SDL's [`EventPump`](sdl2::EventPump). See those docs for more info.
pub struct Events {
    pump: EventPump,
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

/// Representation of a sprite.
pub struct Sprite {
    rect: Rect,
    surf: Surface<'static>,
    angle: f64,
}

impl Sprite {
    /// Create a new Sprite. The `path` is relative to the current directory while running.
    ///
    /// Don't forget to call [`draw()`](Self::draw()) after this.
    /// ```
    /// # use cat_box::*;
    /// let s = Sprite::new("duck.png", 500, 400).unwrap();
    /// ```
    pub fn new<P: AsRef<Path>>(path: P, x: i32, y: i32) -> Result<Self> {
        let ops = RWops::from_file(path, "r")?;
        let surf = ops.load()?;

        let srect = surf.rect();
        let dest_rect: Rect = Rect::from_center((x, y), srect.width(), srect.height());

        Ok(Self {
            rect: dest_rect,
            surf,
            angle: 0.0,
        })
    }

    /// Draws the sprite to the window. This should only be called inside your main event loop.
    ///
    /// ```no_run
    /// # use cat_box::*;
    /// # let mut s = Sprite::new("duck.png", 500, 400).unwrap();
    /// # let game = Game::new("sprite demo", 1000, 1000);
    /// # game.run(|ctx, _| {
    /// s.draw(ctx);
    /// # });
    /// ```
    pub fn draw(&mut self, ctx: &mut Context) -> Result<()> {
        let (creator, canvas) = ctx.inner();
        let text = creator.create_texture_from_surface(&self.surf)?;

        canvas.copy_ex(&text, None, self.rect, self.angle, None, false, false)?;

        Ok(())
    }

    /// Translate the sprite, in the form of (delta x, delta y)
    ///
    /// ```
    /// # use cat_box::*;
    /// # let mut s = Sprite::new("duck.png", 500, 400).unwrap();
    /// s.translate((5, 10));
    /// ```
    pub fn translate(&mut self, position: (i32, i32)) {
        let new_x = self.rect.x() + position.0;
        let new_y = self.rect.y() - position.1;

        self.rect.set_x(new_x);
        self.rect.set_y(new_y);
    }

    /// Set the angle of the sprite, in degrees of clockwise rotation.
    ///
    /// ```
    /// # use cat_box::*;
    /// # let mut s = Sprite::new("duck.png", 500, 400).unwrap();
    /// s.set_angle(45.0);
    /// ```
    pub fn set_angle(&mut self, angle: f64) {
        self.angle = angle;
    }

    /// Get the angle of the sprite, in degrees of clockwise rotation.
    ///
    /// ```
    /// # use cat_box::*;
    /// # let s = Sprite::new("duck.png", 500, 400).unwrap();
    /// let angle = s.angle();
    /// ```
    pub fn angle(&self) -> f64 {
        self.angle
    }

    /// Get the x and y coordinates of the center of the sprite, in the form of (x, y).
    ///
    /// ```
    /// # use cat_box::*;
    /// # let s = Sprite::new("duck.png", 500, 400).unwrap();
    /// let (x, y) = s.position();
    /// ```
    pub fn position(&self) -> (i32, i32) {
        self.rect.center().into()
    }
}

pub enum TextMode {
    Transparent {
        colour: (u8, u8, u8)
    },
    Shaded {
        foreground: (u8, u8, u8),
        background: (u8, u8, u8)
    }
}

/// Game context.
///
/// In most cases, this should never actually be used; instead, just pass it around to the various cat-box functions such as [`Sprite::draw()`].
pub struct Context {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    ttf_subsystem: Sdl2TtfContext
}

impl Context {
    fn new(canvas: Canvas<Window>, ttf_subsystem: Sdl2TtfContext) -> Self {
        let creator = canvas.texture_creator();
        Self {
            canvas,
            texture_creator: creator,
            ttf_subsystem
        }
    }

    /// Get the inner [`Canvas`](sdl2::render::Canvas) and [`TextureCreator`](sdl2::render::TextureCreator).
    ///
    /// Only use this method if you know what you're doing.
    pub fn inner(&mut self) -> (&TextureCreator<WindowContext>, &mut Canvas<Window>) {
        (&self.texture_creator, &mut self.canvas)
    }

    fn update(&mut self) {
        self.canvas.present();
    }

    fn clear(&mut self) {
        self.canvas.clear();
    }

    /// Set the background colour. See [`Canvas::set_draw_color()`](sdl2::render::Canvas::set_draw_color()) for more info.
    pub fn set_background_colour(&mut self, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
    }
}

pub fn draw_text<S: AsRef<str>>(ctx: &mut Context, text: S, font: &str, size: u16, pos: (i32, i32), mode: TextMode) -> Result<()> {
    let font =  ctx.ttf_subsystem.load_font(font, size)?;
    let renderer = font.render(text.as_ref());

    let surf = match mode {
        TextMode::Transparent { colour: (r, g, b) } => renderer.solid(Color::RGB(r, g, b)),
        TextMode::Shaded { foreground: (fr, fg, fb), background: (br, bg, bb) } => renderer.shaded(Color::RGB(fr, fg, fb), Color::RGB(br, bg, bb)),
    }?;

    drop(font);
    let (creator, canvas) = ctx.inner();
    let texture = creator.create_texture_from_surface(&surf)?;

    let srect = surf.rect();
    let dest_rect: Rect = Rect::from_center(pos, srect.width(), srect.height());

    canvas.copy_ex(&texture, None, dest_rect, 0.0, None, false, false)?;

    Ok(())
}

/// Representation of the game.
pub struct Game {
    /// The title that the window displays.
    pub title: String,
    /// The width of the opened window
    pub width: u32,
    /// The height of the opened window
    pub height: u32,
    stopped: Cell<bool>,
}

impl Game {
    /// Creates a new Game struct.
    ///
    /// Make sure to use [`Self::run()`] to actually begin the game logic.
    ///
    /// ```
    /// # use cat_box::Game;
    /// Game::new("cool game", 1000, 1000);
    /// ```
    ///
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            stopped: Cell::new(false),
        }
    }

    /// Runs the game. Note: this method blocks, as it uses an infinite loop.
    ///
    /// ```no_run
    /// # use cat_box::Game;
    /// # let game = Game::new("Cool game", 1000, 1000);
    /// game.run(|ctx, events| {
    ///     // Game logic goes here
    /// });
    /// ```
    pub fn run<F: FnMut(&mut Context, &mut Events)>(&self, mut func: F) -> Result<()> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(&self.title, self.width, self.height)
            .position_centered()
            // .opengl()
            .vulkan()
            .build()?;

        let canvas = window.into_canvas().build()?;
        let s = sdl2::ttf::init().unwrap();

        let event_pump = sdl_context.event_pump()?;

        let mut events = Events { pump: event_pump };

        let mut ctx = Context::new(canvas, s);

        loop {
            if self.stopped.get() {
                break;
            }
            ctx.clear();
            func(&mut ctx, &mut events);
            ctx.update();
        }

        Ok(())
    }

    /// Stops the game loop. This method should be called inside the closure that you passed to [`Self::run()`].
    /// ```
    /// # use cat_box::Game;
    /// # let game = Game::new("asjdhfkajlsdh", 0, 0);
    /// // ... in the game loop:
    /// game.terminate();
    /// ```
    pub fn terminate(&self) {
        self.stopped.set(true);
    }
}
