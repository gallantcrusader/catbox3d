use catbox::{Event, Game, Keycode};

fn main() {
    let mut game = Game::new("catbox demo", 1000, 800);

    game.run(|canvas, event_pump| {
        canvas.set_draw_color(catbox::Color::RGB(0, 59, 111));
        canvas.clear();
        for event in event_pump.poll_iter() {
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
