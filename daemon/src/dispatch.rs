use std::{sync::mpsc::channel, thread, time::Duration};

use config::CONFIG_PATH;
use log::{info, debug};
use notify::{recommended_watcher, Watcher, RecursiveMode};

use crate::{listen, handle_nevent, Event};

pub fn start_loop() -> ! {
    let (tx, rx) = channel::<Event>();
    let (input_tx, config_tx) = (tx.clone(), tx.clone());
    
    // listen for libinput
    thread::spawn(move || listen(input_tx));
    thread::sleep(Duration::from_millis(10));
    
    // listen for config updates
    let mut watcher = recommended_watcher(move |res: notify::Result<notify::Event>| {
        handle_nevent(res.unwrap(), &config_tx)
    }).unwrap();
    watcher.watch(CONFIG_PATH.join("config.toml").as_path(), RecursiveMode::NonRecursive).unwrap();

    info!("Listening for events...");
    for event in rx {
        debug!("Got an event...");
    }
    panic!("ran out of events");
}