use std::{sync::mpsc::channel, thread, time::Duration};
mod libinput;
use libinput::{listen, Event};
use keycodes::KEY;

fn main() {
    let (tx, rx) = channel::<Event>();
    thread::spawn(|| listen(tx));
    thread::sleep(Duration::from_millis(10));

    for event in rx {
        if let Event::Keyboard(kb) = event {
            if let Ok(key) = KEY::try_from(kb.key as u16) {
                println!("AAAAAAAAAAAAAAAAAAA {}", String::from(key))
            }
        }
    }
}