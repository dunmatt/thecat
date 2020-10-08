/// This module houses configuration structs for external widgets.
use penrose::draw::{Color, TextStyle};

/// The default workspace names.
pub const DEFAULT_WORKSPACE_NAMES: &[&str] = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];
/// The default text style for external widgets.
pub const DEFAULT_TEXT_STYLE: TextStyle = TextStyle {
    font: "ProFont For Powerline".to_string(),
    point_size: 11,
    fg: 0xEBDB_B2FF.into(),
    bg: Some(0x2828_28FF.into()),
    padding: (2.0, 2.0),
};

/// All of the settings afforded by a penrose::draw::Workspaces widget.
#[derive(Clone, Debug)]
pub struct WorkspacesConfiguration<'a> {
    /// The names to display for each workspace.
    pub names: &'a [&'a str],
    /// The font and color information to use for the text.
    pub style: &'a TextStyle,
    /// The background color to use for the current workspace.
    pub highlight: Color,
    /// The text color to use for the names of empty workspaces.
    pub empty: Color,
}

impl<'a> Default for WorkspacesConfiguration<'a> {
    fn default() -> WorkspacesConfiguration<'a> {
        WorkspacesConfiguration {
            names: DEFAULT_WORKSPACE_NAMES,
            style: &DEFAULT_TEXT_STYLE,
            highlight: 0x4585_88FF.into(),
            empty: 0x3C38_36FF.into(),
        }
    }
}

/// All of the settings afforded by a penrose::draw::ActiveWindowName widget.
#[derive(Clone, Debug)]
pub struct ActiveWindowNameConfiguration<'a> {
    /// The font and color information to use for the text.
    pub style: &'a TextStyle,
    /// The maximum length to display of a window's name.
    pub max_name_length: u32,
    /// Should this widget occupy as much space as possible?
    pub greedy: bool,
    /// Should this widget alight to the right side of the screen?
    pub right_justified: bool,
}

impl<'a> Default for ActiveWindowNameConfiguration<'a> {
    fn default() -> ActiveWindowNameConfiguration<'a> {
        ActiveWindowNameConfiguration {
            style: &DEFAULT_TEXT_STYLE,
            max_name_length: 80,
            greedy: true,
            right_justified: false,
        }
    }
}

/// All of the settings afforded by a penrose::draw::CurrentLayout widget.
#[derive(Clone, Debug)]
pub struct CurrentLayoutConfiguration<'a> {
    /// The font and color information to use for the text.
    pub style: &'a TextStyle,
}

impl<'a> Default for CurrentLayoutConfiguration<'a> {
    fn default() -> CurrentLayoutConfiguration<'a> {
        CurrentLayoutConfiguration { style: &DEFAULT_TEXT_STYLE }
    }
}
