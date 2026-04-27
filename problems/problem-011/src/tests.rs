//! Tests for `compute_max_product`.
//!
//! To enable, add this line to the top of `main.rs`:
//!
//!     #[cfg(test)] mod tests;
//!
//! Then run `cargo test`.
//!
//! Currently assumes:
//!   - `compute_max_product(grid: &[Vec<u64>]) -> Option<BestRun>`
//!   - `BestRun { product: u64, start: (usize, usize), direction: Direction }`
//!     deriving `Debug` + `PartialEq`
//!   - `Direction` deriving `Debug` + `PartialEq` + `Clone` + `Copy`
//!
//! When you refactor to a real `Grid` struct, change the helper below to build
//! a `Grid` and update the parameter type. Tests stay the same shape.

use super::{compute_max_product, BestRun, Direction};

fn tiny_grid() -> Vec<Vec<u64>> {
    vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 1, 2, 3],
        vec![4, 5, 6, 7],
    ]
}

#[test]
fn finds_largest_product_in_tiny_grid() {
    // Hand-checked: row 1 (5*6*7*8 = 1680) beats every other run of 4.
    let grid = tiny_grid();
    let best = compute_max_product(&grid).expect("4x4 grid has at least one valid run");

    assert_eq!(best.product, 1680);
    assert_eq!(best.start, (1, 0));
    assert_eq!(best.direction, Direction::Right);
}

#[test]
fn returns_none_for_too_small_grid() {
    // 3x3 can't fit a run of 4 in any direction.
    let grid = vec![vec![1, 2, 3]; 3];
    assert!(compute_max_product(&grid).is_none());
}

#[test]
fn finds_down_right_diagonal_when_it_wins() {
    let grid = vec![
        vec![9, 1, 1, 1],
        vec![1, 9, 1, 1],
        vec![1, 1, 9, 1],
        vec![1, 1, 1, 9],
    ];
    let best = compute_max_product(&grid).expect("grid should have a run");

    assert_eq!(best.product, 9 * 9 * 9 * 9);
    assert_eq!(best.start, (0, 0));
    assert_eq!(best.direction, Direction::DownRight);
}

#[test]
fn finds_down_left_diagonal_when_it_wins() {
    let grid = vec![
        vec![1, 1, 1, 9],
        vec![1, 1, 9, 1],
        vec![1, 9, 1, 1],
        vec![9, 1, 1, 1],
    ];
    let best = compute_max_product(&grid).expect("grid should have a run");

    assert_eq!(best.product, 9 * 9 * 9 * 9);
    assert_eq!(best.start, (0, 3));
    assert_eq!(best.direction, Direction::DownLeft);
}
