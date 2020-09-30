pub mod u8_i8;

/// Comparison trait for integer types
pub trait IntCmp<Other: Copy>: Copy {
    fn cmp_eq(self, other: Other) -> bool;

    fn cmp_ne(self, other: Other) -> bool;

    fn cmp_ge(self, other: Other) -> bool;

    fn cmp_gt(self, other: Other) -> bool;

    fn cmp_le(self, other: Other) -> bool;

    fn cmp_lt(self, other: Other) -> bool;
}


