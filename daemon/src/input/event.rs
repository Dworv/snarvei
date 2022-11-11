use libinput::event::{self, keyboard::KeyboardEventTrait, pointer::PointerEventTrait};

/// An event from the user
/// 
/// This enum acts as a middleman between `input.rs`
/// and the main code for ease of use.
#[derive(Debug)]
pub enum InputEvent {
    /// A keyboard button being pressed.
    Keyboard(KeyboardEvent),
    /// A mouse button being pressed.
    MouseButton(MouseButtonEvent),
    // MouseScrl(MouseScroll)
}

impl TryFrom<event::Event> for InputEvent {
    type Error = ();
    fn try_from(event: event::Event) -> Result<Self, ()> {
        use InputEvent::*;
        match event {
            // Keyboard events
            libinput::Event::Keyboard(keyboard_event) => {
                match keyboard_event {
                    event::KeyboardEvent::Key(key_event) => {
                        Ok(Keyboard(
                            KeyboardEvent { 
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
            libinput::Event::Pointer(pointer_event) => {
                match pointer_event {
                    event::PointerEvent::Button(button_event) => {
                        Ok(MouseButton(
                            MouseButtonEvent {
                                button: button_event.button(),
                                down: match button_event.button_state() {
                                    event::tablet_pad::ButtonState::Pressed => true,
                                    event::tablet_pad::ButtonState::Released => false,
                                },
                                time: button_event.time_usec()
                            }
                        ))
                    },
                    _ => Err(()),
                }
            },
            _ => Err(()),
        }
    }
}

/// Extra information behind a keyboard button being 
/// pressed.
#[derive(Debug)]
pub struct KeyboardEvent {
    /// The key as a `u32`.
    /// 
    /// In the future, this may need to be further 
    /// abstracted to unify with other input and 
    /// output methods.
    pub key: u32,
    /// Whether or not the key is down. Didn't feel
    /// that it was worth making an enum, but this 
    /// could change in the future.
    down: bool,
    /// The time in microseconds that the button was pressed.
    time: u64
}

/// Extra information behind a keyboard button being 
/// pressed. 
/// 
/// May be changed to `MouseAction` in the future to 
/// account for scrolling.
#[derive(Debug)]
pub struct MouseButtonEvent {
    /// The key as a `u32`.
    /// 
    /// This is very likely to become its own enum in
    /// the future, as having it as such a big number for 
    /// probably a max of 12-15 buttons seems excessive.
    pub button: u32,
    /// Whether or not the button was pressed down.
    /// 
    /// Didn't decide to make this an enum, may reconsider.
    down: bool,
    /// When the MouseButtonEvent was pressed down.
    time: u64
}

pub trait EventTime {
    /// The time in microseconds that the event took place.
    fn microseconds(&self) -> u64;
    /// The time in seconds that the event took place.
    /// 
    /// Floors the microseconds value.
    fn seconds(&self) -> u64 { self.microseconds() / 1000000 }
    /// The time in minutes that the event took place.
    /// 
    /// Floors the microseconds value.
    fn minutes(&self) -> u64 { self.microseconds() / 1000000 / 60 }
    /// The time in hours that the event took place.
    /// 
    /// Floors the microseconds value.
    fn hours(&self) -> u64 { self.microseconds() / 1000000 / 60 / 60 }
    /// The time in days that the event took place.
    /// 
    /// Floors the microseconds value.
    fn days(&self) -> u64 { self.microseconds() / 1000000 / 60 / 60 / 24 }
}

impl EventTime for KeyboardEvent {
    fn microseconds(&self) -> u64 {
        self.time
    }
}

impl EventTime for MouseButtonEvent {
    fn microseconds(&self) -> u64 {
        self.time
    }
}

impl EventTime for InputEvent {
    fn microseconds(&self) -> u64 {
        match self {
            InputEvent::Keyboard(kb) => kb.time,
            InputEvent::MouseButton(ms) => ms.time,
        }
    }
}

pub trait EventUpDown {
    /// Is the key/button down?
    fn down(&self) -> bool;
    /// Is the key/button up?
    fn up(&self) -> bool {!self.down()}
}

impl EventUpDown for MouseButtonEvent {
    fn down(&self) -> bool {
        self.down
    }
}

impl EventUpDown for KeyboardEvent {
    fn down(&self) -> bool {
        self.down
    }
}