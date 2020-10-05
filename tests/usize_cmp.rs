use int_cmp::IntCmp;

mod usize_i8_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(usize::MIN, usize::MIN as i8));
        assert!(!IntCmp::cmp_eq(1_usize, -1_i8));
        assert!(!IntCmp::cmp_eq(usize::MIN, i8::MIN));
        assert!(IntCmp::cmp_eq(i8::MAX as usize, i8::MAX));
        assert!(!IntCmp::cmp_eq(usize::MAX, i8::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(usize::MIN, usize::MIN as i8));
        assert!(IntCmp::cmp_ne(1_usize, -1_i8));
        assert!(IntCmp::cmp_ne(usize::MIN, i8::MIN));
        assert!(!IntCmp::cmp_ne(i8::MAX as usize, i8::MAX));
        assert!(IntCmp::cmp_ne(usize::MAX, i8::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(usize::MIN, usize::MIN as i8));
        assert!(IntCmp::cmp_ge(usize::MIN, -1_i8));
        assert!(IntCmp::cmp_ge(usize::MIN, i8::MIN));
        assert!(IntCmp::cmp_ge(i8::MAX as usize, i8::MAX));
        assert!(IntCmp::cmp_ge(usize::MAX, i8::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(usize::MIN, usize::MIN as i8));
        assert!(IntCmp::cmp_gt(usize::MIN, -1_i8));
        assert!(IntCmp::cmp_gt(usize::MIN, i8::MIN));
        assert!(!IntCmp::cmp_gt(i8::MAX as usize, i8::MAX));
        assert!(IntCmp::cmp_gt(usize::MAX, i8::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(usize::MIN, usize::MIN as i8));
        assert!(!IntCmp::cmp_le(usize::MIN, -1_i8));
        assert!(!IntCmp::cmp_le(usize::MIN, i8::MIN));
        assert!(IntCmp::cmp_le(i8::MAX as usize, i8::MAX));
        assert!(!IntCmp::cmp_le(usize::MAX, i8::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(usize::MIN, usize::MIN as i8));
        assert!(!IntCmp::cmp_lt(usize::MIN, -1_i8));
        assert!(!IntCmp::cmp_lt(usize::MIN, i8::MIN));
        assert!(!IntCmp::cmp_lt(i8::MAX as usize, i8::MAX));
        assert!(!IntCmp::cmp_lt(usize::MAX, i8::MAX));
    }
}

mod usize_i16_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(usize::MIN, usize::MIN as i16));
        assert!(!IntCmp::cmp_eq(1_usize, -1_i16));
        assert!(!IntCmp::cmp_eq(usize::MIN, i16::MIN));
        assert!(IntCmp::cmp_eq(i16::MAX as usize, i16::MAX));
        assert!(!IntCmp::cmp_eq(usize::MAX, i16::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(usize::MIN, usize::MIN as i16));
        assert!(IntCmp::cmp_ne(1_usize, -1_i16));
        assert!(IntCmp::cmp_ne(usize::MIN, i16::MIN));
        assert!(!IntCmp::cmp_ne(i16::MAX as usize, i16::MAX));
        assert!(IntCmp::cmp_ne(usize::MAX, i16::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(usize::MIN, usize::MIN as i16));
        assert!(IntCmp::cmp_ge(usize::MIN, -1_i16));
        assert!(IntCmp::cmp_ge(usize::MIN, i16::MIN));
        assert!(IntCmp::cmp_ge(i16::MAX as usize, i16::MAX));
        assert!(IntCmp::cmp_ge(usize::MAX, i16::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(usize::MIN, usize::MIN as i16));
        assert!(IntCmp::cmp_gt(usize::MIN, -1_i16));
        assert!(IntCmp::cmp_gt(usize::MIN, i16::MIN));
        assert!(!IntCmp::cmp_gt(i16::MAX as usize, i16::MAX));
        assert!(IntCmp::cmp_gt(usize::MAX, i16::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(usize::MIN, usize::MIN as i16));
        assert!(!IntCmp::cmp_le(usize::MIN, -1_i16));
        assert!(!IntCmp::cmp_le(usize::MIN, i16::MIN));
        assert!(IntCmp::cmp_le(i16::MAX as usize, i16::MAX));
        assert!(!IntCmp::cmp_le(usize::MAX, i16::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(usize::MIN, usize::MIN as i16));
        assert!(!IntCmp::cmp_lt(usize::MIN, -1_i16));
        assert!(!IntCmp::cmp_lt(usize::MIN, i16::MIN));
        assert!(!IntCmp::cmp_lt(i16::MAX as usize, i16::MAX));
        assert!(!IntCmp::cmp_lt(usize::MAX, i16::MAX));
    }
}

mod usize_i32_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(usize::MIN, usize::MIN as i32));
        assert!(!IntCmp::cmp_eq(usize::MIN, -1_i32));
        assert!(!IntCmp::cmp_eq(usize::MIN, i32::MIN));
        assert!(!IntCmp::cmp_eq(usize::MAX, i32::MAX));
        assert!(IntCmp::cmp_eq(i32::MAX as usize, i32::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(usize::MIN, usize::MIN as i32));
        assert!(IntCmp::cmp_ne(usize::MIN, -1_i32));
        assert!(IntCmp::cmp_ne(usize::MIN, i32::MIN));
        assert!(IntCmp::cmp_ne(usize::MAX, i32::MAX));
        assert!(!IntCmp::cmp_ne(i32::MAX as usize, i32::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(usize::MIN, usize::MIN as i32));
        assert!(IntCmp::cmp_ge(usize::MIN, i32::MIN));
        assert!(!IntCmp::cmp_ge(usize::MIN, i32::MAX));
        assert!(IntCmp::cmp_ge(usize::MAX, i32::MAX));
        assert!(IntCmp::cmp_ge(i32::MAX as usize, i32::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(usize::MIN, usize::MIN as i32));
        assert!(IntCmp::cmp_gt(usize::MIN, i32::MIN));
        assert!(!IntCmp::cmp_gt(usize::MIN, i32::MAX));
        assert!(IntCmp::cmp_gt(usize::MAX, i32::MAX));
        assert!(!IntCmp::cmp_gt(i32::MAX as usize, i32::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(usize::MIN, usize::MIN as i32));
        assert!(!IntCmp::cmp_le(usize::MIN, i32::MIN));
        assert!(IntCmp::cmp_le(usize::MIN, i32::MAX));
        assert!(!IntCmp::cmp_le(usize::MAX, i32::MAX));
        assert!(IntCmp::cmp_le(i32::MAX as usize, i32::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(usize::MIN, usize::MIN as i32));
        assert!(!IntCmp::cmp_lt(usize::MIN, i32::MIN));
        assert!(IntCmp::cmp_lt(usize::MIN, i32::MAX));
        assert!(!IntCmp::cmp_lt(usize::MAX, i32::MAX));
        assert!(!IntCmp::cmp_lt(i32::MAX as usize, i32::MAX));
    }
}

#[cfg(target_pointer_width = "32")]
mod usize_i64_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(usize::MIN, usize::MIN as i64));
        assert!(!IntCmp::cmp_eq(usize::MIN, -1_i64));
        assert!(!IntCmp::cmp_eq(usize::MIN, i64::MIN));
        assert!(!IntCmp::cmp_eq(usize::MAX, i64::MAX));
        assert!(IntCmp::cmp_eq(usize::MAX, usize::MAX as i64));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(usize::MIN, usize::MIN as i64));
        assert!(IntCmp::cmp_ne(usize::MIN, -1_i64));
        assert!(IntCmp::cmp_ne(usize::MIN, i64::MIN));
        assert!(IntCmp::cmp_ne(usize::MAX, i64::MAX));
        assert!(!IntCmp::cmp_ne(usize::MAX, usize::MAX as i64));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(usize::MIN, usize::MIN as i64));
        assert!(IntCmp::cmp_ge(usize::MIN, i64::MIN));
        assert!(!IntCmp::cmp_ge(usize::MIN, i64::MAX));
        assert!(!IntCmp::cmp_ge(usize::MAX, i64::MAX));
        assert!(IntCmp::cmp_ge(usize::MAX, usize::MAX as i64));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(usize::MIN, usize::MIN as i64));
        assert!(IntCmp::cmp_gt(usize::MIN, i64::MIN));
        assert!(!IntCmp::cmp_gt(usize::MIN, i64::MAX));
        assert!(!IntCmp::cmp_gt(usize::MAX, i64::MAX));
        assert!(!IntCmp::cmp_gt(usize::MAX, usize::MAX as i64));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(usize::MIN, usize::MIN as i64));
        assert!(!IntCmp::cmp_le(usize::MIN, i64::MIN));
        assert!(IntCmp::cmp_le(usize::MIN, i64::MAX));
        assert!(IntCmp::cmp_le(usize::MAX, i64::MAX));
        assert!(IntCmp::cmp_le(usize::MAX, usize::MAX as i64));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(usize::MIN, usize::MIN as i64));
        assert!(!IntCmp::cmp_lt(usize::MIN, i64::MIN));
        assert!(IntCmp::cmp_lt(usize::MIN, i64::MAX));
        assert!(IntCmp::cmp_lt(usize::MAX, i64::MAX));
        assert!(!IntCmp::cmp_lt(usize::MAX, usize::MAX as i64));
    }
}

#[cfg(target_pointer_width = "64")]
mod usize_i64_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(usize::MIN, usize::MIN as i64));
        assert!(!IntCmp::cmp_eq(usize::MIN, -1_i64));
        assert!(!IntCmp::cmp_eq(usize::MIN, i64::MIN));
        assert!(!IntCmp::cmp_eq(usize::MAX, i64::MAX));
        assert!(IntCmp::cmp_eq(i64::MAX as usize, i64::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(usize::MIN, usize::MIN as i64));
        assert!(IntCmp::cmp_ne(usize::MIN, -1_i64));
        assert!(IntCmp::cmp_ne(usize::MIN, i64::MIN));
        assert!(IntCmp::cmp_ne(usize::MAX, i64::MAX));
        assert!(!IntCmp::cmp_ne(i64::MAX as usize, i64::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(usize::MIN, usize::MIN as i64));
        assert!(IntCmp::cmp_ge(usize::MIN, i64::MIN));
        assert!(!IntCmp::cmp_ge(usize::MIN, i64::MAX));
        assert!(IntCmp::cmp_ge(usize::MAX, i64::MAX));
        assert!(IntCmp::cmp_ge(i64::MAX as usize, i64::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(usize::MIN, usize::MIN as i64));
        assert!(IntCmp::cmp_gt(usize::MIN, i64::MIN));
        assert!(!IntCmp::cmp_gt(usize::MIN, i64::MAX));
        assert!(IntCmp::cmp_gt(usize::MAX, i64::MAX));
        assert!(!IntCmp::cmp_gt(i64::MAX as usize, i64::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(usize::MIN, usize::MIN as i64));
        assert!(!IntCmp::cmp_le(usize::MIN, i64::MIN));
        assert!(IntCmp::cmp_le(usize::MIN, i64::MAX));
        assert!(!IntCmp::cmp_le(usize::MAX, i64::MAX));
        assert!(IntCmp::cmp_le(i64::MAX as usize, i64::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(usize::MIN, usize::MIN as i64));
        assert!(!IntCmp::cmp_lt(usize::MIN, i64::MIN));
        assert!(IntCmp::cmp_lt(usize::MIN, i64::MAX));
        assert!(!IntCmp::cmp_lt(usize::MAX, i64::MAX));
        assert!(!IntCmp::cmp_lt(i64::MAX as usize, i64::MAX));
    }
}

mod usize_i128_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(usize::MIN, usize::MIN as i128));
        assert!(!IntCmp::cmp_eq(usize::MIN, -1_i128));
        assert!(!IntCmp::cmp_eq(usize::MIN, i128::MIN));
        assert!(!IntCmp::cmp_eq(usize::MAX, i128::MAX));
        assert!(IntCmp::cmp_eq(usize::MAX, usize::MAX as i128));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(usize::MIN, usize::MIN as i128));
        assert!(IntCmp::cmp_ne(usize::MIN, -1_i128));
        assert!(IntCmp::cmp_ne(usize::MIN, i128::MIN));
        assert!(IntCmp::cmp_ne(usize::MAX, i128::MAX));
        assert!(!IntCmp::cmp_ne(usize::MAX, usize::MAX as i128));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(usize::MIN, usize::MIN as i128));
        assert!(IntCmp::cmp_ge(usize::MIN, i128::MIN));
        assert!(!IntCmp::cmp_ge(usize::MIN, i128::MAX));
        assert!(!IntCmp::cmp_ge(usize::MAX, i128::MAX));
        assert!(IntCmp::cmp_ge(usize::MAX, usize::MAX as i128));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(usize::MIN, usize::MIN as i128));
        assert!(IntCmp::cmp_gt(usize::MIN, i128::MIN));
        assert!(!IntCmp::cmp_gt(usize::MIN, i128::MAX));
        assert!(!IntCmp::cmp_gt(usize::MAX, i128::MAX));
        assert!(!IntCmp::cmp_gt(usize::MAX, usize::MAX as i128));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(usize::MIN, usize::MIN as i128));
        assert!(!IntCmp::cmp_le(usize::MIN, i128::MIN));
        assert!(IntCmp::cmp_le(usize::MIN, i128::MAX));
        assert!(IntCmp::cmp_le(usize::MAX, i128::MAX));
        assert!(IntCmp::cmp_le(usize::MAX, usize::MAX as i128));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(usize::MIN, usize::MIN as i128));
        assert!(!IntCmp::cmp_lt(usize::MIN, i128::MIN));
        assert!(IntCmp::cmp_lt(usize::MIN, i128::MAX));
        assert!(IntCmp::cmp_lt(usize::MAX, i128::MAX));
        assert!(!IntCmp::cmp_lt(usize::MAX, usize::MAX as i128));
    }
}
