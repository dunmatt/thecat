//! The horizontal central main layout has a large central area for your most important
//! window(s) and equally sized side areas for others.
//!
//! Within each of the three areas, windows are tiled to remain approximately 4:3, with ties
//! broken in favor of (by which I mean more space given to) the first window(s) in the area.
//!

use penrose::{
    client::Client,
    data_types::{Region, ResizeAction},
    layout::LayoutFunc,
};

use super::{fair::layout_region_fairly, utils, TARGET_ASPECT_RATIO};

/// Creates and returns a closure that performs layouts.
pub fn new() -> LayoutFunc {
    |a, _b, c, d, e| do_horizontal_central_main_layout(a, c, d, e)
}

fn do_horizontal_central_main_layout(
    clients: &[&Client],
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
            let (row_a, row_b) = to_fill.split_at_height(floor);
            let wall = w * 2 / 3;
            let (col_a, col_b) = to_fill.split_at_width(wall);

            let row_ratio = utils::aspect_ratio(&row_a);
            let col_ratio = utils::aspect_ratio(&col_a);

            if (TARGET_ASPECT_RATIO - row_ratio).abs() < (TARGET_ASPECT_RATIO - col_ratio).abs() {
                vec![row_a, row_b]
            } else {
                vec![col_a, col_b]
            }
        }
        _ => layout_region_fairly(to_fill, window_count),
    }
}

fn do_all_main_layout(clients: &[&Client], to_fill: &Region) -> Vec<ResizeAction> {
    let layout = layout_main(to_fill, clients.len() as u32);

    clients.iter().zip(layout).map(|(c, r)| (c.id(), Some(r))).collect()
}

fn do_two_region_layout(
    clients: &[&Client],
    to_fill: &Region,
    main_region_window_count: u32,
    main_region_ratio: f32,
) -> Vec<ResizeAction> {
    // 2/3rds here to account for the main window also getting the left column.
    let main_region_ratio = main_region_ratio * 2.0 / 3.0;
    let (main_w, secondary_w) = region_widths(to_fill, main_region_ratio);
    let (main, secondary) = to_fill.split_at_width(main_w + secondary_w);
    let main_layout = layout_main(&main, main_region_window_count);
    let secondary_layout =
        layout_region_fairly(&secondary, clients.len() as u32 - main_region_window_count);

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
    let (left, remainder) = to_fill.split_at_width(secondary_w);
    let (main, right) = remainder.split_at_width(main_w);

    let left_window_count = (clients.len() as u32 - main_region_window_count) / 2;
    let left_layout = layout_region_fairly(&left, left_window_count);
    let main_layout = layout_main(&main, main_region_window_count);
    let right_layout = layout_region_fairly(
        &right,
        clients.len() as u32 - left_window_count - main_region_window_count,
    );

    clients
        .iter()
        .zip(left_layout.into_iter().chain(main_layout).chain(right_layout))
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
