use std::{sync::mpsc::channel, thread};

use snarvei::{listen, Event};

fn main() {
    let (tx, rx) = channel::<Event>();
    thread::spawn(|| listen(tx));

    for event in rx {
        println!("{event:?}")
    }
}
