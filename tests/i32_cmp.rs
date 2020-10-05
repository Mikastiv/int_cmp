use int_cmp::IntCmp;

mod i32_u8_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u8::MIN as i32, u8::MIN));
        assert!(!IntCmp::cmp_eq(-1_i32, u8::MIN));
        assert!(!IntCmp::cmp_eq(i32::MIN, u8::MIN));
        assert!(!IntCmp::cmp_eq(i32::MAX, u8::MAX));
        assert!(IntCmp::cmp_eq(u8::MAX as i32, u8::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u8::MIN as i32, u8::MIN));
        assert!(IntCmp::cmp_ne(-1_i32, u8::MIN));
        assert!(IntCmp::cmp_ne(i32::MIN, u8::MIN));
        assert!(IntCmp::cmp_ne(i32::MAX, u8::MAX));
        assert!(!IntCmp::cmp_ne(u8::MAX as i32, u8::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u8::MIN as i32, u8::MIN));
        assert!(!IntCmp::cmp_ge(i32::MIN, u8::MIN));
        assert!(IntCmp::cmp_ge(i32::MAX, u8::MIN));
        assert!(IntCmp::cmp_ge(i32::MAX, u8::MAX));
        assert!(IntCmp::cmp_ge(u8::MAX as i32, u8::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u8::MIN as i32, u8::MIN));
        assert!(!IntCmp::cmp_gt(i32::MIN, u8::MIN));
        assert!(IntCmp::cmp_gt(i32::MAX, u8::MIN));
        assert!(IntCmp::cmp_gt(i32::MAX, u8::MAX));
        assert!(!IntCmp::cmp_gt(u8::MAX as i32, u8::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u8::MIN as i32, u8::MIN));
        assert!(IntCmp::cmp_le(i32::MIN, u8::MIN));
        assert!(!IntCmp::cmp_le(i32::MAX, u8::MIN));
        assert!(!IntCmp::cmp_le(i32::MAX, u8::MAX));
        assert!(IntCmp::cmp_le(u8::MAX as i32, u8::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u8::MIN as i32, u8::MIN));
        assert!(IntCmp::cmp_lt(i32::MIN, u8::MIN));
        assert!(!IntCmp::cmp_lt(i32::MAX, u8::MIN));
        assert!(!IntCmp::cmp_lt(i32::MAX, u8::MAX));
        assert!(!IntCmp::cmp_lt(u8::MAX as i32, u8::MAX));
    }
}

mod i32_u16_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u16::MIN as i32, u16::MIN));
        assert!(!IntCmp::cmp_eq(-1_i32, u16::MIN));
        assert!(!IntCmp::cmp_eq(i32::MIN, u16::MIN));
        assert!(!IntCmp::cmp_eq(i32::MAX, u16::MAX));
        assert!(IntCmp::cmp_eq(u16::MAX as i32, u16::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u16::MIN as i32, u16::MIN));
        assert!(IntCmp::cmp_ne(-1_i32, u16::MIN));
        assert!(IntCmp::cmp_ne(i32::MIN, u16::MIN));
        assert!(IntCmp::cmp_ne(i32::MAX, u16::MAX));
        assert!(!IntCmp::cmp_ne(u16::MAX as i32, u16::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u16::MIN as i32, u16::MIN));
        assert!(!IntCmp::cmp_ge(i32::MIN, u16::MIN));
        assert!(IntCmp::cmp_ge(i32::MAX, u16::MIN));
        assert!(IntCmp::cmp_ge(i32::MAX, u16::MAX));
        assert!(IntCmp::cmp_ge(u16::MAX as i32, u16::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u16::MIN as i32, u16::MIN));
        assert!(!IntCmp::cmp_gt(i32::MIN, u16::MIN));
        assert!(IntCmp::cmp_gt(i32::MAX, u16::MIN));
        assert!(IntCmp::cmp_gt(i32::MAX, u16::MAX));
        assert!(!IntCmp::cmp_gt(u16::MAX as i32, u16::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u16::MIN as i32, u16::MIN));
        assert!(IntCmp::cmp_le(i32::MIN, u16::MIN));
        assert!(!IntCmp::cmp_le(i32::MAX, u16::MIN));
        assert!(!IntCmp::cmp_le(i32::MAX, u16::MAX));
        assert!(IntCmp::cmp_le(u16::MAX as i32, u16::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u16::MIN as i32, u16::MIN));
        assert!(IntCmp::cmp_lt(i32::MIN, u16::MIN));
        assert!(!IntCmp::cmp_lt(i32::MAX, u16::MIN));
        assert!(!IntCmp::cmp_lt(i32::MAX, u16::MAX));
        assert!(!IntCmp::cmp_lt(u16::MAX as i32, u16::MAX));
    }
}

mod i32_u32_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u32::MIN as i32, u32::MIN));
        assert!(!IntCmp::cmp_eq(-1_i32, u32::MIN));
        assert!(!IntCmp::cmp_eq(i32::MIN, u32::MIN));
        assert!(!IntCmp::cmp_eq(i32::MAX, u32::MAX));
        assert!(IntCmp::cmp_eq(i32::MAX, i32::MAX as u32));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u32::MIN as i32, u32::MIN));
        assert!(IntCmp::cmp_ne(-1_i32, u32::MIN));
        assert!(IntCmp::cmp_ne(i32::MIN, u32::MIN));
        assert!(IntCmp::cmp_ne(i32::MAX, u32::MAX));
        assert!(!IntCmp::cmp_ne(i32::MAX, i32::MAX as u32));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u32::MIN as i32, u32::MIN));
        assert!(!IntCmp::cmp_ge(i32::MIN, u32::MIN));
        assert!(IntCmp::cmp_ge(i32::MAX, u32::MIN));
        assert!(!IntCmp::cmp_ge(i32::MAX, u32::MAX));
        assert!(IntCmp::cmp_ge(i32::MAX, i32::MAX as u32));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u32::MIN as i32, u32::MIN));
        assert!(!IntCmp::cmp_gt(i32::MIN, u32::MIN));
        assert!(IntCmp::cmp_gt(i32::MAX, u32::MIN));
        assert!(!IntCmp::cmp_gt(i32::MAX, u32::MAX));
        assert!(!IntCmp::cmp_gt(i32::MAX, i32::MAX as u32));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u32::MIN as i32, u32::MIN));
        assert!(IntCmp::cmp_le(i32::MIN, u32::MIN));
        assert!(!IntCmp::cmp_le(i32::MAX, u32::MIN));
        assert!(IntCmp::cmp_le(i32::MAX, u32::MAX));
        assert!(IntCmp::cmp_le(i32::MAX, i32::MAX as u32));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u32::MIN as i32, u32::MIN));
        assert!(IntCmp::cmp_lt(i32::MIN, u32::MIN));
        assert!(!IntCmp::cmp_lt(i32::MAX, u32::MIN));
        assert!(IntCmp::cmp_lt(i32::MAX, u32::MAX));
        assert!(!IntCmp::cmp_lt(i32::MAX, i32::MAX as u32));
    }
}

mod i32_u64_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u64::MIN as i32, u64::MIN));
        assert!(!IntCmp::cmp_eq(-1_i32, 1_u64));
        assert!(!IntCmp::cmp_eq(i32::MIN, u64::MIN));
        assert!(IntCmp::cmp_eq(i32::MAX, i32::MAX as u64));
        assert!(!IntCmp::cmp_eq(i32::MAX, u64::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u64::MIN as i32, u64::MIN));
        assert!(IntCmp::cmp_ne(-1_i32, 1_u64));
        assert!(IntCmp::cmp_ne(i32::MIN, u64::MIN));
        assert!(!IntCmp::cmp_ne(i32::MAX, i32::MAX as u64));
        assert!(IntCmp::cmp_ne(i32::MAX, u64::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u64::MIN as i32, u64::MIN));
        assert!(IntCmp::cmp_ge(i16::MAX, u64::MIN));
        assert!(!IntCmp::cmp_ge(i32::MIN, u64::MIN));
        assert!(IntCmp::cmp_ge(i32::MAX, i32::MAX as u64));
        assert!(!IntCmp::cmp_ge(i32::MAX, u64::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u64::MIN as i32, u64::MIN));
        assert!(IntCmp::cmp_gt(i16::MAX, u64::MIN));
        assert!(!IntCmp::cmp_gt(i32::MIN, u64::MIN));
        assert!(!IntCmp::cmp_gt(i32::MAX, i32::MAX as u64));
        assert!(!IntCmp::cmp_gt(i32::MAX, u64::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u64::MIN as i32, u64::MIN));
        assert!(!IntCmp::cmp_le(i16::MAX, u64::MIN));
        assert!(IntCmp::cmp_le(i32::MIN, u64::MIN));
        assert!(IntCmp::cmp_le(i32::MAX, i32::MAX as u64));
        assert!(IntCmp::cmp_le(i32::MAX, u64::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u64::MIN as i32, u64::MIN));
        assert!(!IntCmp::cmp_lt(i16::MAX, u64::MIN));
        assert!(IntCmp::cmp_lt(i32::MIN, u64::MIN));
        assert!(!IntCmp::cmp_lt(i32::MAX, i32::MAX as u64));
        assert!(IntCmp::cmp_lt(i32::MAX, u64::MAX));
    }
}

mod i32_u128_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(u128::MIN as i32, u128::MIN));
        assert!(!IntCmp::cmp_eq(-1_i32, 1_u128));
        assert!(!IntCmp::cmp_eq(i32::MIN, u128::MIN));
        assert!(IntCmp::cmp_eq(i32::MAX, i32::MAX as u128));
        assert!(!IntCmp::cmp_eq(i32::MAX, u128::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(u128::MIN as i32, u128::MIN));
        assert!(IntCmp::cmp_ne(-1_i32, 1_u128));
        assert!(IntCmp::cmp_ne(i32::MIN, u128::MIN));
        assert!(!IntCmp::cmp_ne(i32::MAX, i32::MAX as u128));
        assert!(IntCmp::cmp_ne(i32::MAX, u128::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(u128::MIN as i32, u128::MIN));
        assert!(IntCmp::cmp_ge(i16::MAX, u128::MIN));
        assert!(!IntCmp::cmp_ge(i32::MIN, u128::MIN));
        assert!(IntCmp::cmp_ge(i32::MAX, i32::MAX as u128));
        assert!(!IntCmp::cmp_ge(i32::MAX, u128::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(u128::MIN as i32, u128::MIN));
        assert!(IntCmp::cmp_gt(i16::MAX, u128::MIN));
        assert!(!IntCmp::cmp_gt(i32::MIN, u128::MIN));
        assert!(!IntCmp::cmp_gt(i32::MAX, i32::MAX as u128));
        assert!(!IntCmp::cmp_gt(i32::MAX, u128::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(u128::MIN as i32, u128::MIN));
        assert!(!IntCmp::cmp_le(i16::MAX, u128::MIN));
        assert!(IntCmp::cmp_le(i32::MIN, u128::MIN));
        assert!(IntCmp::cmp_le(i32::MAX, i32::MAX as u128));
        assert!(IntCmp::cmp_le(i32::MAX, u128::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(u128::MIN as i32, u128::MIN));
        assert!(!IntCmp::cmp_lt(i16::MAX, u128::MIN));
        assert!(IntCmp::cmp_lt(i32::MIN, u128::MIN));
        assert!(!IntCmp::cmp_lt(i32::MAX, i32::MAX as u128));
        assert!(IntCmp::cmp_lt(i32::MAX, u128::MAX));
    }
}

#[cfg(target_pointer_width = "32")]
mod i32_usize_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(usize::MIN as i32, usize::MIN));
        assert!(!IntCmp::cmp_eq(-1_i32, usize::MIN));
        assert!(!IntCmp::cmp_eq(i32::MIN, usize::MIN));
        assert!(!IntCmp::cmp_eq(i32::MAX, usize::MAX));
        assert!(IntCmp::cmp_eq(i32::MAX, i32::MAX as usize));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(usize::MIN as i32, usize::MIN));
        assert!(IntCmp::cmp_ne(-1_i32, usize::MIN));
        assert!(IntCmp::cmp_ne(i32::MIN, usize::MIN));
        assert!(IntCmp::cmp_ne(i32::MAX, usize::MAX));
        assert!(!IntCmp::cmp_ne(i32::MAX, i32::MAX as usize));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(usize::MIN as i32, usize::MIN));
        assert!(!IntCmp::cmp_ge(i32::MIN, usize::MIN));
        assert!(IntCmp::cmp_ge(i32::MAX, usize::MIN));
        assert!(!IntCmp::cmp_ge(i32::MAX, usize::MAX));
        assert!(IntCmp::cmp_ge(i32::MAX, i32::MAX as usize));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(usize::MIN as i32, usize::MIN));
        assert!(!IntCmp::cmp_gt(i32::MIN, usize::MIN));
        assert!(IntCmp::cmp_gt(i32::MAX, usize::MIN));
        assert!(!IntCmp::cmp_gt(i32::MAX, usize::MAX));
        assert!(!IntCmp::cmp_gt(i32::MAX, i32::MAX as usize));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(usize::MIN as i32, usize::MIN));
        assert!(IntCmp::cmp_le(i32::MIN, usize::MIN));
        assert!(!IntCmp::cmp_le(i32::MAX, usize::MIN));
        assert!(IntCmp::cmp_le(i32::MAX, usize::MAX));
        assert!(IntCmp::cmp_le(i32::MAX, i32::MAX as usize));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(usize::MIN as i32, usize::MIN));
        assert!(IntCmp::cmp_lt(i32::MIN, usize::MIN));
        assert!(!IntCmp::cmp_lt(i32::MAX, usize::MIN));
        assert!(IntCmp::cmp_lt(i32::MAX, usize::MAX));
        assert!(!IntCmp::cmp_lt(i32::MAX, i32::MAX as usize));
    }
}

#[cfg(target_pointer_width = "64")]
mod i32_usize_cmp {
    use super::*;
    #[test]
    fn eq() {
        assert!(IntCmp::cmp_eq(usize::MIN as i32, usize::MIN));
        assert!(!IntCmp::cmp_eq(-1_i32, 1_usize));
        assert!(!IntCmp::cmp_eq(i32::MIN, usize::MIN));
        assert!(IntCmp::cmp_eq(i32::MAX, i32::MAX as usize));
        assert!(!IntCmp::cmp_eq(i32::MAX, usize::MAX));
    }

    #[test]
    fn ne() {
        assert!(!IntCmp::cmp_ne(usize::MIN as i32, usize::MIN));
        assert!(IntCmp::cmp_ne(-1_i32, 1_usize));
        assert!(IntCmp::cmp_ne(i32::MIN, usize::MIN));
        assert!(!IntCmp::cmp_ne(i32::MAX, i32::MAX as usize));
        assert!(IntCmp::cmp_ne(i32::MAX, usize::MAX));
    }

    #[test]
    fn ge() {
        assert!(IntCmp::cmp_ge(usize::MIN as i32, usize::MIN));
        assert!(IntCmp::cmp_ge(i16::MAX, usize::MIN));
        assert!(!IntCmp::cmp_ge(i32::MIN, usize::MIN));
        assert!(IntCmp::cmp_ge(i32::MAX, i32::MAX as usize));
        assert!(!IntCmp::cmp_ge(i32::MAX, usize::MAX));
    }

    #[test]
    fn gt() {
        assert!(!IntCmp::cmp_gt(usize::MIN as i32, usize::MIN));
        assert!(IntCmp::cmp_gt(i16::MAX, usize::MIN));
        assert!(!IntCmp::cmp_gt(i32::MIN, usize::MIN));
        assert!(!IntCmp::cmp_gt(i32::MAX, i32::MAX as usize));
        assert!(!IntCmp::cmp_gt(i32::MAX, usize::MAX));
    }

    #[test]
    fn le() {
        assert!(IntCmp::cmp_le(usize::MIN as i32, usize::MIN));
        assert!(!IntCmp::cmp_le(i16::MAX, usize::MIN));
        assert!(IntCmp::cmp_le(i32::MIN, usize::MIN));
        assert!(IntCmp::cmp_le(i32::MAX, i32::MAX as usize));
        assert!(IntCmp::cmp_le(i32::MAX, usize::MAX));
    }

    #[test]
    fn lt() {
        assert!(!IntCmp::cmp_lt(usize::MIN as i32, usize::MIN));
        assert!(!IntCmp::cmp_lt(i16::MAX, usize::MIN));
        assert!(IntCmp::cmp_lt(i32::MIN, usize::MIN));
        assert!(!IntCmp::cmp_lt(i32::MAX, i32::MAX as usize));
        assert!(IntCmp::cmp_lt(i32::MAX, usize::MAX));
    }
}