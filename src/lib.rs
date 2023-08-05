//! Work in progress game engine, inspired by [arcade](https://arcade.academy/).
//!
//! ```no_run
//! use cat_box::{draw_text, Game, Sprite, SpriteCollection, get_mouse_state, get_keyboard_state};
//! use sdl2::keyboard::Scancode;
//!
//! fn main() {
//!    let game = Game::new("catbox demo", 1000, 800);
//!
//!     let mut i = 0u8;
//!     let mut s = Sprite::new("duck.png", 500, 400).unwrap();
//!     let mut s2 = Sprite::new("duck.png", 400, 500).unwrap();
//!
//!     let mut coll = SpriteCollection::new();
//!     for n in 0..10 {
//!         for o in 0..8 {
//!             let x = Sprite::new("duck.png", n * 100, o * 100).unwrap();
//!             coll.push(x);
//!         }
//!     }
//!     game.run(|ctx| {
//!         i = (i + 1) % 255;
//!         ctx.set_background_colour(i as u8, 64, 255);
//!
//!         draw_text(
//!             ctx,
//!             format!("i is {}", i),
//!             "MesloLGS NF Regular.ttf",
//!             72,
//!             (300, 300),
//!             cat_box::TextMode::Shaded {
//!                 foreground: (255, 255, 255),
//!                 background: (0, 0, 0),
//!             },
//!         )
//!         .unwrap();
//!
//!         let (start_x, start_y) = s.position().into();
//!         let m = get_mouse_state(ctx);
//!         let x_diff = m.x - start_x;
//!         let y_diff = m.y - start_y;
//!
//!         let angle = (y_diff as f64).atan2(x_diff as f64);
//!         s.set_angle(angle.to_degrees());
//!
//!         for spr in coll.iter() {
//!             let (start_x, start_y) = spr.position().into();
//!             let m = get_mouse_state(ctx);
//!             let x_diff = m.x - start_x;
//!             let y_diff = m.y - start_y;
//!
//!             let angle = (y_diff as f64).atan2(x_diff as f64);
//!             spr.set_angle(angle.to_degrees());
//!         }
//!
//!         let keys = get_keyboard_state(ctx).keys;
//!
//!         for key in keys {
//!             let offset = match key {
//!                 Scancode::Escape => {
//!                     game.terminate();
//!                     (0, 0)
//!                 },
//!                 Scancode::W | Scancode::Up => (0, 5),
//!                 Scancode::S | Scancode::Down => (0, -5),
//!                 Scancode::A | Scancode::Left => (-5, 0),
//!                 Scancode::D | Scancode::Right => (5, 0),
//!                 _ => (0, 0),
//!             };
//!
//!             s.translate(offset);
//!
//!             for spr in coll.iter() {
//!                 spr.translate(offset);
//!             }
//!         }
//!
//!         s2.draw(ctx).unwrap();
//!         s.draw(ctx).unwrap();
//!         coll.draw(ctx).unwrap();
//!     })
//!     .unwrap();
//! }
//! ```

#![warn(clippy::pedantic)]
#![allow(
    clippy::similar_names,
    clippy::needless_doctest_main,
    clippy::module_name_repetitions,
    clippy::missing_errors_doc
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod math;
pub mod sprite;
use sdl2::sys::SDL_Window;
pub use sprite::physics::*;
pub use sprite::sprite::{Sprite, SpriteCollection};

#[cfg(feature = "audio")]
use rodio::{self, source::Source, Decoder, OutputStream};
use sdl2::{
    mouse::MouseButton,
    rect::Rect,
    render::{Canvas, TextureCreator, TextureValueError},
    ttf::{FontError, InitError, Sdl2TtfContext},
    video::{Window, WindowBuildError, WindowContext},
    EventPump, IntegerOrSdlError,
};
use std::{cell::Cell, path::Path, time::Instant};

use math::vec2::Vec2Int;
#[doc(no_inline)]
pub use sdl2::{self, event::Event, keyboard::Scancode, pixels::Color};

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

macro_rules! error_from_format {
    ($($t:ty),+) => {
        $(
        impl From<$t> for CatboxError {
            fn from(e: $t) -> Self {
                CatboxError(format!("{}", e))
            }
        }
        )+
    };
}

#[derive(Clone, Debug)]
pub struct CatboxError(String);

impl From<String> for CatboxError {
    fn from(e: String) -> Self {
        CatboxError(e)
    }
}

error_from_format! {
    WindowBuildError,
    IntegerOrSdlError,
    TextureValueError,
    FontError,
    InitError
}

#[cfg(feature = "audio")]
error_from_format! {
    rodio::StreamError,
    std::io::Error,
    rodio::decoder::DecoderError,
    rodio::PlayError
}

impl std::fmt::Display for CatboxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
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

/// Game context.
///
/// In most cases, this should never actually be used; instead, just pass it around to the various cat-box functions such as [`Sprite::draw()`].
pub struct Context {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    texture_creator: TextureCreator<WindowContext>,
    ttf_subsystem: Sdl2TtfContext,
}

impl Context {
    fn new(canvas: Canvas<Window>, pump: EventPump, ttf_subsystem: Sdl2TtfContext) -> Self {
        let creator = canvas.texture_creator();
        Self {
            canvas,
            event_pump: pump,
            texture_creator: creator,
            ttf_subsystem,
        }
    }

    /// Get the inner [`Canvas`](sdl2::render::Canvas) and [`TextureCreator`](sdl2::render::TextureCreator).
    ///
    /// Only use this method if you know what you're doing.
    pub fn inner(
        &mut self,
    ) -> (
        &TextureCreator<WindowContext>,
        &mut Canvas<Window>,
        &mut EventPump,
    ) {
        (
            &self.texture_creator,
            &mut self.canvas,
            &mut self.event_pump,
        )
    }

    fn update(&mut self) {
        self.canvas.present();
    }

    fn clear(&mut self) {
        self.canvas.clear();
    }

    fn check_for_quit(&mut self) -> bool {
        let (_, _, pump) = self.inner();

        for event in pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return true;
            }
        }

        false
    }

    /// Set the background colour. See [`Canvas::set_draw_color()`](sdl2::render::Canvas::set_draw_color()) for more info.
    pub fn set_background_colour(&mut self, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
    }
}

/// Set the mode for drawing text.
#[derive(Clone, Copy, Debug)]
pub enum TextMode {
    /// Render the text transparently.
    Transparent { colour: (u8, u8, u8) },
    /// Render the text with a foreground and a background colour.
    ///
    /// This creates a box around the text.
    Shaded {
        foreground: (u8, u8, u8),
        background: (u8, u8, u8),
    },
}

/// Draw text to the screen.
///
/// This loads a font from the current directory, case sensitive.
///
/// `pos` refers to the *center* of the rendered text.
///
/// Refer to [`TextMode`] for information about colouring.
///
/// ``` no_run
/// # use cat_box::*;
/// # let game = Game::new("", 100, 100);
/// # game.run(|ctx| {
/// let mode = TextMode::Shaded {
///     foreground: (255, 255, 255),
///     background: (0, 0, 0)
/// };
/// draw_text(ctx, "text to draw", "arial.ttf", 72, (300, 300), mode);
/// # });
pub fn draw_text<S: AsRef<str>, I: Into<Vec2Int>>(
    ctx: &mut Context,
    text: S,
    font: &str,
    size: u16,
    pos: I,
    mode: TextMode,
) -> Result<()> {
    let font = ctx.ttf_subsystem.load_font(font, size)?;
    let renderer = font.render(text.as_ref());

    let surf = match mode {
        TextMode::Transparent { colour: (r, g, b) } => renderer.solid(Color::RGB(r, g, b)),
        TextMode::Shaded {
            foreground: (fr, fg, fb),
            background: (br, bg, bb),
        } => renderer.shaded(Color::RGB(fr, fg, fb), Color::RGB(br, bg, bb)),
    }?;

    drop(font);
    let (creator, canvas, _) = ctx.inner();
    let texture = creator.create_texture_from_surface(&surf)?;

    let pos = pos.into();

    let srect = surf.rect();
    let dest_rect: Rect = Rect::from_center((pos.x, pos.y), srect.width(), srect.height());

    canvas.copy_ex(&texture, None, dest_rect, 0.0, None, false, false)?;

    Ok(())
}

/// Representation of the mouse state.
pub struct MouseRepr {
    pub buttons: Vec<MouseButton>,
    pub x: i32,
    pub y: i32
}

/// Representation of the keyboard state.
pub struct KeyboardRepr {
    pub keys: Vec<Scancode>,
}

/// Get the mouse state.
/// ```no_run
/// # use cat_box::*;
/// # let game = Game::new("catbox-demo", 10, 10);
/// # game.run(|ctx| {
/// let m = get_mouse_state(ctx);
/// println!("({}, {})", m.x, m.y);
/// # });
pub fn get_mouse_state(ctx: &mut Context) -> MouseRepr {
    let (_, _, pump) = ctx.inner();

    let mouse = pump.mouse_state();

    MouseRepr {
        buttons: mouse.pressed_mouse_buttons().collect(),
        x: mouse.x(),
        y: mouse.y()
    }
}

/// Get the keyboard state.
/// ```no_run
/// # use cat_box::*;
/// # let game = Game::new("catbox-demo", 10, 10);
/// # game.run(|ctx| {
/// let k = get_keyboard_state(ctx);
/// for code in k.keys {
///     println!("{}", code);
/// }
/// # });
pub fn get_keyboard_state(ctx: &mut Context) -> KeyboardRepr {
    let (_, _, pump) = ctx.inner();

    let keyboard = pump.keyboard_state();

    KeyboardRepr {
        keys: keyboard.pressed_scancodes().collect(),
    }
}

/// Representation of the game.
pub struct Game {
    /// The title that the window displays.
    pub title: String,
    /// The width of the opened window
    pub width: u32,
    /// The height of the opened window
    pub height: u32,
    pub time: Cell<Instant>,
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
    #[must_use]
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            time: Instant::now().into(),
            stopped: Cell::new(false),
        }
    }

    ///Gets time elapsed since last timer reset in milliseconds
    ///
    ///Run this within the game loop
    ///```
    ///# use cat_box::Game;
    ///# let game = Game::new("wacky game", 1000, 1000);
    ///# game.run(|ctx| {
    ///     if game.step() >= 1000
    ///     {
    ///         println!("A second has passed approx!");
    ///         game.t_reset();
    ///     }
    ///}).unwrap();
    ///```
    pub fn step(&self) -> u128 {
        self.time.get().elapsed().as_millis()
    }

    ///Resets in-game timer
    pub fn t_reset(&self) {
        self.time.set(Instant::now());
    }

    /// Runs the game. Note: this method blocks, as it uses an infinite loop.
    ///
    /// ```no_run
    /// # use cat_box::Game;
    /// # let game = Game::new("Cool game", 1000, 1000);
    /// game.run(|ctx| {
    ///     // Game logic goes here
    /// });
    /// ```
    pub fn run<F: FnMut(&mut Context)>(&self, mut func: F) -> Result<()> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let mut window_build = video_subsystem.window(&self.title, self.width, self.height);

        //init window
        let window = if cfg!(feature = "opengl") {
            window_build.opengl().build()?
        } else if cfg!(feature = "vulkan") {
            window_build.vulkan().build()?
        } else {
            window_build.build()?
        };

        let canvas = window.into_canvas().build()?;
        let s = sdl2::ttf::init()?;

        let event_pump = sdl_context.event_pump()?;

        let mut ctx = Context::new(canvas, event_pump, s);

        loop {
            if self.stopped.get() || ctx.check_for_quit() {
                break;
            }
            ctx.clear();
            func(&mut ctx);
            ctx.update();
        }

        Ok(())
    }

    /// Runs the game from a raw pointer to a Window (blocks)
    pub unsafe fn run_from_ll<F: FnMut(&mut Context)>(
        &self,
        win: *mut SDL_Window,
        mut func: F,
    ) -> Result<()> {
        unsafe {
            let sdl_context = sdl2::init()?;
            let video_subsystem = sdl_context.video()?;

            let window = Window::from_ll(video_subsystem, win);

            let canvas = window.into_canvas().build()?;
            let s = sdl2::ttf::init()?;

            let event_pump = sdl_context.event_pump()?;

            let mut ctx = Context::new(canvas, event_pump, s);

            loop {
                if self.stopped.get() || ctx.check_for_quit() {
                    break;
                }
                ctx.clear();
                func(&mut ctx);
                ctx.update();
            }

            Ok(())
        }
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

#[cfg(feature = "audio")]
#[cfg_attr(docsrs, doc(cfg(feature = "audio")))]
/// Plays an audio file given the path of file and plays it for y seconds
/// ```no_run
/// # use cat_box::play;
/// play("/path/to/song.mp71", 15);
/// ```
pub fn play<P: AsRef<Path> + Send + 'static>(
    path: P,
    time: u64,
) -> std::thread::JoinHandle<Result<()>> {
    use std::fs::File;
    use std::io::BufReader;
    use std::thread;

    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open(path)?);
        // Decode that sound file into a source
        let source = Decoder::new(file)?;
        // Play the sound directly on the device
        stream_handle.play_raw(source.convert_samples())?;

        // The sound plays in a separate audio thread,
        // so we need to keep the main thread alive while it's playing.
        std::thread::sleep(std::time::Duration::from_secs(time));

        Ok(())
    })
}
