pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }

    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(1, 2), -1);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(1, 2), 2);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(1, 2), 0);
    }
}
