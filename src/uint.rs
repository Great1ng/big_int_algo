use super::digit::{Digit, DoubleDigit, Digits, normalize};

#[derive(Clone, Debug)]
pub struct Uint {
    pub(crate) digits: Digits,
}   

impl Uint {
    pub fn zero() -> Self {
        Self { digits: Digits::with_capacity(0) }
    }

    pub fn normalize(&mut self) {
        normalize(&mut self.digits);
    }

    pub fn bits(&self) -> u128 {
        self.digits.len() as u128 * Digit::BITS as u128
    }

    pub fn div_rem(&self, other: Digit) -> (Self, Digit) {
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

        result_digits.reverse();
        (Uint::from(result_digits), rem as Digit)
    }
}

impl From<Digits> for Uint {
    fn from(mut digits: Digits) -> Self {
        normalize(&mut digits);
        Self { digits }
    }
}

impl From<&[Digit]> for Uint {
    fn from(slice: &[Digit]) -> Self {
        let mut digits = Digits::from(slice);
        normalize(&mut digits);
        Self { digits }
    }
}

impl<const N: usize> From<[Digit; N]> for Uint {
    fn from(array: [Digit; N]) -> Self {
        let mut digits = array.into_iter().collect();
        normalize(&mut digits);
        Self { digits }
    }
}