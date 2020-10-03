use int_cmp::IntCmp;

mod i16_u8_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u8::MIN as i16, u8::MIN));
        assert!(!IntCmp::cmp_eq(-1_i16, u8::MIN));
        assert!(!IntCmp::cmp_eq(i16::MIN, u8::MIN));
        assert!(!IntCmp::cmp_eq(i16::MAX, u8::MAX));
        assert!(IntCmp::cmp_eq(u8::MAX as i16, u8::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u8::MIN as i16, u8::MIN));
        assert!(IntCmp::cmp_ne(-1_i16, u8::MIN));
        assert!(IntCmp::cmp_ne(i16::MIN, u8::MIN));
        assert!(IntCmp::cmp_ne(i16::MAX, u8::MAX));
        assert!(!IntCmp::cmp_ne(u8::MAX as i16, u8::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u8::MIN as i16, u8::MIN));
        assert!(IntCmp::cmp_ge(i16::MAX, u8::MIN));
        assert!(!IntCmp::cmp_ge(i16::MIN, u8::MIN));
        assert!(IntCmp::cmp_ge(i16::MAX, u8::MAX));
        assert!(IntCmp::cmp_ge(u8::MAX as i16, u8::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u8::MIN as i16, u8::MIN));
        assert!(!IntCmp::cmp_gt(i16::MIN, u8::MIN));
        assert!(IntCmp::cmp_gt(i16::MAX, u8::MIN));
        assert!(IntCmp::cmp_gt(i16::MAX, u8::MAX));
        assert!(!IntCmp::cmp_gt(u8::MAX as i16, u8::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u8::MIN as i16, u8::MIN));
        assert!(IntCmp::cmp_le(i16::MIN, u8::MIN));
        assert!(!IntCmp::cmp_le(i16::MAX, u8::MIN));
        assert!(!IntCmp::cmp_le(i16::MAX, u8::MAX));
        assert!(IntCmp::cmp_le(u8::MAX as i16, u8::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u8::MIN as i16, u8::MIN));
        assert!(IntCmp::cmp_lt(i16::MIN, u8::MIN));
        assert!(!IntCmp::cmp_lt(i16::MAX, u8::MIN));
        assert!(!IntCmp::cmp_lt(i16::MAX, u8::MAX));
        assert!(!IntCmp::cmp_lt(u8::MAX as i16, u8::MAX));
    }
}

mod i16_u16_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u16::MIN as i16, u16::MIN));
        assert!(!IntCmp::cmp_eq(-1_i16, u16::MIN));
        assert!(!IntCmp::cmp_eq(i16::MIN, u16::MIN));
        assert!(!IntCmp::cmp_eq(i16::MAX, u16::MAX));
        assert!(IntCmp::cmp_eq(i16::MAX, i16::MAX as u16));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u16::MIN as i16, u16::MIN));
        assert!(IntCmp::cmp_ne(-1_i16, u16::MIN));
        assert!(IntCmp::cmp_ne(i16::MIN, u16::MIN));
        assert!(IntCmp::cmp_ne(i16::MAX, u16::MAX));
        assert!(!IntCmp::cmp_ne(i16::MAX, i16::MAX as u16));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u16::MIN as i16, u16::MIN));
        assert!(IntCmp::cmp_ge(i16::MAX, u16::MIN));
        assert!(!IntCmp::cmp_ge(i16::MIN, u16::MIN));
        assert!(IntCmp::cmp_ge(i16::MAX, i16::MAX as u16));
        assert!(!IntCmp::cmp_ge(i16::MAX, u16::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u16::MIN as i16, u16::MIN));
        assert!(IntCmp::cmp_gt(i16::MAX, u16::MIN));
        assert!(!IntCmp::cmp_gt(i16::MIN, u16::MIN));
        assert!(!IntCmp::cmp_gt(i16::MAX, i16::MAX as u16));
        assert!(!IntCmp::cmp_gt(i16::MAX, u16::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u16::MIN as i16, u16::MIN));
        assert!(IntCmp::cmp_le(-1_i16, u16::MIN));
        assert!(IntCmp::cmp_le(i16::MIN, u16::MIN));
        assert!(IntCmp::cmp_le(i16::MAX, i16::MAX as u16));
        assert!(IntCmp::cmp_le(i16::MAX, u16::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u16::MIN as i16, u16::MIN));
        assert!(IntCmp::cmp_lt(-1_i16, u16::MIN));
        assert!(IntCmp::cmp_lt(i16::MIN, u16::MIN));
        assert!(!IntCmp::cmp_lt(i16::MAX, i16::MAX as u16));
        assert!(IntCmp::cmp_lt(i16::MAX, u16::MAX));
    }
}

mod i16_u32_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u32::MIN as i16, u32::MIN));
        assert!(!IntCmp::cmp_eq(-1_i16, 1_u32));
        assert!(!IntCmp::cmp_eq(i16::MIN, u32::MIN));
        assert!(IntCmp::cmp_eq(i16::MAX, i16::MAX as u32));
        assert!(!IntCmp::cmp_eq(i16::MAX, u32::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u32::MIN as i16, u32::MIN));
        assert!(IntCmp::cmp_ne(-1_i16, 1_u32));
        assert!(IntCmp::cmp_ne(i16::MIN, u32::MIN));
        assert!(!IntCmp::cmp_ne(i16::MAX, i16::MAX as u32));
        assert!(IntCmp::cmp_ne(i16::MAX, u32::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u32::MIN as i16, u32::MIN));
        assert!(IntCmp::cmp_ge(i16::MAX, u32::MIN));
        assert!(!IntCmp::cmp_ge(i16::MIN, u32::MIN));
        assert!(IntCmp::cmp_ge(i16::MAX, i16::MAX as u32));
        assert!(!IntCmp::cmp_ge(i16::MAX, u32::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u32::MIN as i16, u32::MIN));
        assert!(IntCmp::cmp_gt(i16::MAX, u32::MIN));
        assert!(!IntCmp::cmp_gt(i16::MIN, u32::MIN));
        assert!(!IntCmp::cmp_gt(i16::MAX, i16::MAX as u32));
        assert!(!IntCmp::cmp_gt(i16::MAX, u32::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u32::MIN as i16, u32::MIN));
        assert!(IntCmp::cmp_le(-1_i16, u32::MIN));
        assert!(IntCmp::cmp_le(i16::MIN, u32::MIN));
        assert!(IntCmp::cmp_le(i16::MAX, i16::MAX as u32));
        assert!(IntCmp::cmp_le(i16::MAX, u32::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u32::MIN as i16, u32::MIN));
        assert!(IntCmp::cmp_lt(-1_i16, u32::MIN));
        assert!(IntCmp::cmp_lt(i16::MIN, u32::MIN));
        assert!(!IntCmp::cmp_lt(i16::MAX, i16::MAX as u32));
        assert!(IntCmp::cmp_lt(i16::MAX, u32::MAX));
    }
}

mod i16_u64_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u64::MIN as i16, u64::MIN));
        assert!(!IntCmp::cmp_eq(-1_i16, 1_u64));
        assert!(!IntCmp::cmp_eq(i16::MIN, u64::MIN));
        assert!(IntCmp::cmp_eq(i16::MAX, i16::MAX as u64));
        assert!(!IntCmp::cmp_eq(i16::MAX, u64::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u64::MIN as i16, u64::MIN));
        assert!(IntCmp::cmp_ne(-1_i16, 1_u64));
        assert!(IntCmp::cmp_ne(i16::MIN, u64::MIN));
        assert!(!IntCmp::cmp_ne(i16::MAX, i16::MAX as u64));
        assert!(IntCmp::cmp_ne(i16::MAX, u64::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u64::MIN as i16, u64::MIN));
        assert!(IntCmp::cmp_ge(i16::MAX, u64::MIN));
        assert!(!IntCmp::cmp_ge(i16::MIN, u64::MIN));
        assert!(IntCmp::cmp_ge(i16::MAX, i16::MAX as u64));
        assert!(!IntCmp::cmp_ge(i16::MAX, u64::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u64::MIN as i16, u64::MIN));
        assert!(IntCmp::cmp_gt(i16::MAX, u64::MIN));
        assert!(!IntCmp::cmp_gt(i16::MIN, u64::MIN));
        assert!(!IntCmp::cmp_gt(i16::MAX, i16::MAX as u64));
        assert!(!IntCmp::cmp_gt(i16::MAX, u64::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u64::MIN as i16, u64::MIN));
        assert!(IntCmp::cmp_le(-1_i16, u64::MIN));
        assert!(IntCmp::cmp_le(i16::MIN, u64::MIN));
        assert!(IntCmp::cmp_le(i16::MAX, i16::MAX as u64));
        assert!(IntCmp::cmp_le(i16::MAX, u64::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u64::MIN as i16, u64::MIN));
        assert!(IntCmp::cmp_lt(-1_i16, u64::MIN));
        assert!(IntCmp::cmp_lt(i16::MIN, u64::MIN));
        assert!(!IntCmp::cmp_lt(i16::MAX, i16::MAX as u64));
        assert!(IntCmp::cmp_lt(i16::MAX, u64::MAX));
    }
}

mod i16_u128_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u128::MIN as i16, u128::MIN));
        assert!(!IntCmp::cmp_eq(-1_i16, 1_u128));
        assert!(!IntCmp::cmp_eq(i16::MIN, u128::MIN));
        assert!(IntCmp::cmp_eq(i16::MAX, i16::MAX as u128));
        assert!(!IntCmp::cmp_eq(i16::MAX, u128::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u128::MIN as i16, u128::MIN));
        assert!(IntCmp::cmp_ne(-1_i16, 1_u128));
        assert!(IntCmp::cmp_ne(i16::MIN, u128::MIN));
        assert!(!IntCmp::cmp_ne(i16::MAX, i16::MAX as u128));
        assert!(IntCmp::cmp_ne(i16::MAX, u128::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u128::MIN as i16, u128::MIN));
        assert!(IntCmp::cmp_ge(i16::MAX, u128::MIN));
        assert!(!IntCmp::cmp_ge(i16::MIN, u128::MIN));
        assert!(IntCmp::cmp_ge(i16::MAX, i16::MAX as u128));
        assert!(!IntCmp::cmp_ge(i16::MAX, u128::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u128::MIN as i16, u128::MIN));
        assert!(IntCmp::cmp_gt(i16::MAX, u128::MIN));
        assert!(!IntCmp::cmp_gt(i16::MIN, u128::MIN));
        assert!(!IntCmp::cmp_gt(i16::MAX, i16::MAX as u128));
        assert!(!IntCmp::cmp_gt(i16::MAX, u128::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u128::MIN as i16, u128::MIN));
        assert!(IntCmp::cmp_le(-1_i16, u128::MIN));
        assert!(IntCmp::cmp_le(i16::MIN, u128::MIN));
        assert!(IntCmp::cmp_le(i16::MAX, i16::MAX as u128));
        assert!(IntCmp::cmp_le(i16::MAX, u128::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u128::MIN as i16, u128::MIN));
        assert!(IntCmp::cmp_lt(-1_i16, u128::MIN));
        assert!(IntCmp::cmp_lt(i16::MIN, u128::MIN));
        assert!(!IntCmp::cmp_lt(i16::MAX, i16::MAX as u128));
        assert!(IntCmp::cmp_lt(i16::MAX, u128::MAX));
    }
}
