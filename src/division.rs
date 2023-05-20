use super::uint::Uint;
use super::digit::{Digit, DoubleDigit, Digits};
use std::ops::Div;

impl Div<Digit> for &Uint {
    type Output = Uint;

    fn div(self, other: Digit) -> Self::Output {
        let mut result_digits = Digits::new();
        let mut rem: DoubleDigit = 0;
        for i in (0..self.digits.len()).rev() {
            if rem == 0 && self.digits[i] < other {
                rem = self.digits[i] as DoubleDigit;
                result_digits.push(0);
            } else {
                let result = self.digits[i] as DoubleDigit + (rem << Digit::BITS);
                rem = result % other as DoubleDigit;
                result_digits.push((result / other as DoubleDigit) as Digit);
            }
        }

        Uint::from(result_digits)
    }   
}