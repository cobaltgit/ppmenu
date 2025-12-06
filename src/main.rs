use anyhow::Result;
use argh::FromArgs;
use notify_rust::Notification;
use zbus::blocking::Connection;

mod menu;
use menu::Menu;

mod powerprofile;
use powerprofile::PowerProfile;

#[derive(FromArgs)]
/// Power profile selector for power-profiles-daemon (or tlp-pd) using a dmenu-compatible launcher
struct PPPArgs {
    /// launcher to use (supported launchers are fuzzel, rofi, walker, dmenu, bemenu, wofi, tofi)
    #[argh(option, short = 'l')]
    launcher: String,

    /// custom arguments to pass to the launcher
    #[argh(option, short = 'a')]
    launcher_args: Option<String>,
}


fn main() -> Result<()> {
    let connection = Connection::system()?;
    let current_profile = PowerProfile::get_active(&connection)?;
    let placeholder = format!("Current profile: {}", current_profile.name());
    
    let menus: Vec<Menu> = vec![
        Menu { name: String::from("fuzzel"), args: format!("--dmenu --index --placeholder \"{}\"", placeholder), use_index: true },
        Menu { name: String::from("rofi"), args: format!("-dmenu -i -p \"{}\"", placeholder), use_index: true },
        Menu { name: String::from("walker"), args: format!("-d -i -p \"{}\"", placeholder), use_index: true },
        Menu { name: String::from("dmenu"), args: format!("-p \"{}\"", placeholder), use_index: false },
        Menu { name: String::from("bemenu"), args: format!("-p \"{}\"", placeholder), use_index: false },
        Menu { name: String::from("wofi"), args: format!("--show=dmenu -p \"{}\"", placeholder), use_index: false },
        Menu { name: String::from("tofi"), args: format!("-p \"{}\"", placeholder), use_index: false },
    ];

    let args: PPPArgs = argh::from_env();

    let all_menu_names: Vec<String> = menus.iter().map(|m| m.name.clone()).collect();
    if !all_menu_names.contains(&args.launcher) {
        anyhow::bail!(
            "Invalid launcher '{}'. Must be one of: {}",
            args.launcher,
            all_menu_names.join(", ")
        )
    }

    let menu = menus.iter().find(|m| m.name == args.launcher).expect("Could not find menu");

    let new_profile = menu.get_profile(args.launcher_args.as_deref())?;

    if new_profile == current_profile {
        Notification::new()
            .summary("Power Profile Picker")
            .body(&format!("Power profile is already set to {}", current_profile.name()))
            .show()?;
        return Ok(());
    }

    match new_profile.apply(&connection) {
        Ok(()) => {
            Notification::new()
                .summary("Power Profile Picker")
                .body(&format!("Power profile set to {}", new_profile.name()))
                .show()?;
        },
        Err(e) => {
            Notification::new()
                .summary("Power Profile Picker")
                .body(&format!("Unable to set power profile: {:?}", e))
                .show()?;
        }
    }
     
    Ok(())
}
