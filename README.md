## Library of some math functions 

### Features: 
1. Function to output the calculation results as Vec<String>: pub fn output(v: Vec<(f64, f64)>) -> Vec<String>;
1. Function to record the calculation results to data.csv: pub fn file_output(out: &Vec<String>) -> Result<(), MfuncError> ; 
1. Dirichlet beta function: pub fn dirichlet_beta(x: f64, upper: usize) -> f64;
1. Factorial function n!: pub fn factorial(n: usize) -> usize;
1. Double factorial function n!!: pub fn double_factorial(n: usize) -> usize; 
1. Parity function, result either -1.0 or 1.0: pub fn parity(n: usize) -> f64;

### How to use this library: 
1. Add to Cargo.toml: 
```Toml
    [dependencies]
    mfunc = {git = "https://github.com/azavgo/mfunc", branch = "main"}
```
2. Calculate Dirichlet beta function at 1.0, 2.0, and 3.0:  
```Rust
    use mfunc::MfuncError;

    fn main() -> Result<(), MfuncError>{ 
        Ok(())
    }
  
```
