mod digits;
use digits::get_digits;

pub struct DigitPowers {
    exponent: u32,
    digit_power_numbers: Option<Vec<u128>>,
}

impl DigitPowers {
    pub fn new(exponent: u32) -> DigitPowers {
        DigitPowers {
            exponent,
            digit_power_numbers: None,
        }
    }

    pub fn get_numbers(&mut self) -> Vec<u128> {
        if let None = self.digit_power_numbers {
            self.calc_numbers();
        }
        return self.digit_power_numbers.clone().unwrap();
    }  
}

impl DigitPowers {
    fn calc_boundaries(&self) -> (u128, u128) {
        let mut i = 1;
        let mut upper: u128 = 9_u128.pow(self.exponent);
        let mut left_num: u128 = 9;

        while left_num < upper {
            i += 1;
            upper = i * 9_u128.pow(self.exponent);
            left_num = left_num * 10 + 9;
        }

        (2_u128.pow(self.exponent), upper)
    }

    fn calc_digit_power(&self, digit: u128) -> u128 {
        let mut  dp = 0;
        for digit in get_digits(digit) {
            dp += (digit as u128).pow(self.exponent);
        }
        dp
    }

    fn calc_numbers(&mut self) {
        let mut nums: Vec<u128> = Vec::new();
        let (lower, upper) = self.calc_boundaries();

        for num in lower..=upper {
            if num == self.calc_digit_power(num) {
                nums.push(num);
            }
        }
        self.digit_power_numbers = Some(nums);
    }
}

#[cfg(test)]
mod tests {
    use super::DigitPowers;
    #[test]
    fn get_digit_powers_of_4() {
        let mut digit_powers = DigitPowers::new(4);
        assert_eq!(digit_powers.get_numbers().iter().sum::<u128>(), 19316);
    }
}
