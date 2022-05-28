//! Types representing directions and locations in 2d and 3d space.
//!
//!
//! This module contains 3 major types:
//!  - [`Vec2`], a 2d float vector
//!  - [`Vec2Int`], a 2d integer vector
//!  - [`Direction`], a 2d cardinal direction
//!
//! All the types implement the expected [`From`]s and all the relevant operator traits.

use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use sdl2::rect::Point;

// Direction
/// A cardinal direction in a 2d plane.
///
/// Conversions to a [`Vec2`] or [`Vec2Int`] assume that East is positive-x and South is positive-y.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    /// North, or Vec2::from((0, -1))
    North,
    /// North, or Vec2::from((0, 1))
    South,
    /// North, or Vec2::from((1, 0))
    East,
    /// North, or Vec2::from((-1, 0))
    West,
}

#[allow(clippy::enum_glob_use)]
impl Direction {
    /// Flips this `Direction` around both the x- and y-axes.
    pub fn flipped(self) -> Self {
        self.flip_x().flip_y()
    }

    /// Flips this `Direction` around the x-axis.
    pub fn flip_x(self) -> Self {
        use Direction::*;
        match self {
            East => West,
            West => East,
            v => v,
        }
    }

    /// Flips this `Direction` around the y-axis.
    pub fn flip_y(self) -> Self {
        use Direction::*;
        match self {
            North => South,
            South => North,
            v => v,
        }
    }
}

// ...and related op impls
impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.flipped()
    }
}

#[allow(clippy::enum_glob_use)]
impl From<Direction> for Vec2 {
    fn from(v: Direction) -> Self {
        use Direction::*;
        match v {
            North => (0.0, -1.0).into(),
            South => (0.0, 1.0).into(),
            East => (1.0, 0.0).into(),
            West => (-1.0, 0.0).into(),
        }
    }
}

#[allow(clippy::enum_glob_use)]
impl From<Direction> for Vec2Int {
    fn from(v: Direction) -> Self {
        use Direction::*;
        match v {
            North => (0, -1).into(),
            South => (0, 1).into(),
            East => (1, 0).into(),
            West => (-1, 0).into(),
        }
    }
}

impl From<Point> for Vec2 {
    fn from(p: Point) -> Self {
        let x: (i32, i32) = p.into();
        x.into()
    }
}

impl From<Point> for Vec2Int {
    fn from(p: Point) -> Self {
        let x: (i32, i32) = p.into();
        x.into()
    }
}

impl Mul<f32> for Direction {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2::from(self) * rhs
    }
}

impl Mul<i32> for Direction {
    type Output = Vec2Int;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec2Int::from(self) * rhs
    }
}

// Vec2
/// A set of 2 [`f32`]s representing a location or direction in the 2d plane.
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Vec2 {
    /// The x component of the vector.
    pub x: f32,
    /// The y component of the vector.
    pub y: f32,
}

impl Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Vec2").field(&self.x).field(&self.y).finish()
    }
}

impl Vec2 {
    /// Creates a new `Vec2` with the given x- and y-values.
    ///
    /// It is often simpler, and preferred, to just write `(x, y).into()`.
    pub const fn new(x: f32, y: f32) -> Vec2 {
        Self { x, y }
    }

    /// Gets the squared magnitude of the vector.
    ///
    /// Useful for comparisons as it is faster to calculate than `magnitude`.
    pub fn sq_magnitude(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Gets the magnitude of the vector.
    pub fn magnitude(self) -> f32 {
        self.sq_magnitude().sqrt()
    }

    /// Gets the squared distance from this vector to `rhs`.
    ///
    /// Useful for comparisons as it is faster to calculate than `dist`.
    pub fn sq_dist(self, rhs: Self) -> f32 {
        (self - rhs).sq_magnitude()
    }

    /// Gets the distance from this vector to `rhs`.
    pub fn dist(self, rhs: Self) -> f32 {
        (self - rhs).magnitude()
    }

    /// Normalizes the vector, making its magnitude `1`.
    pub fn normalized(self) -> Self {
        self / self.magnitude()
    }

    /// Rounds the vector to a [`Vec2Int`].
    ///
    /// This uses `as i32` under the hood, and as such comes with all the same unfortunate edge cases. Beware.
    pub fn rounded(self) -> Vec2Int {
        #[allow(clippy::cast_possible_truncation)]
        Vec2Int {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from(v: (i32, i32)) -> Self {
        Vec2Int::from(v).to_f32()
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from(v: (f32, f32)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}

impl From<Vec2> for (f32, f32) {
    fn from(v: Vec2) -> Self {
        (v.x, v.y)
    }
}

impl PartialEq<(i32, i32)> for Vec2 {
    fn eq(&self, other: &(i32, i32)) -> bool {
        self == &Self::from(*other)
    }
}

impl PartialEq<(f32, f32)> for Vec2 {
    fn eq(&self, other: &(f32, f32)) -> bool {
        self == &Self::from(*other)
    }
}

// ...and related op impls
impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Self::from(rhs)
    }
}

impl<T> AddAssign<T> for Vec2
where
    Vec2: Add<T, Output = Self>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T> Sub<T> for Vec2
where
    Vec2: Add<T, Output = Self>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        -(-self + rhs)
    }
}

impl<T> SubAssign<T> for Vec2
where
    Vec2: Sub<T, Output = Self>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

// Vec2Int
/// A set of 2 [`i32`]s representing a location or direction in the 2d plane.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Vec2Int {
    /// The x component of the vector.
    pub x: i32,
    /// The y component of the vector.
    pub y: i32,
}

impl Debug for Vec2Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Vec2Int")
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl Vec2Int {
    /// Creates a new `Vec2` with the given x- and y-values.
    ///
    /// It is often simpler, and preferred, to just write `(x, y).into()`.
    pub const fn new(x: i32, y: i32) -> Vec2Int {
        Self { x, y }
    }

    /// Gets the squared magnitude of the vector.
    ///
    /// Useful for comparisons as it is faster to calculate than `magnitude`.
    pub fn sq_magnitude(self) -> i32 {
        self.x * self.x + self.y * self.y
    }

    /// Gets the magnitude of the vector.
    pub fn magnitude(self) -> f32 {
        #[allow(clippy::cast_precision_loss)]
        (self.sq_magnitude() as f32).sqrt()
    }

    /// Gets the squared distance from this vector to `rhs`.
    ///
    /// Useful for comparisons as it is faster to calculate than `dist`.
    pub fn sq_dist(self, rhs: Self) -> i32 {
        (self - rhs).sq_magnitude()
    }

    /// Gets the distance from this vector to `rhs`.
    pub fn dist(self, rhs: Self) -> f32 {
        (self - rhs).magnitude()
    }

    /// Casts this vector to a [`Vec2`].
    ///
    /// This uses `as f32` under the hood, and as such comes with all the same unfortunate edge cases. Beware.
    pub fn to_f32(self) -> Vec2 {
        #[allow(clippy::cast_precision_loss)]
        Vec2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

impl From<(i32, i32)> for Vec2Int {
    fn from(v: (i32, i32)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}

impl From<Vec2Int> for (i32, i32) {
    fn from(v: Vec2Int) -> Self {
        (v.x, v.y)
    }
}

impl PartialEq<(i32, i32)> for Vec2Int {
    fn eq(&self, other: &(i32, i32)) -> bool {
        self == &Self::from(*other)
    }
}

// ...and related op impls
impl Neg for Vec2Int {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1
    }
}

impl Add for Vec2Int {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Vec2Int {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Self::from(rhs)
    }
}

impl<T> AddAssign<T> for Vec2Int
where
    Vec2Int: Add<T, Output = Self>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T> Sub<T> for Vec2Int
where
    Vec2Int: Add<T, Output = Self>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        -(-self + rhs)
    }
}

impl<T> SubAssign<T> for Vec2Int
where
    Vec2Int: Sub<T, Output = Self>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl Mul<i32> for Vec2Int {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<i32> for Vec2Int {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl MulAssign<i32> for Vec2Int {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl DivAssign<i32> for Vec2Int {
    fn div_assign(&mut self, rhs: i32) {
        *self = *self / rhs;
    }
}
