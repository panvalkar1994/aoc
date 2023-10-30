use std::collections::HashSet;

pub fn get_visited_houese_count(input: &str) -> usize {
    let mut set: HashSet<String> = HashSet::new();
    let mut pos = (0, 0);
    set.insert(get_position_str(pos.0, pos.1));
    input.chars().for_each(|c| match c {
        '>' => {
            pos.0 += 1;
            set.insert(get_position_str(pos.0, pos.1));
        }
        '^' => {
            pos.1 += 1;
            set.insert(get_position_str(pos.0, pos.1));
        }
        '<' => {
            pos.0 -= 1;
            set.insert(get_position_str(pos.0, pos.1));
        }
        'v' => {
            pos.1 -= 1;
            set.insert(get_position_str(pos.0, pos.1));
        }
        _ => unreachable!("not possible"),
    });
    set.len()
}

pub fn get_houses_with_atleast_a_visit(input: &str) -> usize {
    let mut set: HashSet<String> = HashSet::new();
    let mut santa = (0, 0);
    let mut robo = (0, 0);
    set.insert(get_position_str(0, 0));

    input.chars().enumerate().for_each(|(i, c)| match c {
        '>' => {
            if i % 2 == 0 {
                santa.0 += 1;
                set.insert(get_position_str(santa.0, santa.1));
            } else {
                robo.0 += 1;
                set.insert(get_position_str(robo.0, robo.1));
            }
        }
        '^' => {
            if i % 2 == 0 {
                santa.1 += 1;
                set.insert(get_position_str(santa.0, santa.1));
            } else {
                robo.1 += 1;
                set.insert(get_position_str(robo.0, robo.1));
            }
        }
        '<' => {
            if i % 2 == 0 {
                santa.0 -= 1;
                set.insert(get_position_str(santa.0, santa.1));
            } else {
                robo.0 -= 1;
                set.insert(get_position_str(robo.0, robo.1));
            }
        }
        'v' => {
            if i % 2 == 0 {
                santa.1 -= 1;
                set.insert(get_position_str(santa.0, santa.1));
            } else {
                robo.1 -= 1;
                set.insert(get_position_str(robo.0, robo.1));
            }
        }
        _ => unreachable!("not possible"),
    });

    set.len()
}

fn get_position_str(x: i32, y: i32) -> String {
    format!("{} - {}", x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_visited_houese_count() {
        let mut input = ">";
        assert_eq!(
            2,
            get_visited_houese_count(input),
            "visited count {}",
            get_visited_houese_count(input)
        );
        input = "^>v<";
        assert_eq!(
            4,
            get_visited_houese_count(input),
            "visited count {}",
            get_visited_houese_count(input)
        );

        input = "^v^v^v^v^v";
        assert_eq!(
            2,
            get_visited_houese_count(input),
            "visited count {}",
            get_visited_houese_count(input)
        );

        let input = std::fs::read_to_string("./inputs/2015_day3.txt").unwrap();
        assert_eq!(
            2081,
            get_visited_houese_count(&input),
            "visited count {}",
            get_visited_houese_count(&input)
        );
    }

    #[test]
    fn test_get_houses_with_atleast_a_visit() {
        let mut input = "^v";
        assert_eq!(
            3,
            get_houses_with_atleast_a_visit(input),
            "visited count {}",
            get_houses_with_atleast_a_visit(input)
        );
        input = "^>v<";
        assert_eq!(
            3,
            get_houses_with_atleast_a_visit(input),
            "visited count {}",
            get_houses_with_atleast_a_visit(input)
        );

        input = "^v^v^v^v^v";
        assert_eq!(
            11,
            get_houses_with_atleast_a_visit(input),
            "visited count {}",
            get_houses_with_atleast_a_visit(input)
        );

        let input = std::fs::read_to_string("./inputs/2015_day3.txt").unwrap();
        assert_eq!(
            2341,
            get_houses_with_atleast_a_visit(&input),
            "visited count {}",
            get_houses_with_atleast_a_visit(&input)
        );
    }
}
