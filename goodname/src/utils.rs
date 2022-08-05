const UPPER_TO_LOWER: [u8; 256] = {
    let mut map = [0; 256];
    let mut c = b'A';
    while c <= b'Z' {
        map[c as usize] = c + (b'a' - b'A'); // To the lower one
        c += 1;
    }
    map
};

const LOWER_TO_UPPER: [u8; 256] = {
    let mut map = [0; 256];
    let mut c = b'a';
    while c <= b'z' {
        map[c as usize] = c - (b'a' - b'A'); // To the lower one
        c += 1;
    }
    map
};

#[inline(always)]
pub const fn is_lower_case(c: u8) -> bool {
    LOWER_TO_UPPER[c as usize] != 0
}

#[inline(always)]
pub const fn is_upper_case(c: u8) -> bool {
    UPPER_TO_LOWER[c as usize] != 0
}

#[inline(always)]
pub const fn to_lower_case(c: u8) -> Option<u8> {
    if is_upper_case(c) {
        Some(UPPER_TO_LOWER[c as usize])
    } else {
        None
    }
}

#[inline(always)]
pub const fn to_upper_case(c: u8) -> Option<u8> {
    if is_lower_case(c) {
        Some(LOWER_TO_UPPER[c as usize])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_case() {
        assert!(!is_lower_case(b'`'));
        assert!(is_lower_case(b'a'));
        assert!(is_lower_case(b'z'));
        assert!(!is_lower_case(b'{'));

        assert!(!is_upper_case(b'@'));
        assert!(is_upper_case(b'A'));
        assert!(is_upper_case(b'Z'));
        assert!(!is_upper_case(b'['));

        assert_eq!(to_lower_case(b'A'), Some(b'a'));
        assert_eq!(to_lower_case(b'Z'), Some(b'z'));
        assert_eq!(to_lower_case(b'a'), None);
        assert_eq!(to_lower_case(b'z'), None);

        assert_eq!(to_upper_case(b'A'), None);
        assert_eq!(to_upper_case(b'Z'), None);
        assert_eq!(to_upper_case(b'a'), Some(b'A'));
        assert_eq!(to_upper_case(b'z'), Some(b'Z'));
    }
}
