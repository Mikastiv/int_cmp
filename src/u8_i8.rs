use super::IntCmp;

use std::u8::MIN as MIN;

impl IntCmp<i8> for u8 {
    fn cmp_eq(self, other: i8) -> bool {
        if other < MIN as i8 {
            return false;
        }

        self == other as u8
    }

    fn cmp_ne(self, other: i8) -> bool {
        if other < MIN as i8 {
            return true;
        }

        self != other as u8
    }

    fn cmp_ge(self, other: i8) -> bool {
        if other < MIN as i8 {
            return true;
        }

        self >= other as u8
    }

    fn cmp_gt(self, other: i8) -> bool {
        if other < MIN as i8 {
            return true;
        }

        self > other as u8
    }

    fn cmp_le(self, other: i8) -> bool {
        if other < MIN as i8 {
            return false;
        }

        self <= other as u8
    }

    fn cmp_lt(self, other: i8) -> bool {
        if other < MIN as i8 {
            return false;
        }

        self < other as u8
    }
}