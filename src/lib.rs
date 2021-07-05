use regex::Regex;

pub trait Card {
    fn check_sum(number: &str) -> bool {
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

    fn validate(number: &str) -> bool;
}

struct Visa;
struct MasterCard;
struct AmEx;
struct Discovery;
struct UnionPay;
struct Diners;
struct JCB;

impl Card for Visa {
    fn validate(number: &str) -> bool {
        let re = Regex::new(r"^4").unwrap();
        Visa::check_sum(number) && number.len() == 16 && re.is_match(number)
    }
}

impl Card for MasterCard {
    fn validate(number: &str) -> bool {
        let re = Regex::new(r"^(51|52|53|54|55|22|23|24|25|26|27)").unwrap();
        MasterCard::check_sum(number) && number.len() == 16 && re.is_match(number)
    }
}

impl Card for AmEx {
    fn validate(number: &str) -> bool {
        let re = Regex::new(r"^(34|37)").unwrap();
        AmEx::check_sum(number) && number.len() == 15 && re.is_match(number)
    }
}

impl Card for Discovery {
    fn validate(number: &str) -> bool {
        let re = Regex::new(r"^(60|64|65)").unwrap();
        Discovery::check_sum(number) && number.len() == 16 && re.is_match(number)
    }
}

impl Card for Diners {
    fn validate(number: &str) -> bool {
        let re = Regex::new(r"^(30|36|38|39)").unwrap();
        UnionPay::check_sum(number)
            && (number.len() == 14 || number.len() == 16)
            && re.is_match(number)
    }
}

impl Card for UnionPay {
    fn validate(number: &str) -> bool {
        let re = Regex::new(r"^62").unwrap();
        UnionPay::check_sum(number) && number.len() == 16 && re.is_match(number)
    }
}

impl Card for JCB {
    fn validate(number: &str) -> bool {
        let re = Regex::new("r^(60|64|65)").unwrap();
        JCB::check_sum(number) && number.len() == 16 && re.is_match(number)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn visa_validate() {
        assert_eq!(true, Visa::validate("4242424242424242"));
    }
}
