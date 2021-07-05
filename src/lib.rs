use regex::Regex;

pub trait Card {
    fn _validate(number: &str) -> bool {
        let digits: Vec<_> = number.chars().into_iter().map(|x| x.to_digit(10)).collect();

        let mut even = 0;
        let mut odd = 0;

        for (i, x) in digits.iter().enumerate().rev() {
            if x.is_none() {
                return false;
            }

            if i % 2 != 0 {
                odd += x.unwrap();
            } else {
                even += (2 * x.unwrap() % 10) + (2 * x.unwrap() / 10);
            }
        }

        (odd + even) % 10 == 0
    }

    fn check_card(number: &str) -> bool;
}

struct Visa;

impl Card for Visa {
    fn check_card(number: &str) -> bool {
        let re = Regex::new(r"^4").unwrap();
        Visa::_validate(number) && number.len() == 16 && re.is_match(number)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_working() {
        assert_eq!(true, Visa::_validate("4242424242424242"));
    }

    #[test]
    fn visa_validate() {
        assert_eq!(true, Visa::check_card("4242424242424242"));
    }
}
