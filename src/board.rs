use rand::rng;
use rand::seq::SliceRandom;

use crate::config::{GRID_CELLS, GRID_HEIGHT, GRID_WIDTH};

/// Generate a random boolean array of given length with a specified number of true values.
pub fn generate_random_bool_array(len: usize, number: usize) -> Vec<bool> {
    let mut array = vec![true; number];
    array.extend(vec![false; len - number]);
    array.shuffle(&mut rng());
    array
}

pub fn compute_neighbor_counts(bomb_distribution: &[bool]) -> Vec<u8> {
    let mut neighbor_counts = vec![0u8; GRID_CELLS];
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let idx = grid_index(x, y);
            if bomb_distribution[idx] {
                continue;
            }
            let mut count = 0u8;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if is_in_bounds(nx, ny)
                        && bomb_distribution[grid_index(nx as usize, ny as usize)]
                    {
                        count += 1;
                    }
                }
            }
            neighbor_counts[idx] = count;
        }
    }
    neighbor_counts
}

pub fn grid_index(x: usize, y: usize) -> usize {
    y * GRID_WIDTH + x
}

pub fn is_in_bounds(x: i32, y: i32) -> bool {
    (0..GRID_WIDTH as i32).contains(&x) && (0..GRID_HEIGHT as i32).contains(&y)
}
