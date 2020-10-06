//! The horizontal central main layout has a large central area for your most important
//! window(s) and equally sized side areas for others.
//!
//! Within each of the three areas, windows are tiled to remain approximately 4:3, with ties
//! broken in favor of (by which I mean more space given to) the first window(s) in the area.
//!

use penrose::{
    client::Client,
    data_types::{Region, ResizeAction, WinId},
    layout::LayoutFunc,
};

use super::utils;

const TARGET_ASPECT_RATIO: f32 = 4.0 / 3.0;

/// Creates and returns a closure that performs layouts.
pub fn new() -> LayoutFunc {
    // wrapping the do function in a closure because I suspect we'll need additional parameters
    |a, b, c, d, e| do_horizontal_central_main_layout(a, b, c, d, e)
}

fn do_horizontal_central_main_layout(
    clients: &[&Client],
    _active_client_id: Option<WinId>,
    to_fill: &Region,
    main_region_window_count: u32,
    main_region_ratio: f32,
) -> Vec<ResizeAction> {
    if main_region_window_count >= clients.len() as u32 {
        do_all_main_layout(clients, to_fill)
    } else if main_region_window_count + 1 == clients.len() as u32 {
        do_two_region_layout(clients, to_fill, main_region_window_count, main_region_ratio)
    } else {
        do_three_region_layout(clients, to_fill, main_region_window_count, main_region_ratio)
    }
}

fn region_widths(to_fill: &Region, main_region_ratio: f32) -> (u32, u32) {
    let (_, _, w, _) = to_fill.values();
    let m = (w as f32 * main_region_ratio).ceil() as u32;

    // The non-main area must be even so it can be cut in half.
    let m = if (w - m) % 2 == 1 { m - 1 } else { m };

    (m, (w - m) / 2)
}

fn layout_main(to_fill: &Region, window_count: u32) -> Vec<Region> {
    match window_count {
        0 => Vec::new(),
        1 => vec![*to_fill],
        2 => {
            let (_, _, w, h) = to_fill.values();
            let floor = h * 3 / 4;
            let (row_a, row_b) = utils::split_at_height(&to_fill, floor);
            let wall = w * 2 / 3;
            let (col_a, col_b) = utils::split_at_width(&to_fill, wall);

            let row_ratio = utils::aspect_ratio(&row_a);
            let col_ratio = utils::aspect_ratio(&col_a);

            if (TARGET_ASPECT_RATIO - row_ratio).abs() < (TARGET_ASPECT_RATIO - col_ratio).abs() {
                vec![row_a, row_b]
            } else {
                vec![col_a, col_b]
            }
        }
        _ => layout_region(to_fill, window_count),
    }
}

// TODO: this should probably move to utils
fn aspect_ratio_sse(layout: &Vec<Region>) -> u32 {
    // u32 instead of f32 because we need Ord
    // * 100.0 here to avoid precision loss
    (layout.iter().map(|r| (utils::aspect_ratio(r) - TARGET_ASPECT_RATIO).powi(2)).sum::<f32>()
        * 100.0) as u32
}

fn layout_region(to_fill: &Region, window_count: u32) -> Vec<Region> {
    (0..window_count)
        .map(|c| layout_region_in_rows(to_fill, window_count, c + 1))
        .min_by_key(aspect_ratio_sse)
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

fn do_all_main_layout(clients: &[&Client], to_fill: &Region) -> Vec<ResizeAction> {
    let layout = layout_main(to_fill, clients.len() as u32);

    clients.iter().zip(layout).map(|(c, r)| (c.id(), Some(r))).collect()
}

// TODO: get rid of the distinction between two and three region layouts, 
//       always try both and pick the min by aspect ratio sse
fn do_two_region_layout(
    clients: &[&Client],
    to_fill: &Region,
    main_region_window_count: u32,
    main_region_ratio: f32,
) -> Vec<ResizeAction> {
    let (main_w, secondary_w) = region_widths(to_fill, main_region_ratio);
    let (main, secondary) = utils::split_at_width(to_fill, main_w + secondary_w);
    let main_layout = layout_main(&main, main_region_window_count);
    let secondary_layout =
        layout_region(&secondary, clients.len() as u32 - main_region_window_count);

    clients
        .iter()
        .zip(main_layout.into_iter().chain(secondary_layout))
        .map(|(c, r)| (c.id(), Some(r)))
        .collect()
}

fn do_three_region_layout(
    clients: &[&Client],
    to_fill: &Region,
    main_region_window_count: u32,
    main_region_ratio: f32,
) -> Vec<ResizeAction> {
    let (main_w, secondary_w) = region_widths(to_fill, main_region_ratio);
    let (left, remainder) = utils::split_at_width(to_fill, secondary_w);
    let (main, right) = utils::split_at_width(&remainder, main_w);

    let left_window_count = (clients.len() as u32 - main_region_window_count) / 2;
    let left_layout = layout_region(&left, left_window_count);
    let main_layout = layout_main(&main, main_region_window_count);
    let right_layout =
        layout_region(&right, clients.len() as u32 - left_window_count - main_region_window_count);

    clients
        .iter()
        .zip(main_layout.into_iter().chain(left_layout).chain(right_layout))
        .map(|(c, r)| (c.id(), Some(r)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fullscreens_single_client() {
        let client = Client::new(0, "hi".to_string(), "there".to_string(), 1, false);
        let screen = Region::new(0, 0, 1920, 1200);
        let results = do_horizontal_central_main_layout(&vec![&client], None, &screen, 1, 0.667);

        println!("{:?}", results);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1, Some(screen));
    }

    #[test]
    fn divides_columns_into_rows() {
        let screen = Region::new(0, 0, 480, 1200);
        let results = layout_region(&screen, 2);
        println!("{:?}", results);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], Region::new(0, 0, 480, 600));
        assert_eq!(results[1], Region::new(0, 600, 480, 600));
    }
}
