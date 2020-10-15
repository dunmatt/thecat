//! WindowList is a widget that lists the windows of the given workspace in order, and highlights
//! the one that currently has focus.  It also deliniates windows in the main group from the
//! others.  The overall idea is to make layout behaviors more discoverable.
//!

use std::cmp::min;

use penrose::{
    core::{ring::Selector, Client},
    data_types::{Region, WinId},
    draw::{Color, DrawContext, Text, TextStyle, Widget},
    hooks::Hook,
    Result, WindowManager,
};

use crate::widgets::DEFAULT_TEXT_STYLE;

/// The default style used for whichever window has focus.
pub const DEFAULT_FOCUSED_TEXT_STYLE: TextStyle =
    TextStyle { bg: Some(Color::from_rgb(0x3C, 0x38, 0x36)), ..DEFAULT_TEXT_STYLE };

/// The default style used for windows that lack focus.
pub const DEFAULT_BACKGROUND_TEXT_STYLE: TextStyle =
    TextStyle { fg: Color::from_rgb(147, 137, 116), ..DEFAULT_TEXT_STYLE };

macro_rules! text_box_iter {
    ($self:ident) => {
        $self.text_boxes.iter_mut().map(|t| &mut t.1)
    };
}

/// WindowList is the widget itself, albe to be put into a bar to list all of your windows.
pub struct WindowList {
    text_boxes: Vec<(WinId, Text)>,
    separator: String,
    separator_color: Option<Color>,
    highlight: TextStyle<'static>,
    style: TextStyle<'static>,
}

/// All of the settings afforded by WindowLists.
#[derive(Clone, Debug)]
pub struct Configuration<'a> {
    /// This string separates windows in the main group from the others.
    pub separator: &'a str,
    /// The color for the separator.  None means "do not display".
    pub separator_color: Option<Color>,
    /// The font and color information to use for the active window.
    pub highlight: &'a TextStyle<'static>,
    /// The font and color information to use for the text.
    pub style: &'a TextStyle<'static>,
}

impl WindowList {
    /// Creates a WindowList for use in a bar.
    pub fn new(config: &Configuration) -> WindowList {
        WindowList {
            text_boxes: Vec::new(),
            separator: config.separator.to_string(),
            separator_color: config.separator_color,
            highlight: config.highlight.clone(),
            style: config.style.clone(),
        }
    }
}

impl WindowList {
    fn repopulate(&mut self, wm: &mut WindowManager, workspace_index: usize) {
        self.text_boxes.clear();

        if let Some(workspace) = wm.workspace(&Selector::Index(workspace_index)) {
            let focused_id = workspace.focused_client();

            for id in workspace.iter() {
                if let Some(client) = wm.client(&Selector::WinId(*id)) {
                    let name = client.wm_name().to_string();
                    let style = if Some(*id) == focused_id { &self.highlight } else { &self.style };
                    let text = Text::new(name, style, true, false);
                    self.text_boxes.push((*id, text));
                }
            }
            // Insert the separator.
            if let Some(color) = self.separator_color {
                let mut sep = Text::new(self.separator.to_string(), &self.style, false, false);
                sep.set_fg(color);
                let i = min(workspace.get_max_main() as usize, self.text_boxes.len());
                self.text_boxes.insert(i, (u32::MAX, sep));
            }
        }
    }
}

impl Widget for WindowList {
    fn draw(
        &mut self,
        ctx: &mut dyn DrawContext,
        screen: usize,
        screen_has_focus: bool,
        w: f64,
        h: f64,
    ) -> Result<()> {
        let mut greedy_count = 0.0;
        let mut polite_width = 0.0;

        for text in text_box_iter!(self) {
            if text.is_greedy() {
                greedy_count += 1.0;
            } else {
                polite_width += text.current_extent(ctx, h)?.0;
            }
        }

        let greedy_width = (w - polite_width) / greedy_count;
        let mut x = ctx.get_x_offset();

        for text in text_box_iter!(self) {
            // TODO: figure out how it is possible for this to be drawing over the clock
            text.draw(ctx, screen, screen_has_focus, greedy_width, h)?;
            if text.is_greedy() {
                x += greedy_width;
            } else {
                x += text.current_extent(ctx, h)?.0;
            }
            ctx.set_x_offset(x);
        }
        ctx.flush();
        Ok(())
    }

    fn current_extent(&mut self, ctx: &mut dyn DrawContext, h: f64) -> Result<(f64, f64)> {
        Ok((
            text_box_iter!(self)
                .filter_map(|t| t.current_extent(ctx, h).map(|p| p.0).ok())
                .sum::<f64>(),
            h,
        ))
    }

    fn require_draw(&self) -> bool {
        self.text_boxes.iter().any(|t| t.1.require_draw())
    }

    fn is_greedy(&self) -> bool {
        true
    }
}

macro_rules! pass_to_widgets {
    ($self:ident, $($toks:tt)*) => {
        crate::pass_through_method_to!($($toks)* |=> text_box_iter!($self));
    };
}

impl Hook for WindowList {
    pass_to_widgets!(self, fn new_client(&mut self, wm: &mut WindowManager<'_>, c: &mut Client));
    pass_to_widgets!(self, fn remove_client(&mut self, wm: &mut WindowManager<'_>, id: WinId));

    fn client_name_updated(
        &mut self,
        wm: &mut WindowManager<'_>,
        id: WinId,
        name: &str,
        is_root: bool,
    ) {
        for (window_id, text_box) in self.text_boxes.iter_mut() {
            if *window_id == id {
                text_box.set_text(name);
            }
            text_box.client_name_updated(wm, id, name, is_root);
        }
    }

    fn layout_applied(
        &mut self,
        wm: &mut WindowManager<'_>,
        workspace_index: usize,
        screen_index: usize,
    ) {
        self.repopulate(wm, workspace_index);

        for text in text_box_iter!(self) {
            text.layout_applied(wm, workspace_index, screen_index);
        }
    }

    pass_to_widgets!(self, fn layout_change(&mut self, wm: &mut WindowManager<'_>, workspace_index: usize, screen_index: usize ));

    fn workspace_change(
        &mut self,
        wm: &mut WindowManager<'_>,
        previous_workspace: usize,
        new_workspace: usize,
    ) {
        self.repopulate(wm, new_workspace);

        for text in text_box_iter!(self) {
            text.workspace_change(wm, previous_workspace, new_workspace);
        }
    }

    pass_to_widgets!(self, fn workspaces_updated(&mut self, wm: &mut WindowManager<'_>, names: &[&str], active: usize ));
    pass_to_widgets!(self, fn screen_change(&mut self, wm: &mut WindowManager<'_>, screen_index: usize));
    pass_to_widgets!(self, fn screens_updated(&mut self, wm: &mut WindowManager<'_>, dimensions: &[Region] ));

    fn focus_change(&mut self, wm: &mut WindowManager<'_>, id: WinId) {
        for (window_id, text_box) in self.text_boxes.iter_mut() {
            if text_box.is_greedy() {
                if *window_id == id {
                    text_box.set_fg(self.highlight.fg);
                    text_box.set_bg(self.highlight.bg);
                } else {
                    text_box.set_fg(self.style.fg);
                    text_box.set_bg(self.style.bg);
                }
            }
            text_box.focus_change(wm, id);
        }
    }

    pass_to_widgets!(self, fn event_handled(&mut self, wm: &mut WindowManager<'_>));
    pass_to_widgets!(self, fn startup(&mut self, wm: &mut WindowManager<'_>));
}

impl<'a> Default for Configuration<'a> {
    fn default() -> Configuration<'a> {
        Configuration {
            separator: " | ",
            separator_color: None, // Some(Color::from_rgb(0x45, 0x85, 0x88)),
            highlight: &DEFAULT_FOCUSED_TEXT_STYLE,
            style: &DEFAULT_BACKGROUND_TEXT_STYLE,
        }
    }
}
