//! A collection of status / info / title bars, things the cat may want to show all the time.

use penrose::{
    draw::{
        bar::statusbar::Position, ActiveWindowName, Color, CurrentLayout, Draw, DrawContext,
        StatusBar, TextStyle, Workspaces,
    },
    Result,
};

use super::widgets::*;

// TODO: create a configuration type here

/// Create a default AwesomeWM style status bar that displays content pulled from the
/// WM_NAME property of the root window.
pub fn awesome_bar<Ctx: DrawContext>(
    drw: Box<dyn Draw<Ctx = Ctx>>,
    height: usize,
    style: &TextStyle,
    highlight: impl Into<Color>,
    empty_ws: impl Into<Color>,
    workspaces: &[&str],
) -> Result<StatusBar<Ctx>> {
    let highlight = highlight.into();

    Ok(StatusBar::try_new(
        drw,
        Position::Top,
        height,
        style.bg.unwrap_or_else(|| 0x000000.into()),
        &[&style.font],
        vec![
            Box::new(Workspaces::new(workspaces, style, highlight, empty_ws)),
            Box::new(ActiveWindowName::new(
                &TextStyle { bg: Some(highlight), padding: (6.0, 4.0), ..style.clone() },
                160,
                true,
                false,
            )),
            Box::new(CurrentLayout::new(style)),
            Box::new(Clock::new(
                // ISO8601_TIME_FORMAT,
                DEFAULT_TIME_FORMAT,
                &TextStyle { bg: Some(highlight), padding: (6.0, 4.0), ..style.clone() },
            )),
        ],
    )?)
}
