use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub enum MfuncError { 
    IOError(std::io::Error),  
}

impl From<std::io::Error> for MfuncError {
    fn from(error: std::io::Error) -> Self {
        MfuncError::IOError(error)
    }
}

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

//Helper parity function checks (-1)^n result based on n
pub fn parity(n: usize) -> f64 {
    match n % 2 {
        0 => 1.0, 
        _ => -1.0,
    }
}

//Dirichlet beta function at x, where upper is the upper limit 
//of the summation 
//https://en.wikipedia.org/wiki/Dirichlet_beta_function 
//Tests suggest that: 
//for x >= 1/2: upper >= 200,000 to get 3 correct decimal points 
//for x = [1/3, 1/2) : upper >= 300,000,000 to get 3 correct decimal points
//for x = [1/4, 1/3) : upper >= 9,000,000,000 to get 2 correct decimal points
pub fn dirichlet_beta(x: f64, upper: usize) -> f64 {
    let mut y: f64 = 0.0;  
    for n in 0..=upper {
        y += parity(n) * 1.0 / (1.0 + 2.0 * n as f64).powf(x); 
    }
    y
}

//*****************************************************
//Helper function to convert the results of the calculations 
//into the vector of strings
//This vector of strings will be recorded as a CSV data file
pub fn output(v: Vec<(f64, f64)>) -> Vec<String>{
    let mut out: Vec<String> = Vec::new(); 
    for e in v {
        out.push(format!("{:.2}, {:.2}", e.0, e.1));
    }
    out
}

//Function that creates a csv text file, opens it to write the 
//output of a calculation in the form x, y
//This file then should be readable by gnuplot for plotting
pub fn file_output(out: &Vec<String>) -> Result<(), MfuncError> {
    let path = "data.csv"; 
    let mut output = File::create(path)?;
    write!(output, "{}", out.join("\n"))?;
    Ok(())
}
//*****************************************************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_01() {
        let mut v: Vec<(f64, f64)> = Vec::new();
        v.push((2.0, 1.0)); 
        v.push((0.0, -3.0));
        let out = output(v);
        file_output(&out).unwrap();
        assert_eq!("2.00, 1.00".to_string(), out[0]);
        assert_eq!("0.00, -3.00".to_string(), out[1]);
    }

    #[test]
    fn test_dirichlet_beta_01() {
        //approximate result at x = 1: 0.7853981633974483096156608
        //upper = 2453 is the min value for the 0.785 approximation
        let result = dirichlet_beta(1.0, 200000);
        assert_eq!(format!("{:.3}", result), "0.785".to_string());
    }

    #[test]
    fn test_dirichlet_beta_02() {
        //approximate result at x = 0.5: 0.6676914571896091766586909 	
        let result = dirichlet_beta(0.5, 200000);
        assert_eq!(format!("{:.3}", result), "0.668".to_string());
    }

    #[test]
    fn test_dirichlet_beta_03() {
        //approximate result at x = 1/3: 0.6178550888488520660725389 	
        let input: f64 = 1.0 / 3.0; 
        let result = dirichlet_beta(input, 300000000);
        assert_eq!(format!("{:.3}", result), "0.618".to_string());
    }
/* This is to save time when I run all tests 
    #[test]
    fn test_dirichlet_beta_04() {
        //approximate result at x = 0.25: 0.5907230564424947318659591 	
        //upper = 9B still results at 0.59209 vs 0.591
        let input: f64 = 0.25; 
        let result = dirichlet_beta(input, 9000000000);
        assert_eq!(format!("{:.5}", result), "0.591".to_string());
    }
*/

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
