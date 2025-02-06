// Events : -> selected_window
// Grid : Draw after event? This part I'm not completely sure of, it also depands very much of the
// gui framework we shall use.

use crate::editor;

pub struct Gui<'a> {
    grid: editor::Editor<'a>,
}

impl<'a> Gui<'a> {
    pub fn run() {
        todo!("running the gui")
    }
}
