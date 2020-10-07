//! Widgets are bits of functionality for composing bars.

use chrono::prelude::*;
use penrose::{
    draw::{bar::widgets::Text, DrawContext, TextStyle, Widget},
    hooks::Hook,
    Result,
};

/// A good balance between ISO compliance and readability, eg: 2001-07-08 00:34
pub const DEFAULT_TIME_FORMAT: &str = "%Y-%m-%d %H:%M";

/// ISO-8601 compliant format strings, eg: 2001-07-08T00:34:60.026490+09:30
pub const ISO8601_TIME_FORMAT: &str = "%+";

/// Clock is exactly what the name suggests; it's a clock for your bar.
pub struct Clock<'a> {
    time_format: &'a str,
    // required because `Text` doesn't have a getter :-/  (TODO: submit a PR to add one)
    displayed_time: String,
    inner_text: Text,
}

impl<'a> Clock<'a> {
    /// Time may be an illusion, but clocks are not.  Don't believe me?  Call this and find out.
    pub fn new(time_format: &'a str, style: &TextStyle) -> Clock<'a> {
        let inner_text = Text::new("".to_string(), style, false, false);
        let mut result = Clock { time_format, displayed_time: "".to_string(), inner_text };
        // result.displayed_time = result.time_string();
        result.update_time();
        result
    }

    fn update_time(&mut self) {
        self.displayed_time = self.time_string();
        // TODO: after penrose 0.1.12 drops, refactor this to avoid the clone
        self.inner_text.set_text(self.displayed_time.clone());
    }

    fn time_string(&self) -> String {
        Local::now().format(self.time_format).to_string()
    }
}

impl<'a> Widget for Clock<'a> {
    fn draw(
        &mut self,
        ctx: &mut dyn DrawContext,
        screen: usize,
        screen_has_focus: bool,
        w: f64,
        h: f64,
    ) -> Result<()> {
        self.update_time();
        // TODO: after penrose 0.1.12 drops, remove this line
        self.inner_text.current_extent(ctx, h); // HACK: this is needed due to a bug in Text::draw
        self.inner_text.draw(ctx, screen, screen_has_focus, w, h)
    }

    fn current_extent(&mut self, ctx: &mut dyn DrawContext, h: f64) -> Result<(f64, f64)> {
        self.inner_text.current_extent(ctx, h)
    }

    fn require_draw(&self) -> bool {
        self.time_string() != self.displayed_time
    }

    fn is_greedy(&self) -> bool {
        false
    }
}

impl<'a> Hook for Clock<'a> {
    // Hook provides various callbacks that Penrose triggers, we can override any of them.
    // For a complete list of the available callbacks, see:
    // https://docs.rs/penrose/0.1.11/penrose/core/hooks/trait.Hook.html
}
