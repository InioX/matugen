use std::process::Command;

#[cfg(target_os = "linux")]
pub fn reload_apps_linux() {
    restart("kitty", "SIGUSR1");
    restart("waybar", "SIGUSR2");
}

fn restart(name: &str, signal: &str) {
    info!("Restarting {}", name);
    let mut kill = Command::new("pkill");
    kill.arg(format!("-{}", signal));
    kill.arg(name);

    kill.spawn();
}
