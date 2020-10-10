//! A collection of status / info / title bars, things the cat may want to show all the time.

use penrose::{
    draw::{
        bar::statusbar::Position, Color, CurrentLayout, Draw, DrawContext, StatusBar, Workspaces,
    },
    Result,
};

use crate::widgets::*;

/// All of the settings afforded by awesome_bars.
#[derive(Clone, Debug)]
pub struct AwesomeBarConfiguration<'w, 'c, 'l> {
    /// How much vertical space the bar occupies at the edge of the screen.
    pub bar_height: u32,
    /// Where the bar should appear on the screen.
    pub position: Position,
    /// What color the bar's background should be, defauts to the Workspaces' background color.
    pub background: Option<Color>,
    /// The settings for the workspaces widget in the left corner of the screen.
    pub workspaces: configurations::WorkspacesConfiguration<'w>,
    /// The settings for the open windows widget.
    pub window_list: window_list::Configuration<'static>,
    /// The settings for the clock in the right corner of the screen.
    pub clock: clock::Configuration<'c>,
    /// The settings for the current layout widget in the right corner of the screen.
    pub layout: configurations::CurrentLayoutConfiguration<'l>,
}

/// Create a default AwesomeWM style status bar that displays content pulled from the
/// WM_NAME property of the root window.
pub fn awesome_bar<Ctx: DrawContext>(
    drw: Box<dyn Draw<Ctx = Ctx>>,
    config: &AwesomeBarConfiguration,
) -> Result<StatusBar<Ctx>> {
    Ok(StatusBar::try_new(
        drw,
        config.position,
        config.bar_height as usize,
        config.find_background_color(),
        &config.find_fonts(),
        vec![
            Box::new(Workspaces::new(
                config.workspaces.names,
                config.workspaces.style,
                config.workspaces.highlight,
                config.workspaces.empty,
            )),
            Box::new(window_list::WindowList::new(&config.window_list)),
            Box::new(clock::Clock::new(&config.clock)),
            Box::new(CurrentLayout::new(config.layout.style)),
        ],
    )?)
}

impl<'w, 'c, 'l> AwesomeBarConfiguration<'w, 'c, 'l> {
    fn find_background_color(&self) -> Color {
        self.background
            .or(self.workspaces.style.bg)
            .or(self.clock.style.bg)
            .or(self.layout.style.bg)
            .unwrap_or(0.into()) // black
    }

    fn find_fonts<'f>(&self) -> Vec<&'f str>
    where
        'w: 'f,
        'c: 'f,
        'l: 'f,
    {
        vec![&self.workspaces.style.font, &self.clock.style.font, &self.layout.style.font]
    }
}

impl<'w, 'c, 'l> Default for AwesomeBarConfiguration<'w, 'c, 'l> {
    fn default() -> AwesomeBarConfiguration<'w, 'c, 'l> {
        AwesomeBarConfiguration {
            bar_height: 18,
            position: Position::Top,
            background: None,
            workspaces: Default::default(),
            window_list: Default::default(),
            clock: Default::default(),
            layout: Default::default(),
        }
    }
}
