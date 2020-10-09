//! Layouts are all of the fun and exciting ways the cat may barf windows onto your screen(s).
//!

use penrose::layout::{Layout, LayoutConf};

mod fair;
mod horizontal_central_main;
mod utils;

/// This is the window aspect ratio that the tiling algorithm is trying to approximate.
pub const TARGET_ASPECT_RATIO: f32 = 16.0 / 9.0;

/// This layout has one large central main area, with peripheral windows tiled in an aspect ratio
/// aware way.
pub fn make_horizontal_central_main_layout() -> Layout {
    // TODO: should these constants be parameters?
    Layout::new("[focus]", LayoutConf::default(), horizontal_central_main::new(), 2, 0.5)
}

/// This layout tries to give all windows equal real estate (in an aspect ratio aware way).
pub fn make_fair_layout() -> Layout {
    Layout::new("[fair]", LayoutConf::default(), fair::new(), 1, 0.5)
}
