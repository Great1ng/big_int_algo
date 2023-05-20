use super::uint::Uint;
use super::digit::{Digit, DoubleDigit};
use std::ops::MulAssign;

impl MulAssign<Digit> for Uint {
    fn mul_assign(&mut self, other: Digit) {
        let mut carry = 0;
        for digit in self.digits.iter_mut() {
            let result = *digit as DoubleDigit * other as DoubleDigit + carry;
            (carry, *digit) = (result >> Digit::BITS, result as Digit); 
        }

        if carry > 0 {
            self.digits.push(carry as Digit);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial() {
        let mut factorial = Uint::from([1]);
        let mut result = Vec::with_capacity(10);
        for i in 1..10 {
            factorial *= i;
            result.push(factorial.clone());
        }

        assert_eq!(&result, &[1_u64, 2, 6, 24, 120, 720, 5040, 40320, 362880])
    }
}