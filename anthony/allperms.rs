fn all_substrings(string: String) -> Vec<str> {
    let mut string: String = string;
    let mut substrings = Vec::new();

    for i in 0..string.length() {
        
    }
}

// fn ntiles (string: &str) -> u32 {
// }

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex0() {
        assert_eq!(8, ntiles("AAB"));
    }

    #[test]
    fn test_ex1() {
        assert_eq!(188, ntiles("AAABBC"));
    }

    #[test]
    fn test_ex2() {
        assert_eq!(1, ntiles("V"));
    }
}
