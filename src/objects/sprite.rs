use sdl2::{
    image::ImageRWops, /*     pixels::{Color, PixelFormatEnum}, */
    rect::Rect, rwops::RWops, surface::Surface,
};
use std::{
    ops::{Deref, DerefMut},
    path::Path,
    slice::IterMut,
};

use crate::math::vec2::Vec2Int;

use crate::{Context, Result};

/// Representation of a sprite.
pub struct Sprite {
    pub rect: Rect,
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

    /// Create a new sprite using a slice of bytes, like what is returned from `include_bytes!`
    ///
    /// Don't forget to call [`draw()`](Self::draw()) after this.
    /// ```
    /// # use cat_box::*;
    /// let bytes = include_bytes!("../../duck.png");
    /// let s = Sprite::from_bytes(bytes, 500, 400).unwrap();
    /// ```
    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B, x: i32, y: i32) -> Result<Self> {
        let ops = RWops::from_bytes(bytes.as_ref())?;
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
    /// # game.run(|ctx| {
    /// s.draw(ctx);
    /// # });
    /// ```
    pub fn draw(&mut self, ctx: &mut Context) -> Result<()> {
        let (creator, canvas, _) = ctx.inner();
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
    pub fn translate<I: Into<Vec2Int>>(&mut self, position: I) {
        let position = position.into();
        let new_x = self.rect.x() + position.x;
        let new_y = self.rect.y() - position.y;

        self.rect.set_x(new_x);
        self.rect.set_y(new_y);
    }

    ///translates up by given amount
    pub fn up(&mut self, vel: i32) {
        self.translate(Vec2Int::new(0, vel));
    }

    /// translates down by given amount
    pub fn down(&mut self, vel: i32) {
        self.translate(Vec2Int::new(0, vel * -1));
    }

    /// translates left by given amount
    pub fn left(&mut self, vel: i32) {
        self.translate(Vec2Int::new(vel * -1, 0));
    }

    ///translates right by given amount
    pub fn right(&mut self, vel: i32) {
        self.translate(Vec2Int::new(vel, 0));
    }

    /// Reposition the center of the sprite in the form of (x, y)
    ///
    /// ```
    /// # use cat_box::*;
    /// # let mut s = Sprite::new("duck.png", 500, 400).unwrap();
    /// s.set_position((5, 10));
    /// ```
    pub fn set_position<I: Into<Vec2Int>>(&mut self, position: I) {
        let position = position.into();
        self.rect.center_on((position.x, position.y));
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
    #[must_use]
    pub fn angle(&self) -> f64 {
        self.angle
    }

    /// Get the x and y coordinates of the center of the sprite, in the form of (x, y).
    ///
    /// ```
    /// # use cat_box::*;
    /// # let s = Sprite::new("duck.png", 500, 400).unwrap();
    /// let (x, y) = s.position().into();
    /// ```
    #[must_use]
    pub fn position(&self) -> Vec2Int {
        self.rect.center().into()
    }
}

/// Manages a collection of [`Sprite`]s.
///
/// Technically, this is a thin wrapper around a simple [`Vec`] of sprites,
/// although with some convenience methods.
#[derive(Default)]
pub struct SpriteCollection {
    v: Vec<Sprite>,
}

impl SpriteCollection {
    /// Creates a new [`SpriteCollection`].
    ///
    /// See [`Vec::new()`] for more information.
    /// ```
    /// # use cat_box::*;
    /// let sprites = SpriteCollection::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self { v: Vec::new() }
    }

    /// Creates a new [`SpriteCollection`] with the specified capacity.
    ///
    /// The collection will be able to hold exactly `capacity` items without reallocating.
    /// ```
    /// # use cat_box::*;
    /// let sprites = SpriteCollection::with_capacity(10);
    /// ```
    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            v: Vec::with_capacity(cap),
        }
    }

    /// Draw all the sprites in this collection to the window.
    /// This should only be called inside the main event loop.
    /// ```no_run
    /// # use cat_box::*;
    /// # let mut sprites = SpriteCollection::new();
    /// # let mut game = Game::new("asjdfhalksjdf", 1, 1);
    /// # game.run(|ctx| {
    /// sprites.draw(ctx);
    /// # });
    /// ```
    pub fn draw(&mut self, ctx: &mut Context) -> Result<()> {
        for s in &mut self.v {
            s.draw(ctx)?;
        }

        Ok(())
    }

    /// Add a new [`Sprite`] to the end of this collection.
    /// ```
    /// # use cat_box::*;
    /// let mut sprites = SpriteCollection::new();
    /// let s = Sprite::new("duck.png", 500, 400).unwrap();
    /// sprites.push(s);
    /// ```
    pub fn push(&mut self, s: Sprite) {
        self.v.push(s);
    }

    /// Inserts an element at position `index` within the collection.
    /// Shifts all elements after it to the right.
    /// ```
    /// # use cat_box::*;
    /// let mut sprites = SpriteCollection::new();
    /// let s = Sprite::new("duck.png", 500, 400).unwrap();
    /// sprites.insert(s, 0);
    /// ```
    pub fn insert(&mut self, s: Sprite, index: usize) {
        self.v.insert(index, s);
    }

    /// Removes and returns the last element, or `None` if the collection is empty.
    /// ```
    /// # use cat_box::*;
    /// let mut sprites = SpriteCollection::new();
    /// let s = sprites.pop();
    /// ```
    pub fn pop(&mut self) -> Option<Sprite> {
        self.v.pop()
    }

    /// Removes and returns the element at `index`.
    /// Shifts all elements after it to the left.
    /// This method will panic if the index is out of bounds.
    /// ```
    /// # use cat_box::*;
    /// let mut sprites = SpriteCollection::new();
    /// # let s = Sprite::new("duck.png", 500, 400).unwrap();
    /// # sprites.push(s);
    /// sprites.remove(0);
    /// ```
    pub fn remove(&mut self, index: usize) -> Sprite {
        self.v.remove(index)
    }

    /// Return an iterator over the sprites in this collection.
    /// Use this to modify the sprites themselves, for example to set their position or angle.
    pub fn iter(&mut self) -> IterMut<'_, Sprite> {
        self.v.iter_mut()
    }

    /// Clears the collection, without touching the allocated capacity.
    /// ```
    /// # use cat_box::*;
    /// let mut sprites = SpriteCollection::new();
    /// # let s = Sprite::new("duck.png", 500, 400).unwrap();
    /// # sprites.push(s);
    /// sprites.clear();
    /// ```
    pub fn clear(&mut self) {
        self.v.clear();
    }

    /// Move all the elements of `other` into `Self`.
    /// ```
    /// # use cat_box::*;
    /// let mut sprites = SpriteCollection::new();
    /// let mut sprites2 = SpriteCollection::new();
    /// # let s = Sprite::new("duck.png", 500, 400).unwrap();
    /// # let s2 = Sprite::new("duck.png", 400, 500).unwrap();
    /// # sprites.push(s);
    /// # sprites2.push(s2);
    /// sprites.concat(sprites2);
    /// ```
    pub fn concat(&mut self, mut other: SpriteCollection) {
        self.v.append(&mut *other);
    }

    /// Returns the length of this vector.
    #[must_use]
    pub fn len(&self) -> usize {
        self.v.len()
    }

    /// Get a reference to the element at `index`, or `None` if it doesn't exist.
    /// ```
    /// # use cat_box::*;
    /// let mut sprites = SpriteCollection::new();
    /// # let s = Sprite::new("duck.png", 500, 400).unwrap();
    /// # sprites.push(s);
    /// let s = sprites.get(0);
    /// ```
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Sprite> {
        self.v.get(index)
    }

    /// Return the inner Vec. Only use this method if you know what you're doing.
    #[must_use]
    pub fn inner(&self) -> &Vec<Sprite> {
        &self.v
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }
}

impl Deref for SpriteCollection {
    type Target = Vec<Sprite>;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl DerefMut for SpriteCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}
