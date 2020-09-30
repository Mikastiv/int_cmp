use super::IntCmp;

use std::u8::MIN as MIN;
use std::u8::MAX as MAX;

impl IntCmp<i16> for u8 {
    fn cmp_eq(self, other: i16) -> bool {
        if other < MIN as i16 || other > MAX as i16 {
            return false;
        }

        self == other as u8
    }

    fn cmp_ne(self, other: i16) -> bool {
        if other < MIN as i16 || other > MAX as i16 {
            return true;
        }

        self != other as u8
    }

    fn cmp_ge(self, other: i16) -> bool {
        if other < MIN as i16 {
            return true;
        } else if other > MAX as i16 {
            return false;
        }

        self >= other as u8
    }

    fn cmp_gt(self, other: i16) -> bool {
        if other < MIN as i16 {
            return true;
        } else if other > MAX as i16 {
            return false;
        }

        self > other as u8
    }

    fn cmp_le(self, other: i16) -> bool {
        if other < MIN as i16 {
            return false;
        } else if other > MAX as i16 {
            return true;
        }

        self <= other as u8
    }

    fn cmp_lt(self, other: i16) -> bool {
        if other < MIN as i16 {
            return false;
        } else if other > MAX as i16 {
            return true;
        }

        self < other as u8
    }
}
