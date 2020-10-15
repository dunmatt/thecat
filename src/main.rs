#![forbid(unsafe_code)]

use std::collections::HashMap;

use penrose::{
    core::ring::Selector,
    data_types::Change::{Less, More},
    draw::XCBDraw,
    // contrib::actions::focus_or_spawn
    gen_keybindings,
    helpers::index_selectors,
    run_external,
    run_internal,
    Backward,
    Config,
    Forward,
    Result,
    WindowManager,
    XcbConnection,
};

use thecat::*;

// TODO LIST //
// TODO: switch cargo.toml back to pointing upstream once https://github.com/sminez/penrose/pull/75 lands
// TODO: commit and push
// TODO: add simple theme support
// TODO: add some sort of tracking so that windows in the main region don't get pushed out
// TODO: command line parameters for the style options, perhaps even dynamically adjustable

fn main() -> Result<()> {
    let bar_config = bars::AwesomeBarConfiguration::default();

    let mut config = Config::default();
    config.border_px = 1;
    config.gap_px = 0;
    config.bar_height = bar_config.bar_height;
    config.floating_classes = &["rofi"];

    config.hooks.push(Box::new(bars::awesome_bar(Box::new(XCBDraw::new()?), &bar_config)?));

    // -- layouts --
    config.layouts =
        vec![layouts::make_horizontal_central_main_layout(), layouts::make_fair_layout()];

    let key_bindings = gen_keybindings! {
        "M-C-f" => run_external!("firefox");
        "M-r" => run_external!("/home/matt/code/thecat/scripts/rofi-wrap");
        "M-C-s" => run_external!("subl");
        "M-C-v" => run_external!("pavucontrol");
        "M-C-y" => run_external!("yed");
        "M-Return" => run_external!("alacritty");
        "M-S-Return" => run_external!("urxvt");
        "Print" => run_external!("scrot -e 'mv $f ~/screenshots/' 2> /dev/null");

        "M-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-j" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Backward);
        "M-k" => run_internal!(cycle_client, Forward);
        "M-S-k" => run_internal!(drag_client, Forward);
        "M-space" => run_internal!(cycle_layout, Forward);
        "M-S-space" => run_internal!(cycle_layout, Backward);
        "M-S-c" => run_internal!(kill_client);
        "M-S-q" => run_internal!(exit);
        "M-bracketright" => run_internal!(cycle_screen, Forward);
        "M-bracketleft" => run_internal!(cycle_screen, Backward);
        "M-S-Up" => run_internal!(update_max_main, More);
        "M-S-Down" => run_internal!(update_max_main, Less);
        "M-S-Right" => run_internal!(update_main_ratio, More);
        "M-S-Left" => run_internal!(update_main_ratio, Less);

        refmap [ config.ws_range() ] in {
            "M-{}" => focus_workspace [ index_selectors(config.workspaces.len()) ];
            "M-S-{}" => client_to_workspace [ index_selectors(config.workspaces.len()) ];
        };
    };

    let conn = XcbConnection::new()?;
    let mut wm = WindowManager::init(config, &conn);
    wm.grab_keys_and_run(key_bindings, HashMap::new());

    Ok(())
}
