use std::path::Path;

use crate::{objects::sprite::Sprite, MouseRepr};
use crate::{Context, Result};
use sdl2::mouse::MouseButton;

pub enum Actions {
    PRESSED,
    DEPRESSED,
    HOVER,
    NONE,
}

pub struct Button {
    pub sprite: Sprite,
    pub x: i32,
    pub y: i32,
    //pub pressed: bool,
    pub actions: Vec<Actions>,
}

impl Button {
    pub fn new<P: AsRef<Path>>(path: P, x: i32, y: i32) -> Result<Self> {
        let s = Sprite::new(path, x, y)?;
        Ok(Button {
            sprite: s,
            x,
            y,
            actions: Vec::new(),
        })
    }

    pub fn draw(&mut self, ctx: &mut Context) -> Result<()> {
        self.sprite.set_position((self.x, self.y));

        /*if self.pressed {
            f();
            self.pressed = false;
        }*/

        self.sprite.draw(ctx)
    }
    pub fn clicked(&mut self, m: &MouseRepr, f: fn()) {
        let state = m.buttons.get(0).unwrap_or_else(|| &MouseButton::Unknown);
        /*if state == &MouseButton::Left {
            if m.coll_with_sprite(&self.sprite) {
                self.pressed = true;
            } else {
                self.pressed = false;
            }
        } else {
            self.pressed = false;
        }*/
        if let Some(action) = self.actions.pop() {
            match action {
                Actions::PRESSED => f(),
                _ => (),
            }
        }
        if state == &MouseButton::Left {
            if m.coll_with_sprite(&self.sprite) {
                self.actions.push(Actions::PRESSED);
            }
        } else if m.coll_with_sprite(&self.sprite) {
            self.actions.push(Actions::HOVER);
        } else {
            if self.actions.len() > 0 {
                self.actions.pop().unwrap();
            }
        }
    }
}
