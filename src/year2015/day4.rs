pub fn get_hash_starts_with(key: &str, start: &str) -> usize {
    let mut value = 1;
    loop {
        let data = format!("{}{}", key, value);
        let digest = md5::compute(data);
        if format!("{:x}", digest).starts_with(start) {
            return value;
        }
        value += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_hash_input() {
        let mut key = "abcdef";
        assert_eq!(
            609043 as usize,
            get_hash_starts_with(key, "00000"),
            "found value for {}",
            key
        );
        key = "pqrstuv";
        assert_eq!(
            1048970 as usize,
            get_hash_starts_with(key, "00000"),
            "found value for {}",
            key
        );
        key = "bgvyzdsv";
        assert_eq!(
            254575 as usize,
            get_hash_starts_with(key, "00000"),
            "found value for {}",
            key
        );
        assert_eq!(
            1038736 as usize,
            get_hash_starts_with(key, "000000"),
            "found value for {}",
            key
        );
    }
}
