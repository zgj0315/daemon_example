use std::{env, fs, process::Command};

use dev_util::log::log_init;
use tokio::sync::mpsc::{channel, Sender};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log_init();
    log::info!("daemon start");
    let current_dir = env::current_dir().unwrap();
    for entry in fs::read_dir(current_dir).unwrap() {
        let path = entry.unwrap().path();
        let metadata = fs::metadata(&path).unwrap();
        log::info!("path: {:?}, premissons: {:?}", path, metadata.permissions());
    }
    let (tx, mut rx) = channel(1);
    let cmds = [
        "target/debug/app_a",
        "target/debug/app_b",
        // "target/debug/app_c",
    ];

    for cmd in cmds {
        let tx = tx.clone();
        tokio::spawn(async move {
            app(cmd, tx).await;
        });
    }
    drop(tx);
    log::info!("daemon is working");
    let _ = rx.recv().await;
    log::info!("daemon end");
    Ok(())
}

async fn app(cmd: &str, _tx: Sender<&str>) {
    loop {
        log::info!("app {} start", cmd);
        let mut app_child = Command::new(cmd).spawn().expect("failed to execute");
        log::info!("app process id: {}", &app_child.id());
        app_child.wait().expect("command wasn't running");
        log::warn!("app {} end", cmd);
    }
}
