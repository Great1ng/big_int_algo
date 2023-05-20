use super::digit::Digit;
use super::uint::Uint;
use std::ops::{Add, AddAssign};

#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn adc(carry: bool, a: Digit, b: Digit, out: &mut Digit) -> bool {
    core::arch::x86_64::_addcarry_u64(carry as u8, a, b, out) != 0
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
unsafe fn add2_unchecked(a: &mut Uint, b: &Uint) {
    let mut carry = false;
    let a_len = a.digits.len();
    let b_len = b.digits.len();
    let mut a_ptr = a.digits.as_mut_ptr();
    let mut b_ptr = b.digits.as_ptr();
    let a_end = a_ptr.add(a_len);
    let b_end = b_ptr.add(b_len);

    while b_ptr != b_end {
        carry = adc(carry, *a_ptr, *b_ptr, a_ptr.as_mut().unwrap_unchecked());
        a_ptr = a_ptr.add(1);
        b_ptr = b_ptr.add(1);
    }

    while a_ptr != a_end {
        if !carry { break; }
        (*a_ptr, carry) = (*a_ptr).overflowing_add(carry as Digit);
        a_ptr = a_ptr.add(1);
    }

    if carry {
        a.digits.push(carry as Digit);
    }
}

/// safe version of add2_unchecked
#[inline(always)]
fn add2(a: &mut Uint, b: &Uint) {
    if a.digits.len() < b.digits.len() {
        a.digits.resize(b.digits.len(), 0);
    }
    unsafe { add2_unchecked(a, b) };
}

/// Calculates sum of (a) and (b) then store the result in (c)
fn add3(c: &mut Uint, a: &Uint, b: &Uint) {
    let a_len = a.digits.len();
    let b_len = b.digits.len();
    
    let a_ptr = a.digits.as_ptr();
    let b_ptr = b.digits.as_ptr();
    let c_ptr = c.digits.as_mut_ptr();

    c.digits.reserve(a_len.max(b_len) + 1);

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
    
    add2(c, b);
}

impl AddAssign<&Uint> for Uint {
    #[inline]
    fn add_assign(&mut self, other: &Uint) {
        add2(self, other);        
    }
}

impl AddAssign<Uint> for Uint {
    #[inline]
    fn add_assign(&mut self, other: Uint) {
        add2(self, &other);
    }
}

impl Add<&Uint> for &Uint {
    type Output = Uint;

    #[inline]
    fn add(self, other: &Uint) -> Self::Output {
        let mut result = Uint::zero();
        add3(&mut result, self, other);
        result
    }
}

impl Add<Uint> for Uint {
    type Output = Uint;

    #[inline]
    fn add(self, other: Uint) -> Self::Output {
        &self + &other
    }
}

impl Add<&Uint> for Uint {
    type Output = Uint;

    #[inline]
    fn add(self, other: &Uint) -> Self::Output {
        &self + other
    }
}

impl Add<Uint> for &Uint {
    type Output = Uint;

    #[inline]
    fn add(self, other: Uint) -> Self::Output {
        self + &other  
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Uint::from([1 << 63]);
        let b = Uint::from([1 << 63]);
        let mut c = Uint::from([0, 1]);
        assert_eq!(&a + &b, c);
        c += Uint::from([1]);
        assert_eq!(c, Uint::from([1, 1]));
    }
}