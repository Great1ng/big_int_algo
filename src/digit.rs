pub(crate) type Digit = u64;
pub(crate) type DoubleDigit = u128;
pub(crate) type Digits = smallvec::SmallVec<[Digit; 8]>;

pub fn normalize(digits: &mut Digits) {
    while let Some(digit) = digits.pop() {
        if digit != 0 {
            digits.push(digit);
            break;
        }
    }
}