//! Basic physics utilities for cat-box.
//!
//! Still ***very much work-in-progress***

#![allow(clippy::cast_possible_wrap)]

use crate::{Sprite, SpriteCollection};
use std::cmp::max;

// https://github.com/pythonarcade/arcade/blob/d2ce45a9b965020cde57a2a88536311e04504e6e/arcade/sprite_list/spatial_hash.py#L356

fn collided(sprite1: &Sprite, sprite2: &Sprite) -> bool {
    let coll_rad1 = max(sprite1.rect.width(), sprite1.rect.height()) as i32;
    let coll_rad2 = max(sprite2.rect.width(), sprite2.rect.height()) as i32;

    let collision_radius = coll_rad1 + coll_rad2;
    let collision_diameter = collision_radius * collision_radius;

    let diff_x = sprite1.position().x - sprite2.position().x;
    let diff_x2 = diff_x * diff_x;

    if diff_x2 > collision_diameter {
        return false;
    }

    let diff_y = sprite1.position().y - sprite2.position().y;
    let diff_y2 = diff_y * diff_y;

    if diff_y2 > collision_diameter {
        return false;
    }

    sprite1.rect.has_intersection(sprite2.rect)
}

/// Check if two sprites are touching or overlapping.
#[must_use]
pub fn check_for_collision(sprite1: &Sprite, sprite2: &Sprite) -> bool {
    collided(sprite1, sprite2)
}

#[must_use]
pub fn check_for_collision_with_point(sprite1: &Sprite, point: &crate::math::vec2::Vec2Int) -> bool {
    let coll_rad = max(sprite1.rect.width(), sprite1.rect.height()) as i32;

    let coll2 = coll_rad * coll_rad;

    let diff_x = sprite1.position().x - point.x;
    let diff_x2 = diff_x * diff_x;

    if diff_x2 > coll2 { return false; }

    let diff_y = sprite1.position().y - point.y;
    let diff_y2 = diff_y * diff_y;

    if diff_y2 > coll2 {
        return false;
    }

    return true;

}

/// Check if the sprite is colliding with any sprite in the collection, and return a list of
/// references to the sprites which are colliding
#[must_use]
pub fn check_for_collision_with_collection<'a>(
    sprite: &Sprite,
    list: &'a SpriteCollection,
) -> Vec<&'a Sprite> {
    list.inner()
        .iter()
        .filter(|s| check_for_collision(sprite, s))
        .collect()
}
