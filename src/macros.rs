// First type has to be unsigned and second signed
#[macro_export]
macro_rules! signed_cmp {
    // Use this when MAX value of first type is smaller than max value of second type
    ($small:ty => $big:ty) => {
        impl IntCmp<$big> for $small {
            fn cmp_eq(self, other: $big) -> bool {
                if other < 0 || other > <$small>::MAX as $big {
                    return false;
                }

                self == other as $small
            }

            fn cmp_ne(self, other: $big) -> bool {
                if other < 0 || other > <$small>::MAX as $big {
                    return true;
                }

                self != other as $small
            }

            fn cmp_ge(self, other: $big) -> bool {
                if other < 0 {
                    return true;
                } else if other > <$small>::MAX as $big {
                    return false;
                }

                self >= other as $small
            }

            fn cmp_gt(self, other: $big) -> bool {
                if other < 0 {
                    return true;
                } else if other > <$small>::MAX as $big {
                    return false;
                }

                self > other as $small
            }

            fn cmp_le(self, other: $big) -> bool {
                if other < 0 {
                    return false;
                } else if other > <$small>::MAX as $big {
                    return true;
                }

                self <= other as $small
            }

            fn cmp_lt(self, other: $big) -> bool {
                if other < 0 {
                    return false;
                } else if other > <$small>::MAX as $big {
                    return true;
                }

                self < other as $small
            }
        }
    };

    // Use this when MAX value of first type is bigger than max value of second type
    ($big:ty > $small:ty) => {
        impl IntCmp<$small> for $big {
            fn cmp_eq(self, other: $small) -> bool {
                if other < 0 {
                    return false;
                }

                self == other as $big
            }

            fn cmp_ne(self, other: $small) -> bool {
                if other < 0 {
                    return true;
                }

                self != other as $big
            }

            fn cmp_ge(self, other: $small) -> bool {
                if other < 0 {
                    return true;
                }

                self >= other as $big
            }

            fn cmp_gt(self, other: $small) -> bool {
                if other < 0 {
                    return true;
                }

                self > other as $big
            }

            fn cmp_le(self, other: $small) -> bool {
                if other < 0 {
                    return false;
                }

                self <= other as $big
            }

            fn cmp_lt(self, other: $small) -> bool {
                if other < 0 as $small {
                    return false;
                }

                self < other as $big
            }
        }
    };
}
