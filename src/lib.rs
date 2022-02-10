use rand::distributions::{Distribution, Uniform};

#[derive(PartialEq, Debug)]
pub enum LuhnError {
    InvalidLength,
    InvalidPrefix,
}

const DOUBLERESULT: [usize; 10] = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];

pub fn generate(length: usize, prefix: &[usize]) -> Result<Vec<usize>, LuhnError> {
    if length < 1 || prefix.len() > length {
        return Err(LuhnError::InvalidLength);
    };

    for &x in prefix {
        if x > 9 {
            return Err(LuhnError::InvalidPrefix);
        }
    }

    let step = Uniform::from(1..10);
    let mut rng = rand::thread_rng();

    let mut number: Vec<usize> = Vec::with_capacity(length);
    number.extend_from_slice(prefix);

    let mut n = prefix.len();
    while n < length - 1 {
        number.push(step.sample(&mut rng));
        n += 1;
    }

    number.push(calculate_luhn_sum(number.clone()));
    Ok(number)
}

fn calculate_luhn_sum(number: Vec<usize>) -> usize {
    let mut n = number.len();
    let mut double = true;
    let mut sum = 0;

    while n > 0 {
        sum += match double {
            true => DOUBLERESULT[number[n - 1]],
            false => number[n - 1],
        };

        double = !double;
        n -= 1;
    }

    let checksum = sum % 10;
    match checksum {
        0 => 0,
        _ => 10 - checksum,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_invalid_length() {
        let prefix = vec![0, 1];
        match generate(1, &prefix) {
            Err(error) => assert_eq!(error, LuhnError::InvalidLength),
            Ok(_) => panic!("failed to catch empty length"),
        }
    }

    #[test]
    fn test_generate_invalid_prefix() {
        let prefix = vec![20, 10];
        match generate(10, &prefix) {
            Err(error) => assert_eq!(error, LuhnError::InvalidPrefix),
            Ok(_) => panic!("failed to catch invalid prefix"),
        }
    }

    #[test]
    fn test_generate() {
        let prefix = vec![0, 1, 8, 9, 9, 5, 3, 6, 6, 4, 5, 7, 1, 5, 3];
        let result = [prefix.clone(), vec![9]].concat();
        match generate(16, &prefix) {
            Ok(v) => assert_eq!(result, v),
            Err(_) => panic!("unexpected err result"),
        }
    }
}
