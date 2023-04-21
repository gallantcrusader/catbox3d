use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

// Vec3
/// A set of 3 [`f32`]s representing a location or direction in the 3d plane.
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Vec3 {
    /// The x component of the vector.
    pub x: f32,
    /// The y component of the vector.
    pub y: f32,
    /// The z component of the vector
    pub z: f32,
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Vec3")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl Vec3 {
    /// Creates a new `Vec3` with the given x- and y-values.
    ///
    /// It is often simpler, and preferred, to just write `(x, y).into()`.
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Self { x, y, z }
    }

    /// Gets the squared magnitude of the vector.
    ///
    /// Useful for comparisons as it is faster to calculate than `magnitude`.
    #[must_use]
    pub fn sq_magnitude(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Gets the magnitude of the vector.
    #[must_use]
    pub fn magnitude(self) -> f32 {
        self.sq_magnitude().sqrt()
    }

    /// Gets the squared distance from this vector to `rhs`.
    ///
    /// Useful for comparisons as it is faster to calculate than `dist`.
    #[must_use]
    pub fn sq_dist(self, rhs: Self) -> f32 {
        (self - rhs).sq_magnitude()
    }

    /// Gets the distance from this vector to `rhs`.
    #[must_use]
    pub fn dist(self, rhs: Self) -> f32 {
        (self - rhs).magnitude()
    }

    /// Normalizes the vector, making its magnitude `1`.
    #[must_use]
    pub fn normalized(self) -> Self {
        self / self.magnitude()
    }

    /// Rounds the vector to a [`Vec3Int`].
    ///
    /// This uses `as i32` under the hood, and as such comes with all the same unfortunate edge cases. Beware.
    #[must_use]
    pub fn rounded(self) -> Vec3Int {
        #[allow(clippy::cast_possible_truncation)]
        Vec3Int {
            x: self.x as i32,
            y: self.y as i32,
            z: self.z as i32,
        }
    }
}

impl From<(i32, i32, i32)> for Vec3 {
    fn from(v: (i32, i32, i32)) -> Self {
        Vec3Int::from(v).to_f32()
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(v: (f32, f32, f32)) -> Self {
        Self {
            x: v.0,
            y: v.1,
            z: v.2,
        }
    }
}

impl From<Vec3> for (f32, f32, f32) {
    fn from(v: Vec3) -> Self {
        (v.x, v.y, v.z)
    }
}

impl PartialEq<(i32, i32, i32)> for Vec3 {
    fn eq(&self, other: &(i32, i32, i32)) -> bool {
        self == &Self::from(*other)
    }
}

impl PartialEq<(f32, f32, f32)> for Vec3 {
    fn eq(&self, other: &(f32, f32, f32)) -> bool {
        self == &Self::from(*other)
    }
}

// ...and related op impls
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> AddAssign<T> for Vec3
where
    Vec3: Add<T, Output = Self>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T> Sub<T> for Vec3
where
    Vec3: Add<T, Output = Self>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        -(-self + rhs)
    }
}

impl<T> SubAssign<T> for Vec3
where
    Vec3: Sub<T, Output = Self>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

// Vec3Int
/// A set of 2 [`i32`]s representing a location or direction in the 2d plane.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Vec3Int {
    /// The x component of the vector.
    pub x: i32,
    /// The y component of the vector.
    pub y: i32,

    pub z: i32,
}

impl Debug for Vec3Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Vec3Int")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl Vec3Int {
    /// Creates a new `Vec3` with the given x- and y-values.
    ///
    /// It is often simpler, and preferred, to just write `(x, y).into()`.
    #[must_use]
    pub const fn new(x: i32, y: i32, z: i32) -> Vec3Int {
        Self { x, y, z }
    }

    /// Gets the squared magnitude of the vector.
    ///
    /// Useful for comparisons as it is faster to calculate than `magnitude`.
    #[must_use]
    pub fn sq_magnitude(self) -> i32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Gets the magnitude of the vector.
    #[must_use]
    pub fn magnitude(self) -> f32 {
        #[allow(clippy::cast_precision_loss)]
        (self.sq_magnitude() as f32).sqrt()
    }

    /// Gets the squared distance from this vector to `rhs`.
    ///
    /// Useful for comparisons as it is faster to calculate than `dist`.
    #[must_use]
    pub fn sq_dist(self, rhs: Self) -> i32 {
        (self - rhs).sq_magnitude()
    }

    /// Gets the distance from this vector to `rhs`.
    #[must_use]
    pub fn dist(self, rhs: Self) -> f32 {
        (self - rhs).magnitude()
    }

    /// Casts this vector to a [`Vec3`].
    ///
    /// This uses `as f32` under the hood, and as such comes with all the same unfortunate edge cases. Beware.
    #[must_use]
    pub fn to_f32(self) -> Vec3 {
        #[allow(clippy::cast_precision_loss)]
        Vec3 {
            x: self.x as f32,
            y: self.y as f32,
            z: self.z as f32,
        }
    }
}

impl From<(i32, i32, i32)> for Vec3Int {
    fn from(v: (i32, i32, i32)) -> Self {
        Self {
            x: v.0,
            y: v.1,
            z: v.2,
        }
    }
}

impl From<Vec3Int> for (i32, i32, i32) {
    fn from(v: Vec3Int) -> Self {
        (v.x, v.y, v.z)
    }
}

impl PartialEq<(i32, i32, i32)> for Vec3Int {
    fn eq(&self, other: &(i32, i32, i32)) -> bool {
        self == &Self::from(*other)
    }
}

// ...and related op impls
impl Neg for Vec3Int {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1
    }
}

impl Add for Vec3Int {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> AddAssign<T> for Vec3Int
where
    Vec3Int: Add<T, Output = Self>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T> Sub<T> for Vec3Int
where
    Vec3Int: Add<T, Output = Self>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        -(-self + rhs)
    }
}

impl<T> SubAssign<T> for Vec3Int
where
    Vec3Int: Sub<T, Output = Self>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl Mul<i32> for Vec3Int {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<i32> for Vec3Int {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl MulAssign<i32> for Vec3Int {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl DivAssign<i32> for Vec3Int {
    fn div_assign(&mut self, rhs: i32) {
        *self = *self / rhs;
    }
}
