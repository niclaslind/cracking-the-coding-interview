pub fn insert(n: i32, m: i32, i: i32, j: i32) -> i32 {
    // Create a mask to clear bits i through j in n.
    //
    // EXAMPLE: i = 2, j = 4. Result should be 11100011.
    // For simplicity we'll use just 8 bits for the exmaple.
    let all_ones = !0;

    // 1s before position j, then 0s. left = 11100000
    let left = all_ones << (j + 1);

    // 1's after position i. right = 00000011
    let right = (1 << i) - 1;

    // All 1s, expect for 0s between i and j. mask = 11100011
    let mask = left | right;

    // Clear bits j through i then put m in there
    let n_cleared = n & mask;   // clear bits j through i
    let m_shitfted = m << i;    // move m into correct position

    // OR them, and we're done
    n_cleared | m_shitfted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        assert_eq!(insert(0b100_0000_0000, 0b10011, 2, 6), 0b100_0100_1100)
    }
}
