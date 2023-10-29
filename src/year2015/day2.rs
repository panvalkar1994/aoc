pub fn get_wrap_paper(input: &str) -> u32 {
    let numbers = get_dimensions(input);
    return 2 * (numbers[0] * numbers[1] + numbers[1] * numbers[2] + numbers[0] * numbers[2])
        + numbers[0] * numbers[1];
}

pub fn get_ribber_length(input: &str) -> u32 {
    let numbers = get_dimensions(input);
    return 2 * (numbers[0] + numbers[1]) + (numbers[0] * numbers[1] * numbers[2]);
}

fn get_dimensions(input: &str) -> Vec<u32> {
    let numbers: Vec<&str> = input.split('x').collect();
    let mut numbers: Vec<u32> = numbers
        .iter()
        .map(|s| {
            return s.to_owned().to_string().parse().unwrap();
        })
        .collect();

    numbers.sort();
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_wrap_papper() {
        let mut input = "2x3x4";
        assert_eq!(58, get_wrap_paper(input), "paper required for {}", input);
        input = "1x1x10";
        assert_eq!(43, get_wrap_paper(input), "paper required for {}", input);
    }

    #[test]
    fn test_day2() {
        let lines = std::fs::read_to_string("./inputs/2015_day2.txt").unwrap();
        let mut paper = 0;
        let mut ribben = 0;
        for line in lines.split('\n') {
            paper += get_wrap_paper(line);
            ribben += get_ribber_length(line);
        }
        assert_eq!(1586300, paper, "total paper : {}", paper);
        assert_eq!(3737498, ribben, "total ribben : {}", ribben);
    }
}
