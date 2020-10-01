use super::IntCmp;

macro_rules! u8_signed_cmp {
    ($type:ty) => {
        impl IntCmp<$type> for u8 {
            fn cmp_eq(self, other: $type) -> bool {
                if other < u8::MIN as $type || other > u8::MAX as $type {
                    return false;
                }

                self == other as u8
            }

            fn cmp_ne(self, other: $type) -> bool {
                if other < u8::MIN as $type || other > u8::MAX as $type {
                    return true;
                }

                self != other as u8
            }

            fn cmp_ge(self, other: $type) -> bool {
                if other < u8::MIN as $type {
                    return true;
                } else if other > u8::MAX as $type {
                    return false;
                }

                self >= other as u8
            }

            fn cmp_gt(self, other: $type) -> bool {
                if other < u8::MIN as $type {
                    return true;
                } else if other > u8::MAX as $type {
                    return false;
                }

                self > other as u8
            }

            fn cmp_le(self, other: $type) -> bool {
                if other < u8::MIN as $type {
                    return false;
                } else if other > u8::MAX as $type {
                    return true;
                }

                self <= other as u8
            }

            fn cmp_lt(self, other: $type) -> bool {
                if other < u8::MIN as $type {
                    return false;
                } else if other > u8::MAX as $type {
                    return true;
                }

                self < other as u8
            }
        }
    };
}

macro_rules! u8_unsigned_cmp {
    ($type:ty) => {
        impl IntCmp<$type> for u8 {
            fn cmp_eq(self, other: $type) -> bool {
                self as $type == other
            }

            fn cmp_ne(self, other: $type) -> bool {
                self as $type != other
            }

            fn cmp_ge(self, other: $type) -> bool {
                self as $type >= other
            }

            fn cmp_gt(self, other: $type) -> bool {
                self as $type > other
            }

            fn cmp_le(self, other: $type) -> bool {
                self as $type <= other
            }

            fn cmp_lt(self, other: $type) -> bool {
                self as $type < other
            }
        }
    };
}

impl IntCmp<i8> for u8 {
    fn cmp_eq(self, other: i8) -> bool {
        if other < u8::MIN as i8 {
            return false;
        }

        self == other as u8
    }

    fn cmp_ne(self, other: i8) -> bool {
        if other < u8::MIN as i8 {
            return true;
        }

        self != other as u8
    }

    fn cmp_ge(self, other: i8) -> bool {
        if other < u8::MIN as i8 {
            return true;
        }

        self >= other as u8
    }

    fn cmp_gt(self, other: i8) -> bool {
        if other < u8::MIN as i8 {
            return true;
        }

        self > other as u8
    }

    fn cmp_le(self, other: i8) -> bool {
        if other < u8::MIN as i8 {
            return false;
        }

        self <= other as u8
    }

    fn cmp_lt(self, other: i8) -> bool {
        if other < u8::MIN as i8 {
            return false;
        }

        self < other as u8
    }
}

u8_signed_cmp! {i16}
u8_signed_cmp! {i32}
u8_signed_cmp! {i64}
u8_signed_cmp! {i128}
u8_unsigned_cmp! {u16}
u8_unsigned_cmp! {u32}
u8_unsigned_cmp! {u64}
u8_unsigned_cmp! {u128}
