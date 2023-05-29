use crate::digit::Digits;

use super::uint::Uint;
use super::digit::{Digit, DoubleDigit};
use std::ops::{Mul, MulAssign};

#[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
#[inline(always)]
unsafe fn mul(a: Digit, b: Digit, hi: &mut Digit) -> Digit {
    core::arch::x86_64::_mulx_u64(a, b, hi)
}

#[cfg(not(all(target_arch = "x86_64", target_feature = "bmi2")))]
#[inline(always)]
unsafe fn mul(a: Digit, b: Digit, hi: &mut Digit) -> Digit {
    let res = a as DoubleDigit * b as DoubleDigit;
    *hi = (res >> u64::BITS) as Digit;
    res as Digit
}

#[inline(always)]
fn mul_add(a: Digit, b: Digit, c: Digit) -> (Digit, Digit) {
    let mut hi = 0;
    let (lo, carry) = unsafe { mul(a, b, &mut hi) }.overflowing_add(c);
    (lo, hi + carry as Digit) 
}

#[inline(always)]
fn mul2_digit(a: &mut Uint, b: Digit) {
    let mut hi = 0;
    let mut a_ptr = a.digits.as_mut_ptr();
    let a_end = unsafe { a_ptr.add(a.digits.len()) };
    while a_ptr != a_end {
        unsafe {
            (*a_ptr, hi) = mul_add(*a_ptr, b, hi);
            a_ptr = a_ptr.add(1);
        }
    }

    if hi > 0 {
        a.digits.push(hi);
    }
}

fn mul2(a: &mut Uint, b: &Uint) {
    let result_digits = Digits::with_capacity(a.digits.len() + b.digits.len());
    let mut result = Uint::from(result_digits);

    for (shift, &digit) in b.digits.iter().enumerate() {
        crate::addition::shift_add2(&mut result, &(&*a * digit), shift);
    }
}

fn mul3<'a>(c: &mut Uint, mut a: &'a Uint, mut b: &'a Uint) {
    c.digits.reserve(a.digits.len() + b.digits.len());

    if a.digits.len() < b.digits.len() {
        core::mem::swap(&mut a, &mut b);
    }

    for (shift, &digit) in b.digits.iter().enumerate() {
        crate::addition::shift_add2(c, &(a * digit), shift);
    }
}

impl MulAssign<&Uint> for Uint {
    #[inline]
    fn mul_assign(&mut self, other: &Uint) {
        mul2(self, other)
    }
}

impl MulAssign<Uint> for Uint {
    #[inline]
    fn mul_assign(&mut self, other: Uint) {
        mul2(self, &other)
    }
}

impl MulAssign<Digit> for Uint {
    #[inline]
    fn mul_assign(&mut self, other: Digit) {
        mul2_digit(self, other)
    }
}

impl Mul<Digit> for Uint {
    type Output = Uint;

    #[inline]
    fn mul(self, other: Digit) -> Self::Output {
        let mut result = self;
        mul2_digit(&mut result, other);
        result
    }
}

impl Mul<Digit> for &Uint {
    type Output = Uint;

    #[inline]
    fn mul(self, other: Digit) -> Self::Output {
        let mut result = self.clone();
        mul2_digit(&mut result, other);
        result
    }
}

impl Mul<&Uint> for &Uint {
    type Output = Uint;

    #[inline]
    fn mul(self, other: &Uint) -> Self::Output {
        let mut result = Uint::zero();
        mul3(&mut result, self, other);
        result
    }
}

impl Mul<Uint> for Uint {
    type Output = Uint;

    #[inline]
    fn mul(self, other: Uint) -> Self::Output {
        &self * &other
    }
}

impl Mul<&Uint> for Uint {
    type Output = Uint;

    #[inline]
    fn mul(self, other: &Uint) -> Self::Output {
        &self * other
    }
}

impl Mul<Uint> for &Uint {
    type Output = Uint;

    #[inline]
    fn mul(self, other: Uint) -> Self::Output {
        self * &other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn factorial(n: u64) -> Uint {
        let mut factorial = Uint::from([1]);
        for i in 2..=n {
            factorial *= i;
        }
        factorial
    }

    #[test]
    fn digit_mul() {
        assert_eq!(factorial(31).to_string(), "8222838654177922817725562880000000");
        assert_eq!(factorial(50).to_string(), "30414093201713378043612608166064768844377641568960512000000000000");
    }

    #[test]
    fn two_big_numbers() {
        let a = factorial(40);
        let b = factorial(42);
        let mut c = b.clone();

        for i in 2..=40 {
            c *= i;
        }   

        assert_eq!(a * b, c);
    }

    #[test]
    fn very_big() {
        let num = factorial(1000);
        println!("{}\n bits: {}\n digits: {}", num, num.bits(), num.bits() / 64);
    }
}