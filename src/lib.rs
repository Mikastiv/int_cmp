macro_rules! impl_signed_cmp {
    // First type has to be unsigned and second signed
    // Use this when MAX value of first type is smaller than max value of second type
    ($u_small:ty => $i_big:ty) => {
        impl IntCmp<$i_big> for $u_small {
            #[inline(always)]
            fn cmp_eq(self, other: $i_big) -> bool {
                if other < 0 || other > <$u_small>::MAX as $i_big {
                    return false;
                }

                self == other as $u_small
            }

            #[inline(always)]
            fn cmp_ne(self, other: $i_big) -> bool {
                if other < 0 || other > <$u_small>::MAX as $i_big {
                    return true;
                }

                self != other as $u_small
            }

            #[inline(always)]
            fn cmp_ge(self, other: $i_big) -> bool {
                if other < 0 {
                    return true;
                } else if other > <$u_small>::MAX as $i_big {
                    return false;
                }

                self >= other as $u_small
            }

            #[inline(always)]
            fn cmp_gt(self, other: $i_big) -> bool {
                if other < 0 {
                    return true;
                } else if other > <$u_small>::MAX as $i_big {
                    return false;
                }

                self > other as $u_small
            }

            #[inline(always)]
            fn cmp_le(self, other: $i_big) -> bool {
                if other < 0 {
                    return false;
                } else if other > <$u_small>::MAX as $i_big {
                    return true;
                }

                self <= other as $u_small
            }

            #[inline(always)]
            fn cmp_lt(self, other: $i_big) -> bool {
                if other < 0 {
                    return false;
                } else if other > <$u_small>::MAX as $i_big {
                    return true;
                }

                self < other as $u_small
            }
        }
    };

    // First type has to be unsigned and second signed
    // Use this when MAX value of first type is bigger than max value of second type
    ($u_big:ty > $i_small:ty) => {
        impl IntCmp<$i_small> for $u_big {
            #[inline(always)]
            fn cmp_eq(self, other: $i_small) -> bool {
                if other < 0 {
                    return false;
                }

                self == other as $u_big
            }

            #[inline(always)]
            fn cmp_ne(self, other: $i_small) -> bool {
                if other < 0 {
                    return true;
                }

                self != other as $u_big
            }

            #[inline(always)]
            fn cmp_ge(self, other: $i_small) -> bool {
                if other < 0 {
                    return true;
                }

                self >= other as $u_big
            }

            #[inline(always)]
            fn cmp_gt(self, other: $i_small) -> bool {
                if other < 0 {
                    return true;
                }

                self > other as $u_big
            }

            #[inline(always)]
            fn cmp_le(self, other: $i_small) -> bool {
                if other < 0 {
                    return false;
                }

                self <= other as $u_big
            }

            #[inline(always)]
            fn cmp_lt(self, other: $i_small) -> bool {
                if other < 0 {
                    return false;
                }

                self < other as $u_big
            }
        }
    };
    
    // First type has to be signed and second unsigned
    // Use this when MAX value of first type is smaller than max value of second type
    ($i_small:ty ; $u_big:ty) => {
        impl IntCmp<$u_big> for $i_small {
            #[inline(always)]
            fn cmp_eq(self, other: $u_big) -> bool {
                if self < 0 {
                    return false;
                }

                self as $u_big == other
            }

            #[inline(always)]
            fn cmp_ne(self, other: $u_big) -> bool {
                if self < 0 {
                    return true;
                }

                self as $u_big != other
            }

            #[inline(always)]
            fn cmp_ge(self, other: $u_big) -> bool {
                if self < 0 {
                    return false;
                }

                self as $u_big >= other
            }

            #[inline(always)]
            fn cmp_gt(self, other: $u_big) -> bool {
                if self < 0 {
                    return false;
                }

                self as $u_big > other
            }

            #[inline(always)]
            fn cmp_le(self, other: $u_big) -> bool {
                if self < 0 {
                    return true;
                }

                self as $u_big <= other
            }

            #[inline(always)]
            fn cmp_lt(self, other: $u_big) -> bool {
                if self < 0 {
                    return true;
                }

                self as $u_big < other
            }
        }
    };

    // First type has to be signed and second unsigned
    // Use this when MAX value of first type is bigger than max value of second type
    ($i_big:ty | $u_small:ty) => {
        impl IntCmp<$u_small> for $i_big {
            #[inline(always)]
            fn cmp_eq(self, other: $u_small) -> bool {
                if self < 0 || self > <$u_small>::MAX as $i_big {
                    return false;
                }

                self as $u_small == other
            }

            #[inline(always)]
            fn cmp_ne(self, other: $u_small) -> bool {
                if self < 0 || self > <$u_small>::MAX as $i_big {
                    return true;
                }

                self as $u_small != other
            }

            #[inline(always)]
            fn cmp_ge(self, other: $u_small) -> bool {
                if self < 0 {
                    return false;
                } else if self > <$u_small>::MAX as $i_big {
                    return true;
                }

                self as $u_small >= other
            }

            #[inline(always)]
            fn cmp_gt(self, other: $u_small) -> bool {
                if self < 0 {
                    return false;
                } else if self > <$u_small>::MAX as $i_big {
                    return true;
                }

                self as $u_small > other
            }

            #[inline(always)]
            fn cmp_le(self, other: $u_small) -> bool {
                if self < 0 {
                    return true;
                } else if self > <$u_small>::MAX as $i_big {
                    return false;
                }

                self as $u_small <= other
            }

            #[inline(always)]
            fn cmp_lt(self, other: $u_small) -> bool {
                if self < 0 {
                    return true;
                } else if self > <$u_small>::MAX as $i_big {
                    return false;
                }

                self as $u_small < other
            }
        }
    };
}

/// Comparison trait for integer types
pub trait IntCmp<Other: Integer> {
    fn cmp_eq(self, other: Other) -> bool;

    fn cmp_ne(self, other: Other) -> bool;

    fn cmp_ge(self, other: Other) -> bool;

    fn cmp_gt(self, other: Other) -> bool;

    fn cmp_le(self, other: Other) -> bool;

    fn cmp_lt(self, other: Other) -> bool;
}

/// Standard integer type trait
pub trait Integer {}

impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for u128 {}
impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for i128 {}

impl_signed_cmp! {u8 > i8}
impl_signed_cmp! {u8 => i16}
impl_signed_cmp! {u8 => i32}
impl_signed_cmp! {u8 => i64}
impl_signed_cmp! {u8 => i128}

impl_signed_cmp! {u16 > i8}
impl_signed_cmp! {u16 > i16}
impl_signed_cmp! {u16 => i32}
impl_signed_cmp! {u16 => i64}
impl_signed_cmp! {u16 => i128}

impl_signed_cmp! {u32 > i8}
impl_signed_cmp! {u32 > i16}
impl_signed_cmp! {u32 > i32}
impl_signed_cmp! {u32 => i64}
impl_signed_cmp! {u32 => i128}

impl_signed_cmp! {u64 > i8}
impl_signed_cmp! {u64 > i16}
impl_signed_cmp! {u64 > i32}
impl_signed_cmp! {u64 > i64}
impl_signed_cmp! {u64 => i128}

impl_signed_cmp! {u128 > i8}
impl_signed_cmp! {u128 > i16}
impl_signed_cmp! {u128 > i32}
impl_signed_cmp! {u128 > i64}
impl_signed_cmp! {u128 > i128}

impl_signed_cmp! {i8 ; u8}
impl_signed_cmp! {i8 ; u16}
impl_signed_cmp! {i8 ; u32}
impl_signed_cmp! {i8 ; u64}
impl_signed_cmp! {i8 ; u128}

impl_signed_cmp! {i16 | u8}
impl_signed_cmp! {i16 ; u16}
impl_signed_cmp! {i16 ; u32}
impl_signed_cmp! {i16 ; u64}
impl_signed_cmp! {i16 ; u128}

impl_signed_cmp! {i32 | u8}
impl_signed_cmp! {i32 | u16}
impl_signed_cmp! {i32 ; u32}
impl_signed_cmp! {i32 ; u64}
impl_signed_cmp! {i32 ; u128}

impl_signed_cmp! {i64 | u8}
impl_signed_cmp! {i64 | u16}
impl_signed_cmp! {i64 | u32}
impl_signed_cmp! {i64 ; u64}
impl_signed_cmp! {i64 ; u128}

impl_signed_cmp! {i128 | u8}
impl_signed_cmp! {i128 | u16}
impl_signed_cmp! {i128 | u32}
impl_signed_cmp! {i128 | u64}
impl_signed_cmp! {i128 ; u128}