use super::digit::Digit;
use super::uint::Uint;
use std::ops::{Sub, SubAssign};

#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn sbb(carry: bool, a: Digit, b: Digit, out: &mut Digit) -> bool {
    core::arch::x86_64::_subborrow_u64(carry as u8, a, b, out) != 0
}

#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
unsafe fn adc(carry: bool, a: Digit, b: Digit, out: &mut Digit) -> bool {
    use super::digit::DoubleDigit;
    let result = a as DoubleDigit + b as DoubleDigit + carry as DoubleDigit;
    *out = result as Digit;
    (result >> Digit::BITS) != 0
}

/// (a) must have more or equal amount of digits than (b)
#[inline(always)]
unsafe fn sub2_unchecked(a: &mut Uint, b: &Uint) {
    let mut carry = false;
    let a_len = a.digits.len();
    let b_len = b.digits.len();
    let mut a_ptr = a.digits.as_mut_ptr();
    let mut b_ptr = b.digits.as_ptr();
    let a_end = a_ptr.add(a_len);
    let b_end = b_ptr.add(b_len);

    while b_ptr != b_end {
        carry = sbb(carry, *a_ptr, *b_ptr, a_ptr.as_mut().unwrap_unchecked());
        a_ptr = a_ptr.add(1);
        b_ptr = b_ptr.add(1);
    }

    while a_ptr != a_end {
        if !carry { break; }
        (*a_ptr, carry) = (*a_ptr).overflowing_sub(carry as Digit);
        a_ptr = a_ptr.add(1);
    }
}

/// safe version of sub2_unchecked
#[inline(always)]
fn sub2(a: &mut Uint, b: &Uint) {
    if &*a < b {
        panic!("Subtraction is not allowed");
    }
    unsafe { sub2_unchecked(a, b) };
}

/// Calculates difference of (a) and (b) then store the result in (c)
fn sub3(c: &mut Uint, a: &Uint, b: &Uint) {
    let a_len = a.digits.len();
    let b_len = b.digits.len();
    
    let a_ptr = a.digits.as_ptr();
    let b_ptr = b.digits.as_ptr();
    let c_ptr = c.digits.as_mut_ptr();

    c.digits.reserve(a_len.max(b_len));

    if a_len == 0 {
        unsafe {
            core::ptr::copy_nonoverlapping(b_ptr, c_ptr, b_len);
            c.digits.set_len(b_len);
        }
        return;
    }

    unsafe {
        core::ptr::copy_nonoverlapping(a_ptr, c_ptr, a_len);
        c.digits.set_len(a_len);
    }
    
    if b_len == 0 {
        return;
    }
    
    sub2(c, b);
}

impl SubAssign<&Uint> for Uint {
    #[inline]
    fn sub_assign(&mut self, other: &Uint) {
        sub2(self, other); 
        self.normalize();       
    }
}

impl SubAssign<Uint> for Uint {
    #[inline]
    fn sub_assign(&mut self, other: Uint) {
        sub2(self, &other);
        self.normalize();
    }
}

impl Sub<&Uint> for &Uint {
    type Output = Uint;

    #[inline]
    fn sub(self, other: &Uint) -> Self::Output {
        let mut result = Uint::zero();
        sub3(&mut result, self, other);
        result.normalize();
        result
    }
}

impl Sub<Uint> for Uint {
    type Output = Uint;

    #[inline]
    fn sub(self, other: Uint) -> Self::Output {
        &self - &other
    }
}

impl Sub<&Uint> for Uint {
    type Output = Uint;

    #[inline]
    fn sub(self, other: &Uint) -> Self::Output {
        &self - other
    }
}

impl Sub<Uint> for &Uint {
    type Output = Uint;

    #[inline]
    fn sub(self, other: Uint) -> Self::Output {
        self - &other  
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub() {
        let a = Uint::from([1 << 63]);
        let b = Uint::from([1 << 63]);
        let mut c = Uint::zero();
        assert_eq!(&a - &b, c);
        c += Uint::from([0, 1]);
        assert_eq!(c - Uint::from([1]), Uint::from([u64::MAX]));
    }
}