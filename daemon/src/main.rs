use std::{sync::mpsc::channel, thread, time::Duration};

use snarvei_daemon::{listen, Event, KEY_DICT};

fn main() {
    let (tx, rx) = channel::<Event>();
    thread::spawn(|| listen(tx));
    thread::sleep(Duration::from_millis(10));

    for event in rx {
        if let Event::Keyboard(kb) = event {
            if let Some(name) = KEY_DICT.get(&(kb.key as u16)) {
                println!("AAAAAAAAAAAAAAAAAAA {name}")
            }
        }
    }
}