use crate::window::Window;
use raylib::drawing::RaylibDrawHandle;

pub enum GridItems<'a> {
    None,
    Single(&'a Window),
    SplitRight(&'a Window, &'a Window),
    SplitBottom(&'a Window, &'a Window),
    Split4(&'a Window, &'a Window, &'a Window, &'a Window),
}

impl<'a> GridItems<'a> {
    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        match self {
            GridItems::None => (),
            GridItems::Single(s) => s.draw(handle),
            GridItems::SplitRight(_, _) => todo!(),
            GridItems::SplitBottom(_, _) => todo!(),
            GridItems::Split4(_, _, _, _) => todo!(),
        }
    }
}
