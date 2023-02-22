use std::{thread::sleep, time::Duration};

use dev_util::log::log_init;

fn main() {
    log_init();
    log::info!("app a start");
    loop {
        log::info!("app a running");
        sleep(Duration::from_secs(3));
    }
}
