# cat-box

Work in progress game engine, inspired by [arcade](arcade.academy/).

```rs
use cat_box::{Event, Game, Keycode, Sprite};

fn main() {
    let game = Game::new("cat-box demo", 1000, 800);

    let mut i = 0.0;
    let mut s = Sprite::new("duck.png", 500, 400).unwrap();
    game.run(|ctx, event_pump| {
        i = (i + 1.0) % 360.0;

        let (start_x, start_y) = s.position();
        let m = sdl2::mouse::MouseState::new(event_pump.as_ref());
        let x_diff = m.x() - start_x;
        let y_diff = m.y() - start_y;

        let angle = (y_diff as f64).atan2(x_diff as f64);
        s.set_angle(angle.to_degrees());

        for event in event_pump {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => game.terminate(),

                Event::KeyDown { keycode, .. } => {
                    let offset = match keycode.unwrap() {
                        Keycode::W | Keycode::Up => (0, 5),
                        Keycode::S | Keycode::Down => (0, -5),
                        Keycode::A | Keycode::Left => (-5, 0),
                        Keycode::D | Keycode::Right => (5, 0),
                        _ => (0, 0),
                    };

                    s.translate(offset);
                }
                _ => {}
            }
        }

        s.draw(ctx).unwrap();
    })
    .unwrap();
}
```
