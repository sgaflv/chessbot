use crate::bboard::*;


#[inline]
pub fn add_bit(board: &mut BBoard, x: u32, y: u32) {

    if x > 7 { return; }
    if y > 7 { return; }

    (*board) |= 1u64 << (x + y * 8) as u64;
}

#[inline]
pub fn has_bit(board: &BBoard, x: u32, y: u32) -> bool {
    if x > 7 { return false; }
    if y > 7 { return false; }

    return (*board & (1u64 << (x + y * 8) as u64)) > 0;
}


#[cfg(test)]
mod tests {
    use std::num::Wrapping;

    #[test]
    fn test_pop_count() {
        let test3 = 1u64 << 5 | 1 << 10 | 1 << 20;
        assert_eq!(test3.count_ones(), 3);
    }

    #[test]
    fn test_pop_count_side_bits() {
        let test64 = (Wrapping(1u64) << 63) + Wrapping(1);
        assert_eq!(test64.0.count_ones(), 2);
    }

    #[test]
    fn test_pop_count_all_bits() {
        let test64 = Wrapping(0u64) - Wrapping(1);
        assert_eq!(test64.0.count_ones(), 64);
    }

    #[test]
    fn test_pop_count_0() {
        assert_eq!(0u64.count_ones(), 0);
    }
}
