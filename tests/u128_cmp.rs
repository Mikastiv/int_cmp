use int_cmp::IntCmp;

mod u128_i8_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u128::MIN, u128::MIN as i8));
        assert!(!IntCmp::cmp_eq(1_u128, -1_i8));
        assert!(!IntCmp::cmp_eq(u128::MIN, i8::MIN));
        assert!(IntCmp::cmp_eq(i8::MAX as u128, i8::MAX));
        assert!(!IntCmp::cmp_eq(u128::MAX, i8::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u128::MIN, u128::MIN as i8));
        assert!(IntCmp::cmp_ne(1_u128, -1_i8));
        assert!(IntCmp::cmp_ne(u128::MIN, i8::MIN));
        assert!(!IntCmp::cmp_ne(i8::MAX as u128, i8::MAX));
        assert!(IntCmp::cmp_ne(u128::MAX, i8::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u128::MIN, u128::MIN as i8));
        assert!(IntCmp::cmp_ge(u128::MIN, -1_i8));
        assert!(IntCmp::cmp_ge(u128::MIN, i8::MIN));
        assert!(IntCmp::cmp_ge(i8::MAX as u128, i8::MAX));
        assert!(IntCmp::cmp_ge(u128::MAX, i8::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u128::MIN, u128::MIN as i8));
        assert!(IntCmp::cmp_gt(u128::MIN, -1_i8));
        assert!(IntCmp::cmp_gt(u128::MIN, i8::MIN));
        assert!(!IntCmp::cmp_gt(i8::MAX as u128, i8::MAX));
        assert!(IntCmp::cmp_gt(u128::MAX, i8::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u128::MIN, u128::MIN as i8));
        assert!(!IntCmp::cmp_le(u128::MIN, -1_i8));
        assert!(!IntCmp::cmp_le(u128::MIN, i8::MIN));
        assert!(IntCmp::cmp_le(i8::MAX as u128, i8::MAX));
        assert!(!IntCmp::cmp_le(u128::MAX, i8::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u128::MIN, u128::MIN as i8));
        assert!(!IntCmp::cmp_lt(u128::MIN, -1_i8));
        assert!(!IntCmp::cmp_lt(u128::MIN, i8::MIN));
        assert!(!IntCmp::cmp_lt(i8::MAX as u128, i8::MAX));
        assert!(!IntCmp::cmp_lt(u128::MAX, i8::MAX));
    }
}

mod u128_i16_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u128::MIN, u128::MIN as i16));
        assert!(!IntCmp::cmp_eq(1_u128, -1_i16));
        assert!(!IntCmp::cmp_eq(u128::MIN, i16::MIN));
        assert!(IntCmp::cmp_eq(i16::MAX as u128, i16::MAX));
        assert!(!IntCmp::cmp_eq(u128::MAX, i16::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u128::MIN, u128::MIN as i16));
        assert!(IntCmp::cmp_ne(1_u128, -1_i16));
        assert!(IntCmp::cmp_ne(u128::MIN, i16::MIN));
        assert!(!IntCmp::cmp_ne(i16::MAX as u128, i16::MAX));
        assert!(IntCmp::cmp_ne(u128::MAX, i16::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u128::MIN, u128::MIN as i16));
        assert!(IntCmp::cmp_ge(u128::MIN, -1_i16));
        assert!(IntCmp::cmp_ge(u128::MIN, i16::MIN));
        assert!(IntCmp::cmp_ge(i16::MAX as u128, i16::MAX));
        assert!(IntCmp::cmp_ge(u128::MAX, i16::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u128::MIN, u128::MIN as i16));
        assert!(IntCmp::cmp_gt(u128::MIN, -1_i16));
        assert!(IntCmp::cmp_gt(u128::MIN, i16::MIN));
        assert!(!IntCmp::cmp_gt(i16::MAX as u128, i16::MAX));
        assert!(IntCmp::cmp_gt(u128::MAX, i16::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u128::MIN, u128::MIN as i16));
        assert!(!IntCmp::cmp_le(u128::MIN, -1_i16));
        assert!(!IntCmp::cmp_le(u128::MIN, i16::MIN));
        assert!(IntCmp::cmp_le(i16::MAX as u128, i16::MAX));
        assert!(!IntCmp::cmp_le(u128::MAX, i16::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u128::MIN, u128::MIN as i16));
        assert!(!IntCmp::cmp_lt(u128::MIN, -1_i16));
        assert!(!IntCmp::cmp_lt(u128::MIN, i16::MIN));
        assert!(!IntCmp::cmp_lt(i16::MAX as u128, i16::MAX));
        assert!(!IntCmp::cmp_lt(u128::MAX, i16::MAX));
    }
}

mod u128_i32_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u128::MIN, u128::MIN as i32));
        assert!(!IntCmp::cmp_eq(1_u128, -1_i32));
        assert!(!IntCmp::cmp_eq(u128::MIN, i32::MIN));
        assert!(IntCmp::cmp_eq(i32::MAX as u128, i32::MAX));
        assert!(!IntCmp::cmp_eq(u128::MAX, i32::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u128::MIN, u128::MIN as i32));
        assert!(IntCmp::cmp_ne(1_u128, -1_i32));
        assert!(IntCmp::cmp_ne(u128::MIN, i32::MIN));
        assert!(!IntCmp::cmp_ne(i32::MAX as u128, i32::MAX));
        assert!(IntCmp::cmp_ne(u128::MAX, i32::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u128::MIN, u128::MIN as i32));
        assert!(IntCmp::cmp_ge(u128::MIN, -1_i32));
        assert!(IntCmp::cmp_ge(u128::MIN, i32::MIN));
        assert!(IntCmp::cmp_ge(i32::MAX as u128, i32::MAX));
        assert!(IntCmp::cmp_ge(u128::MAX, i32::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u128::MIN, u128::MIN as i32));
        assert!(IntCmp::cmp_gt(u128::MIN, -1_i32));
        assert!(IntCmp::cmp_gt(u128::MIN, i32::MIN));
        assert!(!IntCmp::cmp_gt(i32::MAX as u128, i32::MAX));
        assert!(IntCmp::cmp_gt(u128::MAX, i32::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u128::MIN, u128::MIN as i32));
        assert!(!IntCmp::cmp_le(u128::MIN, -1_i32));
        assert!(!IntCmp::cmp_le(u128::MIN, i32::MIN));
        assert!(IntCmp::cmp_le(i32::MAX as u128, i32::MAX));
        assert!(!IntCmp::cmp_le(u128::MAX, i32::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u128::MIN, u128::MIN as i32));
        assert!(!IntCmp::cmp_lt(u128::MIN, -1_i32));
        assert!(!IntCmp::cmp_lt(u128::MIN, i32::MIN));
        assert!(!IntCmp::cmp_lt(i32::MAX as u128, i32::MAX));
        assert!(!IntCmp::cmp_lt(u128::MAX, i32::MAX));
    }
}

mod u128_i64_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u128::MIN, u128::MIN as i64));
        assert!(!IntCmp::cmp_eq(1_u128, -1_i64));
        assert!(!IntCmp::cmp_eq(u128::MIN, i64::MIN));
        assert!(IntCmp::cmp_eq(i64::MAX as u128, i64::MAX));
        assert!(!IntCmp::cmp_eq(u128::MAX, i64::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u128::MIN, u128::MIN as i64));
        assert!(IntCmp::cmp_ne(1_u128, -1_i64));
        assert!(IntCmp::cmp_ne(u128::MIN, i64::MIN));
        assert!(!IntCmp::cmp_ne(i64::MAX as u128, i64::MAX));
        assert!(IntCmp::cmp_ne(u128::MAX, i64::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u128::MIN, u128::MIN as i64));
        assert!(IntCmp::cmp_ge(u128::MIN, -1_i64));
        assert!(IntCmp::cmp_ge(u128::MIN, i64::MIN));
        assert!(IntCmp::cmp_ge(i64::MAX as u128, i64::MAX));
        assert!(IntCmp::cmp_ge(u128::MAX, i64::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u128::MIN, u128::MIN as i64));
        assert!(IntCmp::cmp_gt(u128::MIN, -1_i64));
        assert!(IntCmp::cmp_gt(u128::MIN, i64::MIN));
        assert!(!IntCmp::cmp_gt(i64::MAX as u128, i64::MAX));
        assert!(IntCmp::cmp_gt(u128::MAX, i64::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u128::MIN, u128::MIN as i64));
        assert!(!IntCmp::cmp_le(u128::MIN, -1_i64));
        assert!(!IntCmp::cmp_le(u128::MIN, i64::MIN));
        assert!(IntCmp::cmp_le(i64::MAX as u128, i64::MAX));
        assert!(!IntCmp::cmp_le(u128::MAX, i64::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u128::MIN, u128::MIN as i64));
        assert!(!IntCmp::cmp_lt(u128::MIN, -1_i64));
        assert!(!IntCmp::cmp_lt(u128::MIN, i64::MIN));
        assert!(!IntCmp::cmp_lt(i64::MAX as u128, i64::MAX));
        assert!(!IntCmp::cmp_lt(u128::MAX, i64::MAX));
    }
}

mod u128_i128_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u128::MIN, u128::MIN as i128));
        assert!(!IntCmp::cmp_eq(u128::MIN, -1_i128));
        assert!(!IntCmp::cmp_eq(u128::MIN, i128::MIN));
        assert!(!IntCmp::cmp_eq(u128::MAX, i128::MAX));
        assert!(IntCmp::cmp_eq(i128::MAX as u128, i128::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u128::MIN, u128::MIN as i128));
        assert!(IntCmp::cmp_ne(u128::MIN, -1_i128));
        assert!(IntCmp::cmp_ne(u128::MIN, i128::MIN));
        assert!(IntCmp::cmp_ne(u128::MAX, i128::MAX));
        assert!(!IntCmp::cmp_ne(i128::MAX as u128, i128::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u128::MIN, u128::MIN as i128));
        assert!(IntCmp::cmp_ge(u128::MIN, i128::MIN));
        assert!(!IntCmp::cmp_ge(u128::MIN, i128::MAX));
        assert!(IntCmp::cmp_ge(u128::MAX, i128::MAX));
        assert!(IntCmp::cmp_ge(i128::MAX as u128, i128::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u128::MIN, u128::MIN as i128));
        assert!(IntCmp::cmp_gt(u128::MIN, i128::MIN));
        assert!(!IntCmp::cmp_gt(u128::MIN, i128::MAX));
        assert!(IntCmp::cmp_gt(u128::MAX, i128::MAX));
        assert!(!IntCmp::cmp_gt(i128::MAX as u128, i128::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u128::MIN, u128::MIN as i128));
        assert!(!IntCmp::cmp_le(u128::MIN, i128::MIN));
        assert!(IntCmp::cmp_le(u128::MIN, i128::MAX));
        assert!(!IntCmp::cmp_le(u128::MAX, i128::MAX));
        assert!(IntCmp::cmp_le(i128::MAX as u128, i128::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u128::MIN, u128::MIN as i128));
        assert!(!IntCmp::cmp_lt(u128::MIN, i128::MIN));
        assert!(IntCmp::cmp_lt(u128::MIN, i128::MAX));
        assert!(!IntCmp::cmp_lt(u128::MAX, i128::MAX));
        assert!(!IntCmp::cmp_lt(i128::MAX as u128, i128::MAX));
    }
}