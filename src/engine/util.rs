pub fn mix_u64(x: u64) -> u64 {
    x.rotate_right(5).wrapping_mul(1099511628211)
}

pub fn mix_usize(x: usize) -> usize {
    mix_u64(x as u64) as usize
}
