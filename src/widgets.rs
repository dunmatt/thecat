//! Widgets are bits of functionality for composing bars.

use penrose::draw::{Color, TextStyle};

/// The default text style for widgets.
pub const DEFAULT_TEXT_STYLE: TextStyle = TextStyle {
    font: "ProFont For Powerline",
    point_size: 11,
    fg: Color::from_rgb(0xEB, 0xDB, 0xB2),
    bg: Some(Color::from_rgb(0x28, 0x28, 0x28)),
    padding: (2.0, 2.0),
};

pub mod clock;
pub mod configurations;
pub mod window_list;
