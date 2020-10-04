use int_cmp::IntCmp;

mod i64_u8_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u8::MIN as i64, u8::MIN));
        assert!(!IntCmp::cmp_eq(-1_i64, u8::MIN));
        assert!(!IntCmp::cmp_eq(i64::MIN, u8::MIN));
        assert!(!IntCmp::cmp_eq(i64::MAX, u8::MAX));
        assert!(IntCmp::cmp_eq(u8::MAX as i64, u8::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u8::MIN as i64, u8::MIN));
        assert!(IntCmp::cmp_ne(-1_i64, u8::MIN));
        assert!(IntCmp::cmp_ne(i64::MIN, u8::MIN));
        assert!(IntCmp::cmp_ne(i64::MAX, u8::MAX));
        assert!(!IntCmp::cmp_ne(u8::MAX as i64, u8::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u8::MIN as i64, u8::MIN));
        assert!(!IntCmp::cmp_ge(i64::MIN, u8::MIN));
        assert!(IntCmp::cmp_ge(i64::MAX, u8::MIN));
        assert!(IntCmp::cmp_ge(i64::MAX, u8::MAX));
        assert!(IntCmp::cmp_ge(u8::MAX as i64, u8::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u8::MIN as i64, u8::MIN));
        assert!(!IntCmp::cmp_gt(i64::MIN, u8::MIN));
        assert!(IntCmp::cmp_gt(i64::MAX, u8::MIN));
        assert!(IntCmp::cmp_gt(i64::MAX, u8::MAX));
        assert!(!IntCmp::cmp_gt(u8::MAX as i64, u8::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u8::MIN as i64, u8::MIN));
        assert!(IntCmp::cmp_le(i64::MIN, u8::MIN));
        assert!(!IntCmp::cmp_le(i64::MAX, u8::MIN));
        assert!(!IntCmp::cmp_le(i64::MAX, u8::MAX));
        assert!(IntCmp::cmp_le(u8::MAX as i64, u8::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u8::MIN as i64, u8::MIN));
        assert!(IntCmp::cmp_lt(i64::MIN, u8::MIN));
        assert!(!IntCmp::cmp_lt(i64::MAX, u8::MIN));
        assert!(!IntCmp::cmp_lt(i64::MAX, u8::MAX));
        assert!(!IntCmp::cmp_lt(u8::MAX as i64, u8::MAX));
    }
}

mod i64_u16_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u16::MIN as i64, u16::MIN));
        assert!(!IntCmp::cmp_eq(-1_i64, u16::MIN));
        assert!(!IntCmp::cmp_eq(i64::MIN, u16::MIN));
        assert!(!IntCmp::cmp_eq(i64::MAX, u16::MAX));
        assert!(IntCmp::cmp_eq(u16::MAX as i64, u16::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u16::MIN as i64, u16::MIN));
        assert!(IntCmp::cmp_ne(-1_i64, u16::MIN));
        assert!(IntCmp::cmp_ne(i64::MIN, u16::MIN));
        assert!(IntCmp::cmp_ne(i64::MAX, u16::MAX));
        assert!(!IntCmp::cmp_ne(u16::MAX as i64, u16::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u16::MIN as i64, u16::MIN));
        assert!(!IntCmp::cmp_ge(i64::MIN, u16::MIN));
        assert!(IntCmp::cmp_ge(i64::MAX, u16::MIN));
        assert!(IntCmp::cmp_ge(i64::MAX, u16::MAX));
        assert!(IntCmp::cmp_ge(u16::MAX as i64, u16::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u16::MIN as i64, u16::MIN));
        assert!(!IntCmp::cmp_gt(i64::MIN, u16::MIN));
        assert!(IntCmp::cmp_gt(i64::MAX, u16::MIN));
        assert!(IntCmp::cmp_gt(i64::MAX, u16::MAX));
        assert!(!IntCmp::cmp_gt(u16::MAX as i64, u16::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u16::MIN as i64, u16::MIN));
        assert!(IntCmp::cmp_le(i64::MIN, u16::MIN));
        assert!(!IntCmp::cmp_le(i64::MAX, u16::MIN));
        assert!(!IntCmp::cmp_le(i64::MAX, u16::MAX));
        assert!(IntCmp::cmp_le(u16::MAX as i64, u16::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u16::MIN as i64, u16::MIN));
        assert!(IntCmp::cmp_lt(i64::MIN, u16::MIN));
        assert!(!IntCmp::cmp_lt(i64::MAX, u16::MIN));
        assert!(!IntCmp::cmp_lt(i64::MAX, u16::MAX));
        assert!(!IntCmp::cmp_lt(u16::MAX as i64, u16::MAX));
    }
}

mod i64_u32_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u32::MIN as i64, u32::MIN));
        assert!(!IntCmp::cmp_eq(-1_i64, u32::MIN));
        assert!(!IntCmp::cmp_eq(i64::MIN, u32::MIN));
        assert!(!IntCmp::cmp_eq(i64::MAX, u32::MAX));
        assert!(IntCmp::cmp_eq(u32::MAX as i64, u32::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u32::MIN as i64, u32::MIN));
        assert!(IntCmp::cmp_ne(-1_i64, u32::MIN));
        assert!(IntCmp::cmp_ne(i64::MIN, u32::MIN));
        assert!(IntCmp::cmp_ne(i64::MAX, u32::MAX));
        assert!(!IntCmp::cmp_ne(u32::MAX as i64, u32::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u32::MIN as i64, u32::MIN));
        assert!(!IntCmp::cmp_ge(i64::MIN, u32::MIN));
        assert!(IntCmp::cmp_ge(i64::MAX, u32::MIN));
        assert!(IntCmp::cmp_ge(i64::MAX, u32::MAX));
        assert!(IntCmp::cmp_ge(u32::MAX as i64, u32::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u32::MIN as i64, u32::MIN));
        assert!(!IntCmp::cmp_gt(i64::MIN, u32::MIN));
        assert!(IntCmp::cmp_gt(i64::MAX, u32::MIN));
        assert!(IntCmp::cmp_gt(i64::MAX, u32::MAX));
        assert!(!IntCmp::cmp_gt(u32::MAX as i64, u32::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u32::MIN as i64, u32::MIN));
        assert!(IntCmp::cmp_le(i64::MIN, u32::MIN));
        assert!(!IntCmp::cmp_le(i64::MAX, u32::MIN));
        assert!(!IntCmp::cmp_le(i64::MAX, u32::MAX));
        assert!(IntCmp::cmp_le(u32::MAX as i64, u32::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u32::MIN as i64, u32::MIN));
        assert!(IntCmp::cmp_lt(i64::MIN, u32::MIN));
        assert!(!IntCmp::cmp_lt(i64::MAX, u32::MIN));
        assert!(!IntCmp::cmp_lt(i64::MAX, u32::MAX));
        assert!(!IntCmp::cmp_lt(u32::MAX as i64, u32::MAX));
    }
}

mod i64_u64_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u64::MIN as i64, u64::MIN));
        assert!(!IntCmp::cmp_eq(-1_i64, u64::MIN));
        assert!(!IntCmp::cmp_eq(i64::MIN, u64::MIN));
        assert!(!IntCmp::cmp_eq(i64::MAX, u64::MAX));
        assert!(IntCmp::cmp_eq(i64::MAX, i64::MAX as u64));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u64::MIN as i64, u64::MIN));
        assert!(IntCmp::cmp_ne(-1_i64, u64::MIN));
        assert!(IntCmp::cmp_ne(i64::MIN, u64::MIN));
        assert!(IntCmp::cmp_ne(i64::MAX, u64::MAX));
        assert!(!IntCmp::cmp_ne(i64::MAX, i64::MAX as u64));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u64::MIN as i64, u64::MIN));
        assert!(!IntCmp::cmp_ge(i64::MIN, u64::MIN));
        assert!(IntCmp::cmp_ge(i64::MAX, u64::MIN));
        assert!(!IntCmp::cmp_ge(i64::MAX, u64::MAX));
        assert!(IntCmp::cmp_ge(i64::MAX, i64::MAX as u64));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u64::MIN as i64, u64::MIN));
        assert!(!IntCmp::cmp_gt(i64::MIN, u64::MIN));
        assert!(IntCmp::cmp_gt(i64::MAX, u64::MIN));
        assert!(!IntCmp::cmp_gt(i64::MAX, u64::MAX));
        assert!(!IntCmp::cmp_gt(i64::MAX, i64::MAX as u64));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u64::MIN as i64, u64::MIN));
        assert!(IntCmp::cmp_le(i64::MIN, u64::MIN));
        assert!(!IntCmp::cmp_le(i64::MAX, u64::MIN));
        assert!(IntCmp::cmp_le(i64::MAX, u64::MAX));
        assert!(IntCmp::cmp_le(i64::MAX, i64::MAX as u64));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u64::MIN as i64, u64::MIN));
        assert!(IntCmp::cmp_lt(i64::MIN, u64::MIN));
        assert!(!IntCmp::cmp_lt(i64::MAX, u64::MIN));
        assert!(IntCmp::cmp_lt(i64::MAX, u64::MAX));
        assert!(!IntCmp::cmp_lt(i64::MAX, i64::MAX as u64));
    }
}

mod i64_u128_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u128::MIN as i64, u128::MIN));
        assert!(!IntCmp::cmp_eq(-1_i64, 1_u128));
        assert!(!IntCmp::cmp_eq(i64::MIN, u128::MIN));
        assert!(IntCmp::cmp_eq(i64::MAX, i64::MAX as u128));
        assert!(!IntCmp::cmp_eq(i64::MAX, u128::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u128::MIN as i64, u128::MIN));
        assert!(IntCmp::cmp_ne(-1_i64, 1_u128));
        assert!(IntCmp::cmp_ne(i64::MIN, u128::MIN));
        assert!(!IntCmp::cmp_ne(i64::MAX, i64::MAX as u128));
        assert!(IntCmp::cmp_ne(i64::MAX, u128::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u128::MIN as i64, u128::MIN));
        assert!(IntCmp::cmp_ge(i16::MAX, u128::MIN));
        assert!(!IntCmp::cmp_ge(i64::MIN, u128::MIN));
        assert!(IntCmp::cmp_ge(i64::MAX, i64::MAX as u128));
        assert!(!IntCmp::cmp_ge(i64::MAX, u128::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u128::MIN as i64, u128::MIN));
        assert!(IntCmp::cmp_gt(i16::MAX, u128::MIN));
        assert!(!IntCmp::cmp_gt(i64::MIN, u128::MIN));
        assert!(!IntCmp::cmp_gt(i64::MAX, i64::MAX as u128));
        assert!(!IntCmp::cmp_gt(i64::MAX, u128::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u128::MIN as i64, u128::MIN));
        assert!(!IntCmp::cmp_le(i16::MAX, u128::MIN));
        assert!(IntCmp::cmp_le(i64::MIN, u128::MIN));
        assert!(IntCmp::cmp_le(i64::MAX, i64::MAX as u128));
        assert!(IntCmp::cmp_le(i64::MAX, u128::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u128::MIN as i64, u128::MIN));
        assert!(!IntCmp::cmp_lt(i16::MAX, u128::MIN));
        assert!(IntCmp::cmp_lt(i64::MIN, u128::MIN));
        assert!(!IntCmp::cmp_lt(i64::MAX, i64::MAX as u128));
        assert!(IntCmp::cmp_lt(i64::MAX, u128::MAX));
    }
}
