use int_cmp::IntCmp;

mod u8_i8_cmp {
    use super::*;
    #[test]
    fn u8_i8_eq() {
        assert!(IntCmp::cmp_eq(u8::MIN, u8::MIN as i8));
        assert!(!IntCmp::cmp_eq(1_u8, -1_i8));
        assert!(!IntCmp::cmp_eq(u8::MIN, i8::MIN));
        assert!(IntCmp::cmp_eq(i8::MAX as u8, i8::MAX));
        assert!(!IntCmp::cmp_eq(u8::MAX, i8::MAX));
    }

    #[test]
    fn u8_i8_ne() {
        assert!(!IntCmp::cmp_ne(u8::MIN, u8::MIN as i8));
        assert!(IntCmp::cmp_ne(1_u8, -1_i8));
        assert!(IntCmp::cmp_ne(u8::MIN, i8::MIN));
        assert!(!IntCmp::cmp_ne(i8::MAX as u8, i8::MAX));
        assert!(IntCmp::cmp_ne(u8::MAX, i8::MAX));
    }

    #[test]
    fn u8_i8_ge() {
        assert!(IntCmp::cmp_ge(u8::MIN, u8::MIN as i8));
        assert!(IntCmp::cmp_ge(u8::MIN, -1_i8));
        assert!(IntCmp::cmp_ge(u8::MIN, i8::MIN));
        assert!(IntCmp::cmp_ge(i8::MAX as u8, i8::MAX));
        assert!(IntCmp::cmp_ge(u8::MAX, i8::MAX));
    }

    #[test]
    fn u8_i8_gt() {
        assert!(!IntCmp::cmp_gt(u8::MIN, u8::MIN as i8));
        assert!(IntCmp::cmp_gt(u8::MIN, -1_i8));
        assert!(IntCmp::cmp_gt(u8::MIN, i8::MIN));
        assert!(!IntCmp::cmp_gt(i8::MAX as u8, i8::MAX));
        assert!(IntCmp::cmp_gt(u8::MAX, i8::MAX));
    }

    #[test]
    fn u8_i8_le() {
        assert!(IntCmp::cmp_le(u8::MIN, u8::MIN as i8));
        assert!(!IntCmp::cmp_le(u8::MIN, -1_i8));
        assert!(!IntCmp::cmp_le(u8::MIN, i8::MIN));
        assert!(IntCmp::cmp_le(i8::MAX as u8, i8::MAX));
        assert!(!IntCmp::cmp_le(u8::MAX, i8::MAX));
    }

    #[test]
    fn u8_i8_lt() {
        assert!(!IntCmp::cmp_lt(u8::MIN, u8::MIN as i8));
        assert!(!IntCmp::cmp_lt(u8::MIN, -1_i8));
        assert!(!IntCmp::cmp_lt(u8::MIN, i8::MIN));
        assert!(!IntCmp::cmp_lt(i8::MAX as u8, i8::MAX));
        assert!(!IntCmp::cmp_lt(u8::MAX, i8::MAX));
    }
}
