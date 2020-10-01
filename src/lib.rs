#[macro_export]
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
                if other < 0 as $i_small {
                    return false;
                }

                self < other as $u_big
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
