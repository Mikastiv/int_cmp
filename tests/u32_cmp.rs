use int_cmp::IntCmp;

mod u32_i8_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u32::MIN, u32::MIN as i8));
        assert!(!IntCmp::cmp_eq(1_u32, -1_i8));
        assert!(!IntCmp::cmp_eq(u32::MIN, i8::MIN));
        assert!(IntCmp::cmp_eq(i8::MAX as u32, i8::MAX));
        assert!(!IntCmp::cmp_eq(u32::MAX, i8::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u32::MIN, u32::MIN as i8));
        assert!(IntCmp::cmp_ne(1_u32, -1_i8));
        assert!(IntCmp::cmp_ne(u32::MIN, i8::MIN));
        assert!(!IntCmp::cmp_ne(i8::MAX as u32, i8::MAX));
        assert!(IntCmp::cmp_ne(u32::MAX, i8::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u32::MIN, u32::MIN as i8));
        assert!(IntCmp::cmp_ge(u32::MIN, -1_i8));
        assert!(IntCmp::cmp_ge(u32::MIN, i8::MIN));
        assert!(IntCmp::cmp_ge(i8::MAX as u32, i8::MAX));
        assert!(IntCmp::cmp_ge(u32::MAX, i8::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u32::MIN, u32::MIN as i8));
        assert!(IntCmp::cmp_gt(u32::MIN, -1_i8));
        assert!(IntCmp::cmp_gt(u32::MIN, i8::MIN));
        assert!(!IntCmp::cmp_gt(i8::MAX as u32, i8::MAX));
        assert!(IntCmp::cmp_gt(u32::MAX, i8::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u32::MIN, u32::MIN as i8));
        assert!(!IntCmp::cmp_le(u32::MIN, -1_i8));
        assert!(!IntCmp::cmp_le(u32::MIN, i8::MIN));
        assert!(IntCmp::cmp_le(i8::MAX as u32, i8::MAX));
        assert!(!IntCmp::cmp_le(u32::MAX, i8::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u32::MIN, u32::MIN as i8));
        assert!(!IntCmp::cmp_lt(u32::MIN, -1_i8));
        assert!(!IntCmp::cmp_lt(u32::MIN, i8::MIN));
        assert!(!IntCmp::cmp_lt(i8::MAX as u32, i8::MAX));
        assert!(!IntCmp::cmp_lt(u32::MAX, i8::MAX));
    }
}

mod u32_i16_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u32::MIN, u32::MIN as i16));
        assert!(!IntCmp::cmp_eq(1_u32, -1_i16));
        assert!(!IntCmp::cmp_eq(u32::MIN, i16::MIN));
        assert!(IntCmp::cmp_eq(i16::MAX as u32, i16::MAX));
        assert!(!IntCmp::cmp_eq(u32::MAX, i16::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u32::MIN, u32::MIN as i16));
        assert!(IntCmp::cmp_ne(1_u32, -1_i16));
        assert!(IntCmp::cmp_ne(u32::MIN, i16::MIN));
        assert!(!IntCmp::cmp_ne(i16::MAX as u32, i16::MAX));
        assert!(IntCmp::cmp_ne(u32::MAX, i16::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u32::MIN, u32::MIN as i16));
        assert!(IntCmp::cmp_ge(u32::MIN, -1_i16));
        assert!(IntCmp::cmp_ge(u32::MIN, i16::MIN));
        assert!(IntCmp::cmp_ge(i16::MAX as u32, i16::MAX));
        assert!(IntCmp::cmp_ge(u32::MAX, i16::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u32::MIN, u32::MIN as i16));
        assert!(IntCmp::cmp_gt(u32::MIN, -1_i16));
        assert!(IntCmp::cmp_gt(u32::MIN, i16::MIN));
        assert!(!IntCmp::cmp_gt(i16::MAX as u32, i16::MAX));
        assert!(IntCmp::cmp_gt(u32::MAX, i16::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u32::MIN, u32::MIN as i16));
        assert!(!IntCmp::cmp_le(u32::MIN, -1_i16));
        assert!(!IntCmp::cmp_le(u32::MIN, i16::MIN));
        assert!(IntCmp::cmp_le(i16::MAX as u32, i16::MAX));
        assert!(!IntCmp::cmp_le(u32::MAX, i16::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u32::MIN, u32::MIN as i16));
        assert!(!IntCmp::cmp_lt(u32::MIN, -1_i16));
        assert!(!IntCmp::cmp_lt(u32::MIN, i16::MIN));
        assert!(!IntCmp::cmp_lt(i16::MAX as u32, i16::MAX));
        assert!(!IntCmp::cmp_lt(u32::MAX, i16::MAX));
    }
}

mod u32_i32_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u32::MIN, u32::MIN as i32));
        assert!(!IntCmp::cmp_eq(u32::MIN, -1_i32));
        assert!(!IntCmp::cmp_eq(u32::MIN, i32::MIN));
        assert!(!IntCmp::cmp_eq(u32::MAX, i32::MAX));
        assert!(IntCmp::cmp_eq(i32::MAX as u32, i32::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u32::MIN, u32::MIN as i32));
        assert!(IntCmp::cmp_ne(u32::MIN, -1_i32));
        assert!(IntCmp::cmp_ne(u32::MIN, i32::MIN));
        assert!(IntCmp::cmp_ne(u32::MAX, i32::MAX));
        assert!(!IntCmp::cmp_ne(i32::MAX as u32, i32::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u32::MIN, u32::MIN as i32));
        assert!(IntCmp::cmp_ge(u32::MIN, i32::MIN));
        assert!(!IntCmp::cmp_ge(u32::MIN, i32::MAX));
        assert!(IntCmp::cmp_ge(u32::MAX, i32::MAX));
        assert!(IntCmp::cmp_ge(i32::MAX as u32, i32::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u32::MIN, u32::MIN as i32));
        assert!(IntCmp::cmp_gt(u32::MIN, i32::MIN));
        assert!(!IntCmp::cmp_gt(u32::MIN, i32::MAX));
        assert!(IntCmp::cmp_gt(u32::MAX, i32::MAX));
        assert!(!IntCmp::cmp_gt(i32::MAX as u32, i32::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u32::MIN, u32::MIN as i32));
        assert!(!IntCmp::cmp_le(u32::MIN, i32::MIN));
        assert!(IntCmp::cmp_le(u32::MIN, i32::MAX));
        assert!(!IntCmp::cmp_le(u32::MAX, i32::MAX));
        assert!(IntCmp::cmp_le(i32::MAX as u32, i32::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u32::MIN, u32::MIN as i32));
        assert!(!IntCmp::cmp_lt(u32::MIN, i32::MIN));
        assert!(IntCmp::cmp_lt(u32::MIN, i32::MAX));
        assert!(!IntCmp::cmp_lt(u32::MAX, i32::MAX));
        assert!(!IntCmp::cmp_lt(i32::MAX as u32, i32::MAX));
    }
}

mod u32_i64_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u32::MIN, u32::MIN as i64));
        assert!(!IntCmp::cmp_eq(u32::MIN, -1_i64));
        assert!(!IntCmp::cmp_eq(u32::MIN, i64::MIN));
        assert!(!IntCmp::cmp_eq(u32::MAX, i64::MAX));
        assert!(IntCmp::cmp_eq(u32::MAX, u32::MAX as i64));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u32::MIN, u32::MIN as i64));
        assert!(IntCmp::cmp_ne(u32::MIN, -1_i64));
        assert!(IntCmp::cmp_ne(u32::MIN, i64::MIN));
        assert!(IntCmp::cmp_ne(u32::MAX, i64::MAX));
        assert!(!IntCmp::cmp_ne(u32::MAX, u32::MAX as i64));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u32::MIN, u32::MIN as i64));
        assert!(IntCmp::cmp_ge(u32::MIN, i64::MIN));
        assert!(!IntCmp::cmp_ge(u32::MIN, i64::MAX));
        assert!(!IntCmp::cmp_ge(u32::MAX, i64::MAX));
        assert!(IntCmp::cmp_ge(u32::MAX, u32::MAX as i64));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u32::MIN, u32::MIN as i64));
        assert!(IntCmp::cmp_gt(u32::MIN, i64::MIN));
        assert!(!IntCmp::cmp_gt(u32::MIN, i64::MAX));
        assert!(!IntCmp::cmp_gt(u32::MAX, i64::MAX));
        assert!(!IntCmp::cmp_gt(u32::MAX, u32::MAX as i64));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u32::MIN, u32::MIN as i64));
        assert!(!IntCmp::cmp_le(u32::MIN, i64::MIN));
        assert!(IntCmp::cmp_le(u32::MIN, i64::MAX));
        assert!(IntCmp::cmp_le(u32::MAX, i64::MAX));
        assert!(IntCmp::cmp_le(u32::MAX, u32::MAX as i64));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u32::MIN, u32::MIN as i64));
        assert!(!IntCmp::cmp_lt(u32::MIN, i64::MIN));
        assert!(IntCmp::cmp_lt(u32::MIN, i64::MAX));
        assert!(IntCmp::cmp_lt(u32::MAX, i64::MAX));
        assert!(!IntCmp::cmp_lt(u32::MAX, u32::MAX as i64));
    }
}

mod u32_i128_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u32::MIN, u32::MIN as i128));
        assert!(!IntCmp::cmp_eq(u32::MIN, -1_i128));
        assert!(!IntCmp::cmp_eq(u32::MIN, i128::MIN));
        assert!(!IntCmp::cmp_eq(u32::MAX, i128::MAX));
        assert!(IntCmp::cmp_eq(u32::MAX, u32::MAX as i128));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u32::MIN, u32::MIN as i128));
        assert!(IntCmp::cmp_ne(u32::MIN, -1_i128));
        assert!(IntCmp::cmp_ne(u32::MIN, i128::MIN));
        assert!(IntCmp::cmp_ne(u32::MAX, i128::MAX));
        assert!(!IntCmp::cmp_ne(u32::MAX, u32::MAX as i128));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u32::MIN, u32::MIN as i128));
        assert!(IntCmp::cmp_ge(u32::MIN, i128::MIN));
        assert!(!IntCmp::cmp_ge(u32::MIN, i128::MAX));
        assert!(!IntCmp::cmp_ge(u32::MAX, i128::MAX));
        assert!(IntCmp::cmp_ge(u32::MAX, u32::MAX as i128));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u32::MIN, u32::MIN as i128));
        assert!(IntCmp::cmp_gt(u32::MIN, i128::MIN));
        assert!(!IntCmp::cmp_gt(u32::MIN, i128::MAX));
        assert!(!IntCmp::cmp_gt(u32::MAX, i128::MAX));
        assert!(!IntCmp::cmp_gt(u32::MAX, u32::MAX as i128));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u32::MIN, u32::MIN as i128));
        assert!(!IntCmp::cmp_le(u32::MIN, i128::MIN));
        assert!(IntCmp::cmp_le(u32::MIN, i128::MAX));
        assert!(IntCmp::cmp_le(u32::MAX, i128::MAX));
        assert!(IntCmp::cmp_le(u32::MAX, u32::MAX as i128));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u32::MIN, u32::MIN as i128));
        assert!(!IntCmp::cmp_lt(u32::MIN, i128::MIN));
        assert!(IntCmp::cmp_lt(u32::MIN, i128::MAX));
        assert!(IntCmp::cmp_lt(u32::MAX, i128::MAX));
        assert!(!IntCmp::cmp_lt(u32::MAX, u32::MAX as i128));
    }
}