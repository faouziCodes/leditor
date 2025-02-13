use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::component::Component;

enum CursorState {
    Line,
    Block,
}

pub struct Cursor {
    pos_x: u16,
    pos_y: u16,
    state: CursorState,
    font_h: u32,
    font_w: u32,
}

impl Component for Cursor {
    fn handle_event(&mut self, event: crate::event::ComponentEvent) {
        todo!()
    }

    fn draw(&self, handle: &mut RaylibDrawHandle) {
        let font = handle.get_font_default();
        handle.draw_rectangle(x, y, self.font_w, self.font_h, Color::BLACK);
    }
}
