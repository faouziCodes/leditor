use crate::event::Event;
use crate::grid::GridItems;
use crate::window::Window;
use raylib::RaylibHandle;

#[allow(unused)]
pub struct Editor<'a> {
    selected: Option<&'a mut Window>,
    grid: GridItems<'a>,
}

impl<'a> Editor<'a> {
    pub fn get_event(&mut self, handle: &mut RaylibHandle) -> Option<Event> {
        match handle.get_key_pressed()? {
            _ => todo!("Return the event bound to the key."),
        }
    }

    pub fn run(self) {
        let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();
        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);
            self.grid.draw(&mut d)
        }
    }

    pub fn new() -> Self {
        Editor {
            selected: None,
            grid: GridItems::None,
        }
    }
}
