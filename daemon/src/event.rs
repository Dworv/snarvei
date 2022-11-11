use crate::InputEvent;

#[derive(Debug)]
pub enum Event {
    Input(InputEvent),
    ConfigUpdate
}