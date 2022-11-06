use std::time::Instant;

use snarvei_config::{Config, write_config};

fn main() {
    let start = Instant::now();
    write_config(
        Config::new()
    );
    println!("Finished in {:?}", start.elapsed());
}
