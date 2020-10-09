//! The fair window layout is one that attempts to give all windows equal space.
//!
//! It is not always possible to give all windows equal space and tile the available
//! area, so there are two variants of the fair layout: horizontal and vertical.
//! Vertical fair layouts put the lucky windows above one another, and horizontal
//! fair layouts put them beside one another.
//!

use penrose::{
    client::Client,
    data_types::{Region, ResizeAction},
    layout::LayoutFunc,
};

use super::utils;

/// Creates and returns a closure that performs layouts.
pub fn new() -> LayoutFunc {
    |a, _, c, _, _| do_fair_layout(a, c)
}

fn do_fair_layout(clients: &[&Client], to_fill: &Region) -> Vec<ResizeAction> {
    let layout = layout_region_fairly(to_fill, clients.len() as u32);
    clients.iter().zip(layout).map(|(c, r)| (c.id(), Some(r))).collect()
}

/// Divides the region into approximately even regions that are each as close as possible to 16:9.
pub fn layout_region_fairly(to_fill: &Region, window_count: u32) -> Vec<Region> {
    (0..window_count)
        .map(|c| layout_region_in_rows(to_fill, window_count, c + 1))
        .min_by_key(utils::aspect_ratio_sse)
        .unwrap_or(vec![*to_fill])
}

fn layout_region_in_rows(to_fill: &Region, window_count: u32, full_row_count: u32) -> Vec<Region> {
    let col_count = window_count / full_row_count;
    let top_count = window_count - col_count * full_row_count;
    let row_count = if top_count == 0 { full_row_count } else { full_row_count + 1 };

    let mut results = Vec::new();

    let mut rows = utils::split_into_rows(to_fill, row_count);

    if top_count > 0 {
        let row = rows.remove(0);
        results.append(&mut utils::split_into_columns(&row, top_count));
    }
    for row in rows {
        results.append(&mut utils::split_into_columns(&row, col_count));
    }

    results
}
