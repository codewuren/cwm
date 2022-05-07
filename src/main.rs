/* This is cwm.rs */

use penrose::{
    core::{
        bindings::KeyEventHandler, config::Config, helpers::index_selectors, manager::WindowManager,
    },
    logging_error_handler,
    xcb::new_xcb_backed_window_manager,
    Backward, Forward, Less, More, Selector,
};
use simplelog::{LevelFilter, SimpleLogger};

#[macro_use]
extern crate penrose;

// Main function
fn main() -> penrose::Result<()> {
    // Initialise the logger (use LevelFilter::Debug to enable debug logging)
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    };

    // Deafult configuration
    let config = Config::default();

    // Replace these with your preferred terminal and program launcher
    const terminalcmd: &str = "rofi -show run";
    const roficmd: &str = "alacritty";

    let key_bindings = gen_keybindings! {
        // Program launchers
        "M-r" => run_external!(terminalcmd);
        "M-Return" => run_external!(roficmd);

        // Exit Penrose (important to remember this one!)
        "M-S-q" => run_internal!(exit);

        // Client management
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-c" => run_internal!(kill_client);

        // Workspace management
        "M-Tab" => run_internal!(toggle_workspace);
        "M-A-period" => run_internal!(cycle_workspace, Forward);
        "M-A-comma" => run_internal!(cycle_workspace, Backward);

        // Layout management
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-A-Right" => run_internal!(update_main_ratio, More);
        "M-A-Left" => run_internal!(update_main_ratio, Less);

        map: { "1", "2", "3", "4", "5", "6", "7", "8", "9" } to index_selectors(9) => {
            "M-{}" => focus_workspace (REF);
            "M-S-{}" => client_to_workspace (REF);
        };
    };

    let mut WINDOW_MANAGER =
        new_xcb_backed_window_manager(config, vec![], logging_error_handler())?;
    WINDOW_MANAGER.grab_keys_and_run(key_bindings, map! {})
}
