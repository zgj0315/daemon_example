use std::{thread::sleep, time::Duration};

use dev_util::log::log_init;

fn main() {
    log_init();
    log::info!("app b start");
    loop {
        log::info!("app b running");
        sleep(Duration::from_secs(1));
    }
}
