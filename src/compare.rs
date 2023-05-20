use super::digit::Digit;
use super::uint::Uint;

impl PartialOrd for Uint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.digits.iter().rev().partial_cmp(other.digits.iter().rev())
    }
}

impl Ord for Uint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.digits.iter().rev().cmp(other.digits.iter().rev())
    }
}

impl PartialEq for Uint {
    fn eq(&self, other: &Self) -> bool {
        self.digits.iter().rev().eq(other.digits.iter().rev())
    }
}

impl Eq for Uint {}

impl PartialEq<u8> for Uint {
    fn eq(&self, other: &u8) -> bool {
        self.digits.last() == Some(&(*other as Digit))
    }
}

impl PartialEq<u16> for Uint {
    fn eq(&self, other: &u16) -> bool {
        self.digits.last() == Some(&(*other as Digit))
    }
}

impl PartialEq<u32> for Uint {
    fn eq(&self, other: &u32) -> bool {
        self.digits.last() == Some(&(*other as Digit))
    }
}

impl PartialEq<Digit> for Uint {
    fn eq(&self, other: &Digit) -> bool {
        self.digits.last() == Some(other)
    }
}