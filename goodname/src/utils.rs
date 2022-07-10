const UPPER_TO_LOWER: [u8; 256] = {
    let mut map = [0; 256];
    let mut i = 1;
    while i < 256 {
        let c = i as u8;
        if b'A' <= c && c <= b'Z' {
            map[i] = c + (b'a' - b'A'); // To the lower one
        }
        i += 1;
    }
    map
};

pub const fn is_upper_case(c: u8) -> bool {
    UPPER_TO_LOWER[c as usize] != 0
}

pub const fn to_lower_case(c: u8) -> u8 {
    if is_upper_case(c) {
        UPPER_TO_LOWER[c as usize]
    } else {
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_case() {
        assert!(!is_upper_case(b'@'));
        assert!(is_upper_case(b'A'));
        assert!(is_upper_case(b'Z'));
        assert!(!is_upper_case(b'['));

        assert_eq!(to_lower_case(b'A'), b'a');
        assert_eq!(to_lower_case(b'Z'), b'z');
        assert_eq!(to_lower_case(b'a'), b'a');
        assert_eq!(to_lower_case(b'z'), b'z');
    }
}
