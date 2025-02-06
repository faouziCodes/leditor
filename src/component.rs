use crate::event;
use raylib::RaylibHandle;

pub trait Component {
    fn handle_event(&mut self, event: event::Event);
    fn draw(&self, handle: &mut RaylibHandle);
}
