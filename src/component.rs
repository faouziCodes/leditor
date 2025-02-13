use crate::event;
use raylib::prelude::RaylibDrawHandle;

pub trait Component {
    fn handle_event(&mut self, event: event::ComponentEvent);
    fn draw(&self, handle: &mut RaylibDrawHandle);
}
