use std::{env, process::Command};

use dev_util::log::log_init;
use tokio::sync::mpsc::{channel, Sender};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log_init();
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        log::error!("input error, eg: ./daemon ./app_a ./app_b");
        return Ok(());
    }
    log::info!("daemon start");
    let (tx, mut rx) = channel(1);
    'args_for: for (i, cmd) in args.iter().enumerate() {
        if i == 0 {
            continue 'args_for;
        }
        let cmd = cmd.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            app(&cmd, tx).await;
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
