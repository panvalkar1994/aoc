pub fn count_on_bulbs(grid: &[[bool; 1000]; 1000]) -> i32 {
    let mut count = 0;
    for row in grid {
        for bulb in row {
            if *bulb {
                count += 1;
            }
        }
    }
    count
}

pub fn calculate_total_brightness(grid: &[[u16; 1000]; 1000]) -> usize {
    let mut count = 0;
    for row in grid {
        for bulb in row {
            count += *bulb as usize;
        }
    }
    count
}

pub fn update_grid_state(text: &str, grid: &mut [[bool; 1000]; 1000]) {
    let instuction = parse_instruction(text);
    grid_update(grid, instuction);
}

pub fn update_grid_brightness(text: &str, grid: &mut [[u16; 1000]; 1000]) {
    let instuction = parse_instruction(text);
    update_brightness(grid, instuction);
}

fn update_brightness(grid: &mut [[u16; 1000]; 1000], instruction: Instruction) {
    for i in instruction.start[0]..=instruction.end[0] {
        for j in instruction.start[1]..=instruction.end[1] {
            match instruction.option {
                Action::On => {
                    grid[i][j] += 1;
                }
                Action::Off => {
                    if grid[i][j] == 0 {
                        continue;
                    }
                    grid[i][j] -= 1;
                }
                Action::Toggle => {
                    grid[i][j] += 2;
                }
            }
        }
    }
}

struct Instruction {
    start: [usize; 2],
    end: [usize; 2],
    option: Action,
}

enum Action {
    On,
    Off,
    Toggle,
}

fn parse_instruction(instruction: &str) -> Instruction {
    let instructions: Vec<&str> = instruction.split_whitespace().collect();
    let mut start: [usize; 2] = [0, 0];
    let mut end: [usize; 2] = [0, 0];
    let mut option = Action::Toggle;
    match instructions[0 as usize] {
        "turn" => match instructions[1 as usize] {
            "on" => {
                let first: Vec<&str> = instructions[2].split(",").collect();
                let sec: Vec<&str> = instructions[4].split(",").collect();
                start[0] = first[0 as usize].parse().unwrap();
                start[1] = first[1 as usize].parse().unwrap();
                end[0] = sec[0 as usize].parse().unwrap();
                end[1] = sec[1 as usize].parse().unwrap();
                option = Action::On;
            }
            "off" => {
                let first: Vec<&str> = instructions[2].split(",").collect();
                let sec: Vec<&str> = instructions[4].split(",").collect();
                start[0] = first[0 as usize].parse().unwrap();
                start[1] = first[1 as usize].parse().unwrap();
                end[0] = sec[0 as usize].parse().unwrap();
                end[1] = sec[1 as usize].parse().unwrap();
                option = Action::Off;
            }
            _ => {}
        },
        "toggle" => {
            let first: Vec<&str> = instructions[1].split(",").collect();
            let sec: Vec<&str> = instructions[3].split(",").collect();
            start[0] = first[0 as usize].parse().unwrap();
            start[1] = first[1 as usize].parse().unwrap();
            end[0] = sec[0 as usize].parse().unwrap();
            end[1] = sec[1 as usize].parse().unwrap();
        }
        _ => {}
    };

    Instruction { start, end, option }
}

fn grid_update(grid: &mut [[bool; 1000]; 1000], instruction: Instruction) {
    for i in instruction.start[0]..=instruction.end[0] {
        for j in instruction.start[1]..=instruction.end[1] {
            match instruction.option {
                Action::On => {
                    grid[i][j] = true;
                }
                Action::Off => {
                    grid[i][j] = false;
                }
                Action::Toggle => {
                    grid[i][j] = !grid[i][j];
                }
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_count_all_bulbs() {
        let lines = std::fs::read_to_string("./inputs/2015_day6.txt").unwrap();
        let mut grid = [[false; 1000]; 1000];
        for line in lines.split('\n') {
            update_grid_state(line, &mut grid);
        }

        let on_bulbs = 400410;
        let result = count_on_bulbs(&grid);

        assert_eq!(
            on_bulbs, result,
            "expected {}, got {} bulbs on",
            on_bulbs, result
        );
    }

    #[test]
    fn test_total_brightness() {
        let lines = std::fs::read_to_string("./inputs/2015_day6.txt").unwrap();
        let mut grid = [[0_u16; 1000]; 1000];
        for line in lines.split('\n') {
            dbg!("{}", &line);
            update_grid_brightness(line, &mut grid);
        }

        let brightness = 15343601;
        let result = calculate_total_brightness(&grid);

        assert_eq!(
            brightness, result,
            "expected {}, got {} bulbs on",
            brightness, result
        );
    }
}
