use std::{sync::mpsc::channel, thread, time::Duration};

use snarvei::{listen, Event, EventUpDown};

fn main() {
    let (tx, rx) = channel::<Event>();
    thread::spawn(|| listen(tx));
    thread::sleep(Duration::from_millis(10));

    for event in rx {
        println!("{event:?}");
        if let Event::Keyboard(kb) = event {
            if kb.key == 46 && kb.up() {
                println!("whoa");
            }
        }
    }
}
