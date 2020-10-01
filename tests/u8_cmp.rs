use int_cmp::IntCmp;

mod u8_i8_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u8::MIN, u8::MIN as i8));
        assert!(!IntCmp::cmp_eq(1_u8, -1_i8));
        assert!(!IntCmp::cmp_eq(u8::MIN, i8::MIN));
        assert!(IntCmp::cmp_eq(i8::MAX as u8, i8::MAX));
        assert!(!IntCmp::cmp_eq(u8::MAX, i8::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u8::MIN, u8::MIN as i8));
        assert!(IntCmp::cmp_ne(1_u8, -1_i8));
        assert!(IntCmp::cmp_ne(u8::MIN, i8::MIN));
        assert!(!IntCmp::cmp_ne(i8::MAX as u8, i8::MAX));
        assert!(IntCmp::cmp_ne(u8::MAX, i8::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u8::MIN, u8::MIN as i8));
        assert!(IntCmp::cmp_ge(u8::MIN, -1_i8));
        assert!(IntCmp::cmp_ge(u8::MIN, i8::MIN));
        assert!(IntCmp::cmp_ge(i8::MAX as u8, i8::MAX));
        assert!(IntCmp::cmp_ge(u8::MAX, i8::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u8::MIN, u8::MIN as i8));
        assert!(IntCmp::cmp_gt(u8::MIN, -1_i8));
        assert!(IntCmp::cmp_gt(u8::MIN, i8::MIN));
        assert!(!IntCmp::cmp_gt(i8::MAX as u8, i8::MAX));
        assert!(IntCmp::cmp_gt(u8::MAX, i8::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u8::MIN, u8::MIN as i8));
        assert!(!IntCmp::cmp_le(u8::MIN, -1_i8));
        assert!(!IntCmp::cmp_le(u8::MIN, i8::MIN));
        assert!(IntCmp::cmp_le(i8::MAX as u8, i8::MAX));
        assert!(!IntCmp::cmp_le(u8::MAX, i8::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u8::MIN, u8::MIN as i8));
        assert!(!IntCmp::cmp_lt(u8::MIN, -1_i8));
        assert!(!IntCmp::cmp_lt(u8::MIN, i8::MIN));
        assert!(!IntCmp::cmp_lt(i8::MAX as u8, i8::MAX));
        assert!(!IntCmp::cmp_lt(u8::MAX, i8::MAX));
    }
}

mod u8_i16_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u8::MIN, u8::MIN as i16));
        assert!(!IntCmp::cmp_eq(u8::MIN, -1_i16));
        assert!(!IntCmp::cmp_eq(u8::MIN, i16::MIN));
        assert!(!IntCmp::cmp_eq(u8::MAX, i16::MAX));
        assert!(IntCmp::cmp_eq(u8::MAX, u8::MAX as i16));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u8::MIN, u8::MIN as i16));
        assert!(IntCmp::cmp_ne(u8::MIN, -1_i16));
        assert!(IntCmp::cmp_ne(u8::MIN, i16::MIN));
        assert!(IntCmp::cmp_ne(u8::MAX, i16::MAX));
        assert!(!IntCmp::cmp_ne(u8::MAX, u8::MAX as i16));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u8::MIN, u8::MIN as i16));
        assert!(IntCmp::cmp_ge(u8::MIN, i16::MIN));
        assert!(!IntCmp::cmp_ge(u8::MIN, i16::MAX));
        assert!(!IntCmp::cmp_ge(u8::MAX, i16::MAX));
        assert!(IntCmp::cmp_ge(u8::MAX, u8::MAX as i16));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u8::MIN, u8::MIN as i16));
        assert!(IntCmp::cmp_gt(u8::MIN, i16::MIN));
        assert!(!IntCmp::cmp_gt(u8::MIN, i16::MAX));
        assert!(!IntCmp::cmp_gt(u8::MAX, i16::MAX));
        assert!(!IntCmp::cmp_gt(u8::MAX, u8::MAX as i16));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u8::MIN, u8::MIN as i16));
        assert!(!IntCmp::cmp_le(u8::MIN, i16::MIN));
        assert!(IntCmp::cmp_le(u8::MIN, i16::MAX));
        assert!(IntCmp::cmp_le(u8::MAX, i16::MAX));
        assert!(IntCmp::cmp_le(u8::MAX, u8::MAX as i16));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u8::MIN, u8::MIN as i16));
        assert!(!IntCmp::cmp_lt(u8::MIN, i16::MIN));
        assert!(IntCmp::cmp_lt(u8::MIN, i16::MAX));
        assert!(IntCmp::cmp_lt(u8::MAX, i16::MAX));
        assert!(!IntCmp::cmp_lt(u8::MAX, u8::MAX as i16));
    }
}

mod u8_i32_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u8::MIN, u8::MIN as i32));
        assert!(!IntCmp::cmp_eq(u8::MIN, -1_i32));
        assert!(!IntCmp::cmp_eq(u8::MIN, i32::MIN));
        assert!(!IntCmp::cmp_eq(u8::MAX, i32::MAX));
        assert!(IntCmp::cmp_eq(u8::MAX, u8::MAX as i32));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u8::MIN, u8::MIN as i32));
        assert!(IntCmp::cmp_ne(u8::MIN, -1_i32));
        assert!(IntCmp::cmp_ne(u8::MIN, i32::MIN));
        assert!(IntCmp::cmp_ne(u8::MAX, i32::MAX));
        assert!(!IntCmp::cmp_ne(u8::MAX, u8::MAX as i32));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u8::MIN, u8::MIN as i32));
        assert!(IntCmp::cmp_ge(u8::MIN, i32::MIN));
        assert!(!IntCmp::cmp_ge(u8::MIN, i32::MAX));
        assert!(!IntCmp::cmp_ge(u8::MAX, i32::MAX));
        assert!(IntCmp::cmp_ge(u8::MAX, u8::MAX as i32));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u8::MIN, u8::MIN as i32));
        assert!(IntCmp::cmp_gt(u8::MIN, i32::MIN));
        assert!(!IntCmp::cmp_gt(u8::MIN, i32::MAX));
        assert!(!IntCmp::cmp_gt(u8::MAX, i32::MAX));
        assert!(!IntCmp::cmp_gt(u8::MAX, u8::MAX as i32));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u8::MIN, u8::MIN as i32));
        assert!(!IntCmp::cmp_le(u8::MIN, i32::MIN));
        assert!(IntCmp::cmp_le(u8::MIN, i32::MAX));
        assert!(IntCmp::cmp_le(u8::MAX, i32::MAX));
        assert!(IntCmp::cmp_le(u8::MAX, u8::MAX as i32));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u8::MIN, u8::MIN as i32));
        assert!(!IntCmp::cmp_lt(u8::MIN, i32::MIN));
        assert!(IntCmp::cmp_lt(u8::MIN, i32::MAX));
        assert!(IntCmp::cmp_lt(u8::MAX, i32::MAX));
        assert!(!IntCmp::cmp_lt(u8::MAX, u8::MAX as i32));
    }
}

mod u8_i64_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u8::MIN, u8::MIN as i64));
        assert!(!IntCmp::cmp_eq(u8::MIN, -1_i64));
        assert!(!IntCmp::cmp_eq(u8::MIN, i64::MIN));
        assert!(!IntCmp::cmp_eq(u8::MAX, i64::MAX));
        assert!(IntCmp::cmp_eq(u8::MAX, u8::MAX as i64));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u8::MIN, u8::MIN as i64));
        assert!(IntCmp::cmp_ne(u8::MIN, -1_i64));
        assert!(IntCmp::cmp_ne(u8::MIN, i64::MIN));
        assert!(IntCmp::cmp_ne(u8::MAX, i64::MAX));
        assert!(!IntCmp::cmp_ne(u8::MAX, u8::MAX as i64));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u8::MIN, u8::MIN as i64));
        assert!(IntCmp::cmp_ge(u8::MIN, i64::MIN));
        assert!(!IntCmp::cmp_ge(u8::MIN, i64::MAX));
        assert!(!IntCmp::cmp_ge(u8::MAX, i64::MAX));
        assert!(IntCmp::cmp_ge(u8::MAX, u8::MAX as i64));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u8::MIN, u8::MIN as i64));
        assert!(IntCmp::cmp_gt(u8::MIN, i64::MIN));
        assert!(!IntCmp::cmp_gt(u8::MIN, i64::MAX));
        assert!(!IntCmp::cmp_gt(u8::MAX, i64::MAX));
        assert!(!IntCmp::cmp_gt(u8::MAX, u8::MAX as i64));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u8::MIN, u8::MIN as i64));
        assert!(!IntCmp::cmp_le(u8::MIN, i64::MIN));
        assert!(IntCmp::cmp_le(u8::MIN, i64::MAX));
        assert!(IntCmp::cmp_le(u8::MAX, i64::MAX));
        assert!(IntCmp::cmp_le(u8::MAX, u8::MAX as i64));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u8::MIN, u8::MIN as i64));
        assert!(!IntCmp::cmp_lt(u8::MIN, i64::MIN));
        assert!(IntCmp::cmp_lt(u8::MIN, i64::MAX));
        assert!(IntCmp::cmp_lt(u8::MAX, i64::MAX));
        assert!(!IntCmp::cmp_lt(u8::MAX, u8::MAX as i64));
    }
}

mod u8_i128_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u8::MIN, u8::MIN as i128));
        assert!(!IntCmp::cmp_eq(u8::MIN, -1_i128));
        assert!(!IntCmp::cmp_eq(u8::MIN, i128::MIN));
        assert!(!IntCmp::cmp_eq(u8::MAX, i128::MAX));
        assert!(IntCmp::cmp_eq(u8::MAX, u8::MAX as i128));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u8::MIN, u8::MIN as i128));
        assert!(IntCmp::cmp_ne(u8::MIN, -1_i128));
        assert!(IntCmp::cmp_ne(u8::MIN, i128::MIN));
        assert!(IntCmp::cmp_ne(u8::MAX, i128::MAX));
        assert!(!IntCmp::cmp_ne(u8::MAX, u8::MAX as i128));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u8::MIN, u8::MIN as i128));
        assert!(IntCmp::cmp_ge(u8::MIN, i128::MIN));
        assert!(!IntCmp::cmp_ge(u8::MIN, i128::MAX));
        assert!(!IntCmp::cmp_ge(u8::MAX, i128::MAX));
        assert!(IntCmp::cmp_ge(u8::MAX, u8::MAX as i128));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u8::MIN, u8::MIN as i128));
        assert!(IntCmp::cmp_gt(u8::MIN, i128::MIN));
        assert!(!IntCmp::cmp_gt(u8::MIN, i128::MAX));
        assert!(!IntCmp::cmp_gt(u8::MAX, i128::MAX));
        assert!(!IntCmp::cmp_gt(u8::MAX, u8::MAX as i128));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u8::MIN, u8::MIN as i128));
        assert!(!IntCmp::cmp_le(u8::MIN, i128::MIN));
        assert!(IntCmp::cmp_le(u8::MIN, i128::MAX));
        assert!(IntCmp::cmp_le(u8::MAX, i128::MAX));
        assert!(IntCmp::cmp_le(u8::MAX, u8::MAX as i128));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u8::MIN, u8::MIN as i128));
        assert!(!IntCmp::cmp_lt(u8::MIN, i128::MIN));
        assert!(IntCmp::cmp_lt(u8::MIN, i128::MAX));
        assert!(IntCmp::cmp_lt(u8::MAX, i128::MAX));
        assert!(!IntCmp::cmp_lt(u8::MAX, u8::MAX as i128));
    }
}