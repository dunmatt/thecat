//! This module houses configuration structs for external widgets.

use penrose::draw::{Color, TextStyle};

use crate::{widgets::DEFAULT_TEXT_STYLE, DEFAULT_WORKSPACE_NAMES};

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
            highlight: Color::from_rgb(0x45, 0x85, 0x88),
            empty: Color::from_rgb(0x3C, 0x38, 0x36),
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
