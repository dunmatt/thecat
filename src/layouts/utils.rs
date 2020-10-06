//! General purpose utilities to help with building layouts.  Many of these should migrate upstream.

use penrose::data_types::Region;

/// Computes the aspect ratio of a given region.
pub fn aspect_ratio(r: &Region) -> f32 {
    let (_, _, w, h) = r.values();
    w as f32 / h as f32
}

/// Divides this region into two columns where the first has the given width.
///
/// Panics if new_width is not within the region.
pub fn split_at_width(r: &Region, new_width: u32) -> (Region, Region) {
    let (x, y, w, h) = r.values();
    assert!(new_width < w, "Split out of range.");
    (Region::new(x, y, new_width, h), Region::new(x + new_width, y, w - new_width, h))
}

/// Divides this region into two rows where the first has the given height.
///
/// Panics if new_height is not within the region.
pub fn split_at_height(r: &Region, new_height: u32) -> (Region, Region) {
    let (x, y, w, h) = r.values();
    assert!(new_height < h, "Split out of range.");
    (Region::new(x, y, w, new_height), Region::new(x, y + new_height, w, h - new_height))
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
