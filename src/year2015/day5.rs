pub fn is_nice(input: &str) -> bool {
    let taboo = vec!["ab", "cd", "pq", "xy"];
    for taboo_word in taboo {
        if input.contains(taboo_word) {
            return false;
        }
    }

    let mut vowel_count = 0;
    for c in input.chars() {
        match c {
            'a' | 'e' | 'o' | 'u' | 'i' => {
                vowel_count += 1;
            }
            _ => {}
        }
    }

    if vowel_count < 3 {
        return false;
    }

    let chs: Vec<char> = input.chars().collect();
    for i in 0..input.len() - 1 {
        if chs[i] == chs[i + 1] {
            return true;
        }
    }

    return false;
}

pub fn is_new_nice_str(input: &str) -> bool {
    let mut pair_found = false;
    let letters: Vec<char> = input.chars().collect();
    for i in 0..input.len() - 2 {
        let sub = &input[i..i + 2];
        if input[i + 2..input.len()].contains(sub) {
            pair_found = true;
        }
    }

    if !pair_found {
        return false;
    }

    for i in 0..input.len() - 2 {
        if letters[i] == letters[i + 2] {
            return true;
        }
    }
    false
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice_strings() {
        let mut input = "ugknbfddgicrmopn";
        assert_eq!(
            true,
            is_nice(input),
            "{} is nice ? {}",
            input,
            is_nice(input)
        );
        input = "aaa";
        assert_eq!(
            true,
            is_nice(input),
            "{} is nice ? {}",
            input,
            is_nice(input)
        );

        input = "jchzalrnumimnmhp";
        assert_eq!(
            false,
            is_nice(input),
            "{} is nice ? {}",
            input,
            is_nice(input)
        );

        input = "haegwjzuvuyypxyu";
        assert_eq!(
            false,
            is_nice(input),
            "{} is nice ? {}",
            input,
            is_nice(input)
        );
        input = "dvszwmarrgswjxmb";
        assert_eq!(
            false,
            is_nice(input),
            "{} is nice ? {}",
            input,
            is_nice(input)
        );

        let lines = std::fs::read_to_string("./inputs/2015_day5.txt").unwrap();

        let mut nice_str_count = 0;
        let expect = 238;
        for line in lines.split('\n') {
            if is_nice(line) {
                nice_str_count += 1;
            }
        }

        assert_eq!(
            expect, nice_str_count,
            "Actual nice string count {}",
            nice_str_count
        );
    }

    #[test]
    fn test_new_nice_strings() {
        let mut input = "qjhvhtzxzqqjkmpb";
        let mut result = is_new_nice_str(input);
        assert_eq!(true, result, "{} is nice? {}", input, result);

        input = "xxyxx";
        result = is_new_nice_str(input);
        assert_eq!(true, result, "{} is nice? {}", input, result);

        input = "uurcxstgmygtbstg";
        result = is_new_nice_str(input);
        assert_eq!(false, result, "{} is nice? {}", input, result);

        input = "ieodomkazucvgmuy";
        result = is_new_nice_str(input);
        assert_eq!(false, result, "{} is nice? {}", input, result);

        let lines = std::fs::read_to_string("./inputs/2015_day5.txt").unwrap();

        let mut nice_str_count = 0;
        let expect = 69;
        for line in lines.split('\n') {
            if is_new_nice_str(line) {
                nice_str_count += 1;
            }
        }

        assert_eq!(
            expect, nice_str_count,
            "Actual nice string count {}",
            nice_str_count
        );
    }
}
