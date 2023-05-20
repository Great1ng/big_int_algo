use super::uint::Uint;

fn reverse(s: String) -> String {
    s.chars().rev().collect()
}

impl std::fmt::Display for Uint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.digits.len() == 0 {
            return f.write_str("0");
        }
        
        let mut result = String::new();

        let factor = 1e19 as u64;
        let (mut next, mut rem) = self.div_rem(factor);
        result.push_str(&reverse(format!("{:019}", rem)));

        while next.digits.len() > 1 || next.digits.last() >= Some(&factor) {
            (next, rem) = next.div_rem(factor);
            result.push_str(&reverse(format!("{:019}", rem)));
        }

        if let Some(last) = next.digits.last() {
            result.push_str(&reverse(format!("{:019}", last)));
        }

        let s = reverse(result.trim_end_matches(|c: char| c == '0').to_owned());
        f.write_str(&s)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let a = Uint::from([u64::MAX, u64::MAX]);
        assert_eq!(a.to_string(), u128::MAX.to_string());
    }
}