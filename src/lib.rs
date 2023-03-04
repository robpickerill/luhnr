use rand::distributions::{Distribution, Uniform};

#[derive(PartialEq, Eq, Debug)]
pub enum LuhnError {
    InvalidLength,
    InvalidPrefix,
}

const DOUBLERESULT: [u8; 10] = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];

pub fn validate(number: &[u8]) -> bool {
    match number.len() {
        0 => false,
        1 => matches!(number[0], 0),
        _ => number[number.len() - 1] == calculate_luhn_sum(&number[..(number.len() - 1)]),
    }
}

pub fn generate_with_prefix(length: usize, prefix: &[u8]) -> Result<Vec<u8>, LuhnError> {
    if length < 1 || prefix.len() > length {
        return Err(LuhnError::InvalidLength);
    };

    for &x in prefix {
        if x > 9 {
            return Err(LuhnError::InvalidPrefix);
        }
    }

    let step = Uniform::from(0..10); // half-open range
    let mut rng = rand::thread_rng();

    let mut number = vec![0; length];
    number[..prefix.len()].copy_from_slice(prefix);

    for num in number.iter_mut().take(length - 1).skip(prefix.len()) {
        *num = step.sample(&mut rng);
    }

    number[length - 1] = calculate_luhn_sum(&number[..length - 1]);
    Ok(number)
}

pub fn generate(length: usize) -> Result<Vec<u8>, LuhnError> {
    generate_with_prefix(length, &[])
}

fn calculate_luhn_sum(number: &[u8]) -> u8 {
    let mut double = true;
    let mut sum: usize = 0;

    for digit in number.iter().rev() {
        sum += if double {
            DOUBLERESULT[*digit as usize] as usize
        } else {
            *digit as usize
        };

        double = !double;
    }

    let checksum = sum % 10;
    match checksum {
        0 => 0,
        _ => 10 - checksum as u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_length() {
        assert_eq!(generate(16).unwrap().len(), 16)
    }

    #[test]
    fn test_generate_long() {
        assert_eq!(generate(2000000).unwrap().len(), 2000000)
    }

    #[test]
    fn test_generate_prefix_length() {
        let prefix = vec![0, 1];
        assert_eq!(generate_with_prefix(16, &prefix).unwrap().len(), 16)
    }

    #[test]
    fn test_generate_prefix() {
        let prefix = vec![0, 1];
        let result = generate_with_prefix(16, &prefix).unwrap();
        assert_eq!(result[..2], prefix)
    }

    #[test]
    fn test_generate_invalid_length() {
        let prefix = vec![0, 1];
        match generate_with_prefix(1, &prefix) {
            Err(error) => assert_eq!(error, LuhnError::InvalidLength),
            Ok(_) => panic!("failed to catch empty length"),
        }
    }

    #[test]
    fn test_generate_invalid_prefix() {
        let prefix = vec![20, 10];
        match generate_with_prefix(10, &prefix) {
            Err(error) => assert_eq!(error, LuhnError::InvalidPrefix),
            Ok(_) => panic!("failed to catch prefix error"),
        }
    }

    #[test]
    fn test_generate() {
        let prefix = vec![0, 1, 8, 9, 9, 5, 3, 6, 6, 4, 5, 7, 1, 5, 3];
        let result = [prefix.clone(), vec![9]].concat();
        match generate_with_prefix(16, &prefix) {
            Ok(v) => assert_eq!(result, v),
            Err(_) => panic!("unexpected err result"),
        }
    }

    #[test]
    fn test_validate_true() {
        let number = vec![0, 1, 8, 9, 9, 5, 3, 6, 6, 4, 5, 7, 1, 5, 3, 9];
        assert!(validate(&number))
    }

    #[test]
    fn test_validate_false() {
        let number = vec![0, 1, 8, 9, 9, 5, 3, 6, 6, 4, 5, 7, 1, 5, 3, 5];
        assert!(!validate(&number))
    }

    #[test]
    fn test_validate_empty() {
        let number = vec![];
        assert!(!validate(&number))
    }

    #[test]
    fn test_validate_short_true() {
        let number = vec![0];
        assert!(validate(&number))
    }

    #[test]
    fn test_validate_short_false() {
        let number = vec![2];
        assert!(!validate(&number))
    }

    #[test]
    fn test_validate_visa_test() {
        let number = vec![4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2, 4, 2];
        assert!(validate(&number))
    }

    #[test]
    fn test_generate_validate() {
        (0..100).for_each(|_| {
            let number = generate(16).unwrap();
            assert!(validate(&number))
        })
    }
}
