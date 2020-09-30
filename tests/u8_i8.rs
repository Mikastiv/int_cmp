use int_cmp::IntCmp;

mod u8_i8_cmp {
    use super::*;
    #[test]
    fn u8_i8_eq() {
        assert!(IntCmp::cmp_eq(0_u8, 0_i8));
        assert!(!IntCmp::cmp_eq(1_u8, -1_i8));
        assert!(!IntCmp::cmp_eq(0_u8, -128_i8));
        assert!(IntCmp::cmp_eq(127_u8, 127_i8));
        assert!(!IntCmp::cmp_eq(255_u8, 127_i8));
    }

    #[test]
    fn u8_i8_nq() {
        assert!(!IntCmp::cmp_ne(0_u8, 0_i8));
        assert!(IntCmp::cmp_ne(1_u8, -1_i8));
        assert!(IntCmp::cmp_ne(0_u8, -128_i8));
        assert!(!IntCmp::cmp_ne(127_u8, 127_i8));
        assert!(IntCmp::cmp_ne(255_u8, 127_i8));
    }

    #[test]
    fn u8_i8_ge() {
        assert!(IntCmp::cmp_ge(0_u8, 0_i8));
        assert!(IntCmp::cmp_ge(0_u8, -1_i8));
        assert!(IntCmp::cmp_ge(0_u8, -128_i8));
        assert!(!IntCmp::cmp_ge(0_u8, 127_i8));
        assert!(IntCmp::cmp_ge(255_u8, 127_i8));
    }

    #[test]
    fn u8_i8_gt() {
        assert!(!IntCmp::cmp_gt(0_u8, 0_i8));
        assert!(IntCmp::cmp_gt(0_u8, -1_i8));
        assert!(IntCmp::cmp_gt(0_u8, -128_i8));
        assert!(!IntCmp::cmp_gt(0_u8, 127_i8));
        assert!(IntCmp::cmp_gt(255_u8, 127_i8));
    }

    #[test]
    fn u8_i8_le() {
        assert!(IntCmp::cmp_le(0_u8, 0_i8));
        assert!(!IntCmp::cmp_le(0_u8, -1_i8));
        assert!(!IntCmp::cmp_le(0_u8, -128_i8));
        assert!(IntCmp::cmp_le(0_u8, 127_i8));
        assert!(!IntCmp::cmp_le(255_u8, 127_i8));
    }

    #[test]
    fn u8_i8_lt() {
        assert!(!IntCmp::cmp_lt(0_u8, 0_i8));
        assert!(!IntCmp::cmp_lt(0_u8, -1_i8));
        assert!(!IntCmp::cmp_lt(0_u8, -128_i8));
        assert!(IntCmp::cmp_lt(0_u8, 127_i8));
        assert!(!IntCmp::cmp_lt(255_u8, 127_i8));
    }
}
