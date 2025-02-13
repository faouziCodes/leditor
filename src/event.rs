pub enum EditorEvent {}
pub enum WindowEvent {}
pub enum ComponentEvent {}

pub enum Event {
    EditorEvent(EditorEvent),
    WindowEvent(WindowEvent),
    ComponentEvent(ComponentEvent),
}
