#![forbid(unsafe_code)]

use std::collections::HashMap;

use penrose::{
    data_types::Change::{Less, More},
    draw::{dwm_bar, TextStyle, XCBDraw},
    // contrib::actions::focus_or_spawn
    gen_keybindings,
    helpers::index_selectors,
    layout::{Layout, LayoutConf},
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

// TODO: make all of these command line parameters, perhaps even dynamically adjustable
const HEIGHT: usize = 18;
const PROFONT: &str = "ProFont For Powerline";
const BLACK: u32 = 0x282828ff;
const GREY: u32 = 0x3c3836ff;
const WHITE: u32 = 0xebdbb2ff;
const BLUE: u32 = 0x458588ff;

fn main() -> Result<()> {
    let mut config = Config::default();
    config.border_px = 1;
    config.gap_px = 0;
    config.bar_height = HEIGHT as u32;

    config.hooks.push(Box::new(dwm_bar(
        Box::new(XCBDraw::new()?),
        HEIGHT,
        &TextStyle {
            font: PROFONT.to_string(),
            point_size: 11,
            fg: WHITE.into(),
            bg: Some(BLACK.into()),
            padding: (2.0, 2.0),
        },
        BLUE, // highlight
        GREY, // empty_ws
        &config.workspaces,
    )?));

    // -- layouts --
    config.layouts = vec![Layout::new(
        "[focus]",
        LayoutConf::default(),
        layouts::make_horizontal_central_main_layout(),
        2,
        0.5,
    )];

    let key_bindings = gen_keybindings! {
        "M-C-f" => run_external!("firefox");
        "M-C-s" => run_external!("subl");
        "M-C-v" => run_external!("pavucontrol");
        "M-Return" => run_external!("alacritty");
        "M-S-Return" => run_external!("urxvt");

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

    //         "M-j" => run_internal!(cycle_client, Forward);
    //         "M-k" => run_internal!(cycle_client, Backward);
    //         "M-Tab" => run_internal!(toggle_workspace);
    //         "M-S-bracketright" => run_internal!(drag_workspace, Forward);
    //         "M-S-bracketleft" => run_internal!(drag_workspace, Backward);
    //         "M-grave" => run_internal!(cycle_layout, Forward);
    //         "M-S-grave" => run_internal!(cycle_layout, Backward);
    //         "M-semicolon" => run_external!("dmenu_run");

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
