//! Layouts are all of the fun and exciting ways the cat may barf windows onto your screen(s).
//!

use penrose::layout::LayoutFunc;

mod horizontal_central_main;
mod utils;

/// This layout has one large central main area, with peripheral windows tiled in an aspect ratio
/// aware way.
pub fn make_horizontal_central_main_layout() -> LayoutFunc {
    horizontal_central_main::new()
}
