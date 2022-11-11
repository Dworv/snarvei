use std::sync::mpsc::Sender;

use notify::EventKind;

use crate::Event;

#[derive(Debug)]
pub enum ConfigEvent {
    Reload(bool)
}

pub fn handle_nevent(e: notify::Event, tx: &Sender<Event>) {
    match e.kind {
        EventKind::Modify(_) => {
            tx.send(Event::Config(ConfigEvent::Reload(true))).unwrap();
        }
        _ => {}
    }
}