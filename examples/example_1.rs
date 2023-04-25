#![warn(clippy::pedantic)]

use cat_box::{draw_text, get_keyboard_state, get_mouse_state, Game, Sprite, SpriteCollection};
use sdl2::keyboard::Scancode;

fn main() {
    let game = Game::new("catbox demo", 1000, 800);

    let mut i = 0u8;
    let mut s = Sprite::new("duck.png", 500, 400).unwrap();
    let mut s2 = Sprite::new("duck.png", 400, 500).unwrap();

    let bytes = include_bytes!("../duck.png");

    let mut coll = SpriteCollection::new();
    for n in 0..10 {
        for o in 0..8 {
            let x = Sprite::from_bytes(bytes, n * 100, o * 100).unwrap();
            coll.push(x);
        }
    }
    #[cfg(feature = "audio")]
    cat_box::play("output.mp3", 120);
    game.run(|ctx| {
        let (_, _, _) = ctx.inner();

        //let win = b.window_mut();
        /* let instance_schtuff = win.vulkan_instance_extensions().unwrap(); */

        if game.step() >= 1 {
            i = (i + 1) % 255;
            ctx.set_background_colour(i, 64, 255);

            draw_text(
                ctx,
                format!("i is {i}"),
                "MesloLGS NF Regular.ttf",
                72,
                (300, 300),
                cat_box::TextMode::Shaded {
                    foreground: (255, 255, 255),
                    background: (0, 0, 0),
                },
            )
            .unwrap();

            let (start_x, start_y) = s.position().into();
            let m = get_mouse_state(ctx);
            let x_diff = m.x - start_x;
            let y_diff = m.y - start_y;

            let angle = f64::from(y_diff).atan2(f64::from(x_diff));
            s.set_angle(angle.to_degrees());

            for spr in coll.iter() {
                let (start_x, start_y) = spr.position().into();
                let m = get_mouse_state(ctx);
                let x_diff = m.x - start_x;
                let y_diff = m.y - start_y;

                let angle = f64::from(y_diff).atan2(f64::from(x_diff));
                spr.set_angle(angle.to_degrees());
            }

            let keys = get_keyboard_state(ctx).keys;

            for key in keys {
                let offset = match key {
                    Scancode::Escape => {
                        game.terminate();
                        (0, 0)
                    }
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

            if !cat_box::check_for_collision_with_collection(&s2, &coll).is_empty() {
                println!("Sprites collided! {i}");
            }

            game.t_reset();
        }
        s2.draw(ctx).unwrap();
        s.draw(ctx).unwrap();
        coll.draw(ctx).unwrap();
    })
    .unwrap();
}
