use catbox::{Event, Game, Keycode, Sprite};

fn main() {
    let game = Game::new("catbox demo", 1000, 800);

    let mut i = 0.0;
    let mut s = Sprite::new("duck.png", 500, 400).unwrap();
    game.run(|canvas, event_pump| {
        i = (i + 1.0) % 360.0;

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
                        Keycode::A | Keycode::Left => (5, 0),
                        Keycode::D | Keycode::Right => (-5, 0),
                        _ => (0, 0),
                    };

                    s.translate(offset);
                }
                _ => {}
            }
        }

        s.angle(i);
        s.draw(canvas).unwrap();
    })
    .unwrap();
}
