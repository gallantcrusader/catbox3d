use catbox::{Event, Game, Keycode};

fn main() {
    let game = Game::new("catbox demo", 1000, 800);

    let mut i = 0;
    game.run(|canvas, event_pump| {
        i = (i + 1) % 255;
        canvas.set_draw_color(catbox::Color::RGB(i, 64, 255));
        canvas.clear();
        for event in event_pump {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => game.terminate(),
                _ => {}
            }
        }
    })
    .unwrap();
}
