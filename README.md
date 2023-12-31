# cat-box

[![crates.io](https://img.shields.io/crates/v/cat-box.svg)](https://crates.io/crates/cat-box)
[![Documentation](https://docs.rs/cat-box/badge.svg)](https://docs.rs/cat-box)
[![MIT License](https://img.shields.io/crates/l/cat-box.svg)](./LICENSE)

Work in progress game engine, inspired by [arcade](https://arcade.academy/).

## Getting started

Add `cat-box` to your `Cargo.toml`, and then follow the example below. Read [the documentation](https://docs.rs/cat-box/latest/cat_box/) for more info.

```rs
use cat_box::{draw_text, Game, Sprite, SpriteCollection, get_mouse_state, get_keyboard_state};
use sdl2::keyboard::Scancode;

fn main() {
    let game = Game::new("catbox demo", 1000, 800);

    let mut i = 0u8;
    let mut s = Sprite::new("duck.png", 500, 400).unwrap();
    let mut s2 = Sprite::new("duck.png", 400, 500).unwrap();

    let mut coll = SpriteCollection::new();
    for n in 0..10 {
        for o in 0..8 {
            let x = Sprite::new("duck.png", n * 100, o * 100).unwrap();
            coll.push(x);
        }
    } 
    game.run(|ctx| {
        i = (i + 1) % 255;
        ctx.set_background_colour(i as u8, 64, 255);

        draw_text(
            ctx,
            format!("i is {}", i),
            "MesloLGS NF Regular.ttf",
            72,
            (300, 300),
            cat_box::TextMode::Shaded {
                foreground: (255, 255, 255),
                background: (0, 0, 0),
            },
        )
        .unwrap();

        let (start_x, start_y) = s.position();
        let m = get_mouse_state(ctx);
        let x_diff = m.x - start_x;
        let y_diff = m.y - start_y;

        let angle = (y_diff as f64).atan2(x_diff as f64);
        s.set_angle(angle.to_degrees());

        for spr in coll.iter() {
            let (start_x, start_y) = spr.position();
            let m = get_mouse_state(ctx);
            let x_diff = m.x - start_x;
            let y_diff = m.y - start_y;

            let angle = (y_diff as f64).atan2(x_diff as f64);
            spr.set_angle(angle.to_degrees());
        }

        let keys = get_keyboard_state(ctx).keys;

        for key in keys {
            let offset = match key {
                Scancode::Escape => {
                    game.terminate();
                    (0, 0)
                },
                Scancode::W | Scancode::Up => (0, 5),
                Scancode::S | Scancode::Down => (0, -5),
                Scancode::A | Scancode::Left => (-5, 0),
                Scancode::D | Scancode::Right => (5, 0),
                _ => (0, 0),
            };

            s.translate(offset);

            for spr in coll.iter() {
                spr.translate(offset);
            }
        }

        s2.draw(ctx).unwrap();
        s.draw(ctx).unwrap();
        coll.draw(ctx).unwrap();
    })
    .unwrap();
}
```