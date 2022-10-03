pub mod bit_ops {

    fn most_significant_bits(x: usize) -> usize {
        assert_eq!(usize::BITS, 64);
        (64 - x.leading_zeros() - 1).try_into().unwrap()
    }

    fn is_bit_set(val: usize, bit_pos: usize) -> bool {
        let mask = 1 << bit_pos;
        val & mask == mask
    }

}
