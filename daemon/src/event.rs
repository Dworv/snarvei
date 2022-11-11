use crate::{InputEvent, ConfigEvent};

#[derive(Debug)]
pub enum Event {
    Input(InputEvent),
    Config(ConfigEvent)
}