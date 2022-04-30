use crate::{Sprite, SpriteCollection};
use std::cmp::max;

// https://github.com/pythonarcade/arcade/blob/d2ce45a9b965020cde57a2a88536311e04504e6e/arcade/sprite_list/spatial_hash.py#L356

fn collided(sprite1: &Sprite, sprite2: &Sprite) -> bool {
    let coll_rad1 = max(sprite1.rect.width(), sprite1.rect.height()) as i32;
    let coll_rad2 = max(sprite2.rect.width(), sprite2.rect.height()) as i32;

    let collision_radius = coll_rad1 + coll_rad2;
    let collision_diameter = collision_radius * collision_radius;

    let diff_x = sprite1.position().0 - sprite2.position().0;
    let diff_x2 = diff_x * diff_x;

    if diff_x2 > collision_diameter {
        return false;
    }

    let diff_y = sprite1.position().1 - sprite2.position().1;
    let diff_y2 = diff_y * diff_y;

    if diff_y2 > collision_diameter {
        return false;
    }

    return sprite1.rect.has_intersection(sprite2.rect);
}

/// Check if two sprites are touching or overlapping.
pub fn check_for_collision(sprite1: &Sprite, sprite2: &Sprite) -> bool {
    collided(sprite1, sprite2)
}

/// Check if the sprite is colliding with any sprite in the collection, and return a list of
/// references to the sprites which are colliding
pub fn check_for_collision_with_collection<'a>(
    sprite: &Sprite,
    list: &'a SpriteCollection,
) -> Vec<&'a Sprite> {
    list.inner()
        .iter()
        .filter(|s| check_for_collision(sprite, s))
        .collect()
}