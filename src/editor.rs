use crate::config::Config;
use crate::window::Window;
use raylib::color::Color;
use raylib::prelude::RaylibDraw;

pub enum GridItems<'a> {
    Single(&'a Window),
    SplitRight(&'a Window, &'a Window),
    SplitBottom(&'a Window, &'a Window),
    Split4(&'a Window, &'a Window, &'a Window, &'a Window),
}

pub struct Editor<'a> {
    selected: Option<&'a mut Window>,
    config: Config,
    items: GridItems<'a>,
}

impl<'a> Editor<'a> {
    pub fn run() {
        let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::WHITE);
            d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        }
    }
}
