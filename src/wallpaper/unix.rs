use color_eyre::Report;
use std::process::Command;
use std::process::Stdio;

// use crate::reload::unix::reload_app;

use execute::Execute;

#[cfg(any(target_os = "linux", target_os = "netbsd"))]
pub fn set(
    path: &String,
    wallpaper_cmd: &String,
    pre_hook: &Option<String>,
    arguments: &Option<Vec<String>>,
) -> Result<(), Report> {
    if let Some(hook) = pre_hook {
      spawn_hook(&hook)?//.unwrap();
    }
    info!("Setting wallpaper...");

    // match &wallpaper_tool {
    //     WallpaperTool::Swaybg => set_wallaper_swaybg(path),
    //     WallpaperTool::Swww => set_wallaper_swww(path, swww_options),
    //     WallpaperTool::Nitrogen => set_wallaper_nitrogen(path),
    //     WallpaperTool::Feh => set_wallaper_feh(path, feh_options),
    // }
    let mut binding = Command::new(wallpaper_cmd);
    let cmd = binding.stdout(Stdio::null()).stderr(Stdio::null());
    if let Some(args) = arguments {
      cmd.args(args);
    }
    cmd.arg(path);


    match cmd.spawn() {
        Ok(_) => info!("Successfully set the wallpaper with <blue>{wallpaper_cmd}</>"),
        Err(e) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                error!("Failed to set wallpaper, the program <red>{wallpaper_cmd}</> was not found in PATH!")
            } else {
                error!("Some error(s) occured while setting wallpaper!");
            }
        }
    };
    Ok(())
}



#[cfg(any(target_os = "linux", target_os = "netbsd"))]
fn spawn_hook(hook: &String) -> Result<(), Report> {
    let mut command = execute::shell(hook);

    command.stdout(Stdio::inherit());

    let output = command.execute_output()?;

    if let Some(exit_code) = output.status.code() {
        if exit_code != 0 {
            error!("Failed executing command: {:?}", hook)
        }
    } else {
        eprintln!("Interrupted!");
    }

    Ok(())
}
// #[cfg(any(target_os = "linux", target_os = "netbsd"))]
// fn set_wallaper_swaybg(path: &String) -> Result<(), Report> {
//     reload_app("swaybg", "SIGUSR1")?;
//
//     let mut binding = Command::new("swaybg");
//     let cmd = binding.stdout(Stdio::null()).stderr(Stdio::null());
//     cmd.arg("-i");
//     cmd.arg(path);
//
//     match cmd.spawn() {
//         Ok(_) => info!("Successfully set the wallpaper with <blue>swaybg</>"),
//         Err(e) => {
//             if let std::io::ErrorKind::NotFound = e.kind() {
//                 error!("Failed to set wallpaper, the program <red>swaybg</> was not found in PATH!")
//             } else {
//                 error!("Some error(s) occured while setting wallpaper!");
//             }
//         }
//     };
//     Ok(())
// }
//
// #[cfg(any(target_os = "linux", target_os = "netbsd"))]
// fn set_wallaper_swww(path: &String, swww_options: &Option<Vec<String>>) -> Result<(), Report> {
//     let mut binding = Command::new("swww");
//     let cmd = binding.stdout(Stdio::null()).stderr(Stdio::null());
//     cmd.arg("img");
//     cmd.arg(path);
//
//     if let Some(options) = &swww_options {
//         if !options[0].is_empty() {
//             cmd.args(options);
//         }
//     }
//
//     match cmd.spawn() {
//         Ok(_) => info!("Successfully set the wallpaper with <blue>swww</>"),
//         Err(e) => {
//             if let std::io::ErrorKind::NotFound = e.kind() {
//                 error!("Failed to set wallpaper, the program <red>swww</> was not found in PATH!")
//             } else {
//                 error!("Some error(s) occured while setting wallpaper!");
//             }
//         }
//     };
//     Ok(())
// }
//
// #[cfg(any(target_os = "linux", target_os = "netbsd"))]
// fn set_wallaper_nitrogen(path: &String) -> Result<(), Report> {
//     let mut binding = Command::new("nitrogen");
//     let cmd = binding.stdout(Stdio::null()).stderr(Stdio::null());
//     cmd.arg(path);
//
//     match cmd.spawn() {
//         Ok(_) => info!("Successfully set the wallpaper with <blue>nitrogen</>"),
//         Err(e) => {
//             if let std::io::ErrorKind::NotFound = e.kind() {
//                 error!(
//                     "Failed to set wallpaper, the program <red>nitrogen</> was not found in PATH!"
//                 )
//             } else {
//                 error!("Some error(s) occured while setting wallpaper!");
//             }
//         }
//     };
//     Ok(())
// }
//
// #[cfg(any(target_os = "linux", target_os = "netbsd"))]
// fn set_wallaper_feh(path: &String, feh_options: &Option<Vec<String>>) -> Result<(), Report> {
//     let mut binding = Command::new("feh");
//     let cmd = binding.stdout(Stdio::null()).stderr(Stdio::null());
//
//     if let Some(options) = &feh_options {
//         if !options[0].is_empty() {
//             cmd.args(options);
//         } else {
//             cmd.arg("--bg-scale");
//         }
//     }
//
//     cmd.arg(path);
//
//     match cmd.spawn() {
//         Ok(_) => info!("Successfully set the wallpaper with <blue>feh</>"),
//         Err(e) => {
//             if let std::io::ErrorKind::NotFound = e.kind() {
//                 error!("Failed to set wallpaper, the program <red>feh</> was not found in PATH!")
//             } else {
//                 error!("Some error(s) occured while setting wallpaper!");
//             }
//         }
//     };
//     Ok(())
// }
