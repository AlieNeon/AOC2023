use std::{collections::HashMap, fs};

fn main() {
  const FILE_PATH: &str = "input.txt";
  println!(
    "Welcome to AOC2023 day 8, first we will read the file {}",
    FILE_PATH
  );
  let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

  println!("And we parse the file!");
  let (instructions, map) = parse(&contents);
  println!("First thing: {}", calculate_1(&instructions, &map));
  println!("Second thing: {}", calculate_2(&instructions, &map));
}

fn parse(input: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
  (
    input.split("\n\n").nth(0).unwrap().chars().collect(),
    input
      .split("\n\n")
      .nth(1)
      .unwrap()
      .lines()
      .map(|line| {
        (
          line.split(" = ").nth(0).unwrap().to_string(),
          (
            line
              .split(" = ")
              .nth(1)
              .unwrap()
              .replace("(", "")
              .replace(")", "")
              .split(", ")
              .nth(0)
              .unwrap()
              .to_string(),
            line
              .split(" = ")
              .nth(1)
              .unwrap()
              .replace("(", "")
              .replace(")", "")
              .split(", ")
              .nth(1)
              .unwrap()
              .to_string(),
          ),
        )
      })
      .collect(),
  )
}

fn lcm(a: u64, b: u64) -> u64 {
  (a * b) / gcd(a, b)
}

fn gcd(first: u64, second: u64) -> u64 {
	let mut max = first;
	let mut min = second;
	if min > max {
			let val = max;
			max = min;
			min = val;
	}

	loop {
			let res = max % min;
			if res == 0 {
					return min;
			}

			max = min;
			min = res;
	}
}					

fn calculate(
  start: &str,
  instructions: &Vec<char>,
  map: &HashMap<String, (String, String)>,
	condition: &str
) -> u64 {
  let mut label = start.to_owned();
  let mut steps = 0;
  for c in Iterator::cycle(instructions.iter()) {
    if label.ends_with(condition) {
      break;
    }
    label = if c == &'R' {
      map.get(&label).unwrap().1.clone()
    } else {
      map.get(&label).unwrap().0.clone()
    };
    steps += 1;
  }
  steps
}

fn calculate_1(instructions: &Vec<char>, map: &HashMap<String, (String, String)>) -> u64 {
  calculate("AAA", instructions, map, "ZZZ")
}

fn calculate_2(instructions: &Vec<char>, map: &HashMap<String, (String, String)>) -> u64 {
  map
    .keys()
    .filter(|key| key.ends_with("A"))
    .map(|label| calculate(label, instructions, map, "Z"))
    .fold(1, |a, b| lcm(a, b))
}

#[cfg(test)]
mod test {
  const INPUT1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;
  const INPUT2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
  const INPUT3: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
  use super::*;
  #[test]
  fn test1_1() {
    let (instructions, map) = parse(INPUT1);
    assert_eq!(calculate_1(&instructions, &map), 2);
  }
  #[test]
  fn test1_2() {
    let (instructions, map) = parse(INPUT2);
    assert_eq!(calculate_1(&instructions, &map), 6);
  }
  #[test]
  fn test2() {
    let (instructions, map) = parse(INPUT3);
    assert_eq!(calculate_2(&instructions, &map), 6);
  }
}
