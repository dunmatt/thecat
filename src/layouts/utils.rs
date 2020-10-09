//! General purpose utilities to help with building layouts.  Many of these should migrate
//! upstream.

use penrose::data_types::Region;

use super::TARGET_ASPECT_RATIO;

/// Computes the aspect ratio of a given region.
pub fn aspect_ratio(r: &Region) -> f32 {
    let (_, _, w, h) = r.values();
    w as f32 / h as f32
}

/// Computes the sum of the square errors of each window's aspect ratio versus the target ratio
/// of 16:9.
pub fn aspect_ratio_sse(layout: &Vec<Region>) -> u32 {
    // u32 instead of f32 because we need Ord
    // * 1000.0 here to avoid precision loss
    (layout.iter().map(|r| (aspect_ratio(r) - TARGET_ASPECT_RATIO).powi(2)).sum::<f32>() * 1000.0)
        as u32
}

/// Divides this region into `count` equal columns.
pub fn split_into_columns(r: &Region, count: u32) -> Vec<Region> {
    assert!(count > 0, "Cannot split into 0 columns.");
    let (x, y, w, h) = r.values();
    let width = w / count;

    let mut results = Vec::new();
    for i in 0..(count - 1) {
        results.push(Region::new(x + i * width, y, width, h));
    }
    let accounted_for = (count - 1) * width;
    results.push(Region::new(x + accounted_for, y, w - accounted_for, h));

    results
}

/// Divides this region into `count` equal rows.
pub fn split_into_rows(r: &Region, count: u32) -> Vec<Region> {
    assert!(count > 0, "Cannot split into 0 rows.");
    let (x, y, w, h) = r.values();
    let height = h / count;

    let mut results = Vec::new();
    for i in 0..(count - 1) {
        results.push(Region::new(x, y + i * height, w, height));
    }
    let accounted_for = (count - 1) * height;
    results.push(Region::new(x, y + accounted_for, w, h - accounted_for));

    results
}
