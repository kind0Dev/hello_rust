// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    match b {
        0 => None,
        _ => Some(a / b)
    }
    
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_clamp_n_is_gt(){
        let n = 20;
        let lower = 20;
        let higher = 100;
        let res = clamp(n, lower, higher);
        assert!(res >= lower, "result is not less than lower")
    }

    #[test]
    fn test_clamp_n_is_lt(){
        let n = 50;
        let lower = 20;
        let higher = 100;
        let res = clamp(n, lower, higher);
        assert!(res <= higher, "result is not less than lower")
    }

    #[test]
    fn test_div(){
        let a = 20;
        let b = 4;
        let res = div(a, b);
        match res {
            Some(res) => assert_eq!(res, 5, "divid not correct"),
            None => println!("noting to divide")
        }
    }

    #[test]
    fn test_div_0(){
        let a = 20;
        let b = 0;
        let res = div(a, b);
        match res {
            Some(res) => assert_eq!(res, 5, "divid not correct"),
            None => println!("noting to divide")
        }
    }

    #[test]
    fn test_concat_0(){
        let first = "raji";
        let second = "abdul";
        let res = concat(first, second);
        let expected = "rajiabdul";
       
        assert_eq!(res, expected, "did not concat properly");
        
    }

    #[test]
    fn test_concat_cap_lower(){
        let first = "RAJI";
        let second = "abdul";
        let res = concat(first, second);
        let expected = "RAJIabdul";
       
        assert_eq!(res, expected, "did not concat properly");
        
    }
}