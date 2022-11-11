use std::{sync::mpsc::channel, thread, time::Duration};

use crate::{listen, watch_config, Event};

pub fn start_loop() -> ! {
    let (tx, rx) = channel::<Event>();
    let (input_tx, config_tx) = (tx.clone(), tx.clone());
    thread::spawn(move || listen(input_tx));
    watch_config(config_tx).expect("couldn't watch config");
    thread::sleep(Duration::from_millis(10));

    println!("listening");
    for event in rx {
        dbg!(event);
    }
    panic!("ran out of events");
}