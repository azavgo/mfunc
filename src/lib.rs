//Standard factorial function n!
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

//Standard double factorial function n!!
pub fn double_factorial(n: usize) -> usize {
    match n {
        0 => 1, 
        1 => 1, 
        2 => 2, 
        3 => 3, 
        _ => {
                let mut m: usize = n;
                let mut i = n - 2;  
                while i > 1 {
                    m *= i;
                    i -= 2; 
                } 
                m
             },
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_factorial_01() {
        let result = double_factorial(6);
        assert_eq!(result, 48);
    }

    #[test]
    fn test_double_factorial_02() {
        let result = double_factorial(4);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_double_factorial_03() {
        let result = double_factorial(2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_double_factorial_04() {
        let result = double_factorial(0);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_double_factorial_05() {
        let result = double_factorial(14);
        assert_eq!(result, 645120);
    }

    #[test]
    fn test_double_factorial_06() {
        let result = double_factorial(7);
        assert_eq!(result, 105);
    }

    #[test]
    fn test_double_factorial_07() {
        let result = double_factorial(5);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_double_factorial_08() {
        let result = double_factorial(3);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_double_factorial_09() {
        let result = double_factorial(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_double_factorial_10() {
        let result = double_factorial(13);
        assert_eq!(result, 135135);
    }

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
