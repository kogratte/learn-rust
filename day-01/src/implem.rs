pub fn rotate(start: i32, direction: char, offset: i32) -> i32 {
    match direction {
        'L' => (start - offset).rem_euclid(100),
        'R' => (start + offset).rem_euclid(100),
        _ => start,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let result = rotate(50, 'L', 68);
        assert_eq!(result, 82, "Testing 50 L68: {}", result);

        let result = rotate(82, 'L', 30);
        assert_eq!(result, 52, "Testing 82 L30: {}", result);

        let result = rotate(52, 'R', 48);
        assert_eq!(result, 0, "Testing 52 R48: {}", result);

        let result = rotate(0, 'L', 5);
        assert_eq!(result, 95, "Testing 0 L5: {}", result);
    }
}
