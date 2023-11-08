use std::collections::HashMap;
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Parameter {
    Value(u16),
    Wire(String),
}

impl Parameter {
    pub fn resolve(&self, wire_map: &HashMap<String, u16>) -> Option<u16> {
        match self {
            Parameter::Value(v) => Some(*v),
            Parameter::Wire(s) => wire_map.get(s).copied(),
        }
    }
}

impl From<u16> for Parameter {
    fn from(value: u16) -> Self {
        Self::Value(value)
    }
}

impl From<&str> for Parameter {
    fn from(value: &str) -> Self {
        if let Ok(v) = value.parse() {
            Self::Value(v)
        } else {
            Self::Wire(value.to_string())
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Gate {
    Direct(Parameter),
    And(Parameter, Parameter),
    Or(Parameter, Parameter),
    Lshift(Parameter, u16),
    Rshift(Parameter, u16),
    Not(Parameter),
}

impl Gate {
    fn parse(s: &str) -> Self {
        let v: Vec<&str> = s.split_ascii_whitespace().collect();
        assert!(!v.is_empty() && v.len() <= 3);
        match v.len() {
            1 => {
                let val = *v.get(0).unwrap();
                Self::Direct(val.into())
            }
            2 => {
                assert_eq!(v.get(0), Some(&"NOT"));
                let val = *v.get(1).unwrap();
                Self::Not(val.into())
            }
            3 => {
                let left = *v.get(0).unwrap();
                let right = *v.get(2).unwrap();
                match *v.get(1).unwrap() {
                    "AND" => Self::And(left.into(), right.into()),
                    "OR" => Self::Or(left.into(), right.into()),
                    "LSHIFT" => Self::Lshift(left.into(), right.parse().unwrap()),
                    "RSHIFT" => Self::Rshift(left.into(), right.parse().unwrap()),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!("not possible"),
        }
    }

    fn resolve(&self, wire_map: &HashMap<String, u16>) -> Option<u16> {
        Some(match self {
            Gate::Direct(v) => v.resolve(wire_map)?,
            Gate::And(p, q) => p.resolve(wire_map)? & q.resolve(wire_map)?,
            Gate::Or(p, q) => p.resolve(wire_map)? | q.resolve(wire_map)?,
            Gate::Lshift(p, q) => p.resolve(wire_map)? << q,
            Gate::Rshift(p, q) => p.resolve(wire_map)? >> q,
            Gate::Not(p) => !p.resolve(wire_map)?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Instruction {
    input: Gate,
    output: String,
}

impl Instruction {
    pub fn parse(s: &str) -> Self {
        let v: Vec<&str> = s.split(" -> ").collect();
        Self {
            input: Gate::parse(v.get(0).unwrap()),
            output: v.get(1).unwrap().to_string(),
        }
    }
}

pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| Instruction::parse(l)).collect()
}

pub struct WireState {
    pub map: HashMap<String, u16>,
}

impl WireState {
    pub fn from_instructions(instructions: &[Instruction]) -> Self {
        let mut map = HashMap::new();

        'outer: loop {
            for instruction in instructions {
                if map.contains_key(&instruction.output) {
                    continue;
                }

                if let Some(v) = instruction.input.resolve(&map) {
                    map.insert(instruction.output.to_string(), v);
                }

                if map.len() == instructions.len() {
                    break 'outer;
                }
            }
        }

        Self { map }
    }

    pub fn get(&self, wire: &str) -> Option<u16> {
        self.map.get(wire).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_parse() {
        let examples = vec![
            (
                "123 -> x",
                Instruction {
                    input: Gate::Direct(123.into()),
                    output: "x".into(),
                },
            ),
            (
                "ab -> y",
                Instruction {
                    input: Gate::Direct("ab".into()),
                    output: "y".into(),
                },
            ),
            (
                "x AND y -> d",
                Instruction {
                    input: Gate::And("x".into(), "y".into()),
                    output: "d".into(),
                },
            ),
            (
                "x OR y -> e",
                Instruction {
                    input: Gate::Or("x".into(), "y".into()),
                    output: "e".into(),
                },
            ),
            (
                "x LSHIFT 2 -> f",
                Instruction {
                    input: Gate::Lshift("x".into(), 2),
                    output: "f".into(),
                },
            ),
            (
                "y RSHIFT 2 -> g",
                Instruction {
                    input: Gate::Rshift("y".into(), 2),
                    output: "g".into(),
                },
            ),
            (
                "NOT x -> h",
                Instruction {
                    input: Gate::Not("x".into()),
                    output: "h".into(),
                },
            ),
        ];

        for (input, expected) in examples {
            assert_eq!(Instruction::parse(input), expected);
        }
    }

    #[test]
    fn example_part1() {
        let input = r#"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"#;

        let instructions = parse_instructions(input);

        let wire_state = WireState::from_instructions(&instructions);

        assert_eq!(wire_state.get("d"), Some(72));
        assert_eq!(wire_state.get("e"), Some(507));
        assert_eq!(wire_state.get("f"), Some(492));
        assert_eq!(wire_state.get("g"), Some(114));
        assert_eq!(wire_state.get("h"), Some(65412));
        assert_eq!(wire_state.get("i"), Some(65079));
        assert_eq!(wire_state.get("x"), Some(123));
        assert_eq!(wire_state.get("y"), Some(456));
    }

    #[test]
    fn real_input() {
        let lines = std::fs::read_to_string("./inputs/2015_day7.txt").unwrap();

        let instructions = parse_instructions(&lines);
        let a = WireState::from_instructions(&instructions)
            .get("a")
            .unwrap();
        assert_eq!(a, 3176 as u16);
        let mut manipulated_ins = instructions.to_vec();
        manipulated_ins
            .iter_mut()
            .filter(|it| it.output == "b")
            .for_each(|ir| ir.input = Gate::Direct(Parameter::Value(a)));

        let next_a = WireState::from_instructions(&manipulated_ins)
            .get("a")
            .unwrap();
        assert_eq!(next_a, 14710 as u16);
    }
}
