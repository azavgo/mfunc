//Standard factorial function
pub fn factorial(n: usize) -> usize {
    match n {
        0 => 1,
        1 => 1,
        _ => {
            let mut m: usize = 1;
            for i in 2..=n {
                m = m * i;
            }
            m
        }
    }
}

//Next step is to implement n!!


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_01() {
        let result = factorial(3);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_factorial_02() {
        let result = factorial(0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_factorial_03() {
        let result = factorial(1);
        assert_eq!(result, 1);
    }
}
