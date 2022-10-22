use input::event::{self, keyboard::KeyboardEventTrait, pointer::PointerEventTrait};

#[derive(Debug)]
pub enum Event {
    Keyboard(KeyboardButton),
    Mouse(MouseButton),
    // MouseScrl(MouseScroll)
}

impl TryFrom<event::Event> for Event {
    type Error = ();
    fn try_from(event: event::Event) -> Result<Self, ()> {
        use Event::*;
        match event {
            // Keyboard events
            input::Event::Keyboard(keyboard_event) => {
                match keyboard_event {
                    event::KeyboardEvent::Key(key_event) => {
                        Ok(Keyboard(
                            KeyboardButton { 
                                key: key_event.key(), 
                                down: match key_event.key_state() {
                                    event::tablet_pad::KeyState::Pressed => true,
                                    event::tablet_pad::KeyState::Released => false,
                                }, 
                                time: key_event.time_usec()
                            }
                        ))
                    },
                    _ => Err(()),
                }
            },
            // Mouse events
            input::Event::Pointer(pointer_event) => {
                match pointer_event {
                    event::PointerEvent::Button(button_event) => {
                        Ok(Mouse(
                            MouseButton {
                                button: button_event.button(),
                                down: match button_event.button_state() {
                                    event::tablet_pad::ButtonState::Pressed => true,
                                    event::tablet_pad::ButtonState::Released => false,
                                },
                                time: button_event.time_usec()
                            }
                        ))
                    },
                    // event::PointerEvent::ScrollWheel(_) => todo!(),
                    // event::PointerEvent::ScrollFinger(_) => todo!(),
                    // event::PointerEvent::ScrollContinuous(_) => todo!(),
                    _ => Err(()),
                }
            },
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct KeyboardButton {
    pub key: u32,
    pub down: bool,
    time: u64
}

#[derive(Debug)]
pub struct MouseButton {
    pub button: u32,
    pub down: bool,
    time: u64
}

trait EventTime {
    fn microseconds(&self) -> u64;
    fn seconds(&self) -> u64 { self.microseconds() / 1000000 }
    fn minutes(&self) -> u64 { self.microseconds() / 1000000 / 60 }
    fn hours(&self) -> u64 { self.microseconds() / 1000000 / 60 / 60 }
    fn days(&self) -> u64 { self.microseconds() / 1000000 / 60 / 60 / 24 }
}

impl EventTime for KeyboardButton {
    fn microseconds(&self) -> u64 {
        self.time
    }
}

impl EventTime for MouseButton {
    fn microseconds(&self) -> u64 {
        self.time
    }
}

impl EventTime for Event {
    fn microseconds(&self) -> u64 {
        match self {
            Event::Keyboard(kb) => kb.time,
            Event::Mouse(ms) => ms.time,
        }
    }
}
