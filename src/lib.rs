pub mod macros;

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

signed_cmp! {u8 > i8}
signed_cmp! {u8 => i16}
signed_cmp! {u8 => i32}
signed_cmp! {u8 => i64}
signed_cmp! {u8 => i128}

signed_cmp! {u16 > i8}
signed_cmp! {u16 > i16}
signed_cmp! {u16 => i32}
signed_cmp! {u16 => i64}
signed_cmp! {u16 => i128}

signed_cmp! {u32 > i8}
signed_cmp! {u32 > i16}
signed_cmp! {u32 > i32}
signed_cmp! {u32 => i64}
signed_cmp! {u32 => i128}

signed_cmp! {u64 > i8}
signed_cmp! {u64 > i16}
signed_cmp! {u64 > i32}
signed_cmp! {u64 > i64}
signed_cmp! {u64 => i128}

signed_cmp! {u128 > i8}
signed_cmp! {u128 > i16}
signed_cmp! {u128 > i32}
signed_cmp! {u128 > i64}
signed_cmp! {u128 > i128}
