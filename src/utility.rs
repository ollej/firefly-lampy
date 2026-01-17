use firefly_rust::get_random;

/// Returns a random u32 in the range [min, max] (inclusive).
pub fn random_range(min: u32, max: u32) -> u32 {
    if min >= max {
        return min;
    }
    let range = max - min + 1;
    min + (get_random() % range)
}
