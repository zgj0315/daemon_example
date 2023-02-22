use std::process::Command;

use dev_util::log::log_init;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log_init();
    log::info!("daemon start");
    let cmds = [
        "target/debug/app_a",
        "target/debug/app_b",
        "target/debug/app_c",
    ];
    let mut handles = Vec::new();
    for cmd in cmds {
        let handle = tokio::spawn(async move {
            app(cmd).await;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await?;
    }
    Ok(())
}

async fn app(cmd: &str) {
    loop {
        log::info!("app {} start", cmd);
        let mut app_child = Command::new(cmd).spawn().expect("failed to execute");
        log::info!("app process id: {}", &app_child.id());
        app_child.wait().expect("command wasn't running");
        log::warn!("app {} end", cmd);
    }
}
