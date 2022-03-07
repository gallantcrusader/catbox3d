use catbox::{Event, Game, Keycode, Sprite};

fn main() {
    let game = Game::new("catbox demo", 1000, 800);

    let mut i = 0;
    let s = Sprite::new("/home/yashkarandikar/code/catbox/duck.png", 500, 400).unwrap();
    game.run(|canvas, event_pump| {
        i = (i + 1) % 255;
        // canvas.set_draw_color(catbox::Color::RGB(i, 64, 255));
        // canvas.clear();
        s.draw(canvas, event_pump).unwrap();

        // let m = sdl2::mouse::MouseState::new(event_pump.as_ref());
        // println!("{}, {}", m.x(), m.y());


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
