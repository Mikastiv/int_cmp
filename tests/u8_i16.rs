use int_cmp::IntCmp;

mod u8_i16_cmp {
    use super::*;
    #[test]
    fn u8_i16_eq() {
        assert!(IntCmp::cmp_eq(u8::MIN, u8::MIN as i16));
        assert!(!IntCmp::cmp_eq(u8::MIN, -1_i16));
        assert!(!IntCmp::cmp_eq(u8::MIN, i16::MIN));
        assert!(!IntCmp::cmp_eq(u8::MAX, i16::MAX));
        assert!(IntCmp::cmp_eq(u8::MAX, u8::MAX as i16));
    }

    #[test]
    fn u8_i16_ne() {
        assert!(!IntCmp::cmp_ne(u8::MIN, u8::MIN as i16));
        assert!(IntCmp::cmp_ne(u8::MIN, -1_i16));
        assert!(IntCmp::cmp_ne(u8::MIN, i16::MIN));
        assert!(IntCmp::cmp_ne(u8::MAX, i16::MAX));
        assert!(!IntCmp::cmp_ne(u8::MAX, u8::MAX as i16));
    }

    #[test]
    fn u8_i16_ge() {
        assert!(IntCmp::cmp_ge(u8::MIN, u8::MIN as i16));
        assert!(IntCmp::cmp_ge(u8::MIN, i16::MIN));
        assert!(!IntCmp::cmp_ge(u8::MIN, i16::MAX));
        assert!(!IntCmp::cmp_ge(u8::MAX, i16::MAX));
        assert!(IntCmp::cmp_ge(u8::MAX, u8::MAX as i16));
    }

    #[test]
    fn u8_i16_gt() {
        assert!(!IntCmp::cmp_gt(u8::MIN, u8::MIN as i16));
        assert!(IntCmp::cmp_gt(u8::MIN, i16::MIN));
        assert!(!IntCmp::cmp_gt(u8::MIN, i16::MAX));
        assert!(!IntCmp::cmp_gt(u8::MAX, i16::MAX));
        assert!(!IntCmp::cmp_gt(u8::MAX, u8::MAX as i16));
    }

    #[test]
    fn u8_i16_le() {
        assert!(IntCmp::cmp_le(u8::MIN, u8::MIN as i16));
        assert!(!IntCmp::cmp_le(u8::MIN, i16::MIN));
        assert!(IntCmp::cmp_le(u8::MIN, i16::MAX));
        assert!(IntCmp::cmp_le(u8::MAX, i16::MAX));
        assert!(IntCmp::cmp_le(u8::MAX, u8::MAX as i16));
    }

    #[test]
    fn u8_i16_lt() {
        assert!(!IntCmp::cmp_lt(u8::MIN, u8::MIN as i16));
        assert!(!IntCmp::cmp_lt(u8::MIN, i16::MIN));
        assert!(IntCmp::cmp_lt(u8::MIN, i16::MAX));
        assert!(IntCmp::cmp_lt(u8::MAX, i16::MAX));
        assert!(!IntCmp::cmp_lt(u8::MAX, u8::MAX as i16));
    }
}
