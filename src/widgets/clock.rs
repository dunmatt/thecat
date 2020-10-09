//! A simple, customizable clock widget.

use chrono::prelude::*;
use penrose::{
    draw::{bar::widgets::Text, DrawContext, TextStyle, Widget},
    hooks::Hook,
    Result,
};

use crate::widgets::DEFAULT_TEXT_STYLE;

/// A good balance between ISO compliance and readability, eg: 2001-07-08 00:34
pub const DEFAULT_TIME_FORMAT: &str = "%Y-%m-%d %H:%M";

/// ISO-8601 compliant format strings, eg: 2001-07-08T00:34:60.026490+09:30
pub const ISO8601_TIME_FORMAT: &str = "%+";

/// Clock is exactly what the name suggests; it's a clock for your bar.
pub struct Clock {
    time_format: String,
    inner_text: Text,
}

/// All of the various settings afforded by this clock.
#[derive(Clone, Debug)]
pub struct Configuration<'a> {
    /// The format string (in `chrono` format) you'd like the time displayed in.
    pub time_format: &'a str,
    /// The font and color information to use for the text.
    pub style: &'a TextStyle<'a>,
    /// Should this widget occupy as much space as possible?
    pub greedy: bool,
    /// Should this widget alight to the right side of the screen?
    pub right_justified: bool,
}

impl Clock {
    /// Time may be an illusion, but clocks are not.  Don't believe me?  Call this and find out.
    pub fn new(config: &Configuration) -> Clock {
        let inner_text =
            Text::new("".to_string(), config.style, config.greedy, config.right_justified);
        let mut result = Clock { time_format: config.time_format.to_string(), inner_text };
        result.update_time();
        result
    }

    fn update_time(&mut self) {
        self.inner_text.set_text(self.time_string());
    }

    fn time_string(&self) -> String {
        Local::now().format(&self.time_format).to_string()
    }
}

impl Widget for Clock {
    fn draw(
        &mut self,
        ctx: &mut dyn DrawContext,
        screen: usize,
        screen_has_focus: bool,
        w: f64,
        h: f64,
    ) -> Result<()> {
        self.update_time();
        self.inner_text.draw(ctx, screen, screen_has_focus, w, h)
    }

    fn current_extent(&mut self, ctx: &mut dyn DrawContext, h: f64) -> Result<(f64, f64)> {
        self.inner_text.current_extent(ctx, h)
    }

    fn require_draw(&self) -> bool {
        self.time_string() != *self.inner_text.get_text()
    }

    fn is_greedy(&self) -> bool {
        false
    }
}

impl Hook for Clock {
    // Hook provides various callbacks that Penrose triggers, we can override any of them.
    // For a complete list of the available callbacks, see:
    // https://docs.rs/penrose/0.1.11/penrose/core/hooks/trait.Hook.html
}

impl<'a> Default for Configuration<'a> {
    fn default() -> Configuration<'a> {
        Configuration {
            time_format: DEFAULT_TIME_FORMAT,
            style: &DEFAULT_TEXT_STYLE,
            greedy: false,
            right_justified: false,
        }
    }
}
