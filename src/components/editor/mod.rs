use raylib::prelude::RaylibDrawHandle;

use crate::component::Component;

pub mod buffer;
pub mod cursor;

enum EditorState {
    Edit,
    Show,
}

pub struct Editor {
    buffer: buffer::Buffer,
    cursor: cursor::Cursor,
    state: EditorState,
}

impl Component for Editor {
    fn handle_event(&mut self, event: crate::event::ComponentEvent) {
        todo!()
    }

    fn draw(&self, handle: &mut RaylibDrawHandle) {}
}
