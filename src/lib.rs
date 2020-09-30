pub mod u8_i16;
pub mod u8_i8;

/// Comparison trait for integer types
pub trait IntCmp<Other: Integer> {
    fn cmp_eq(self, other: Other) -> bool;

    fn cmp_ne(self, other: Other) -> bool;

    fn cmp_ge(self, other: Other) -> bool;

    fn cmp_gt(self, other: Other) -> bool;

    fn cmp_le(self, other: Other) -> bool;

    fn cmp_lt(self, other: Other) -> bool;
}

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
