use sfml::graphics::{Shape, Transformable, RenderTarget};

mod game;


const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;

fn main() {
    let mut window = sfml::graphics::RenderWindow::new(
        sfml::window::VideoMode::new(WINDOW_WIDTH, WINDOW_HEIGHT, 32),
        "Wizard posting - Night of the living shitposters",
        sfml::window::Style::DEFAULT,
        &Default::default(),
    );

    let mut circle = sfml::graphics::CircleShape::new(100., 30);
    circle.set_fill_color(sfml::graphics::Color::RED);
    circle.set_position((100., 100.));

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                sfml::window::Event::Closed => window.close(),
                _ => {}
            }
        }

        window.clear(sfml::graphics::Color::BLACK);
        window.draw(&circle);
        window.display();
    }
}
