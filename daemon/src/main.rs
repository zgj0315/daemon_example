use std::{process::Command, thread::sleep, time::Duration};

use dev_util::log::log_init;

fn main() {
    log_init();
    log::info!("daemon start");
    loop {
        log::info!("app a start");
        let mut app_a_child = Command::new("target/debug/app_a")
            .spawn()
            .expect("failed to execute");
        log::info!("app a process id: {}", &app_a_child.id());
        let mut app_b_child = Command::new("target/debug/app_b")
            .spawn()
            .expect("failed to execute");
        log::info!("app b process id: {}", &app_b_child.id());
        app_a_child.wait().expect("command wasn't running");
        log::info!("app a end");
        app_b_child.wait().expect("command wasn't running");
        log::info!("app b end");
        sleep(Duration::from_secs(1));
    }
}
