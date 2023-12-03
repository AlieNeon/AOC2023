use regex::Regex;
use std::{
  collections::{HashMap, HashSet},
  fs,
};

fn main() {
  const FILE_PATH: &str = "input.txt";
  println!(
    "Welcome to AOC2023 day 3, first we will read the file {}",
    FILE_PATH
  );
  let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

  println!("And we parse the file!");
  let data = parse(&contents);
  println!("First thing: {}", calculate_1(&data));
  println!("Second thing: {}", calculate_2(&data));
}

#[derive(Debug)]
struct Part {
  symbol: char,
  value: Vec<i32>,
}

fn parse(input: &str) -> Vec<Part> {
  let renum = Regex::new(r"(?<number>\d+)").expect("Invalid regex");
  let resym = Regex::new(r"(?<symbol>[^\d\.]+)").expect("Invalid regex");
  let mut number_map: HashMap<(usize, usize), (i32, Vec<_>)> = HashMap::new();
  let mut symbol_map: HashMap<(usize, usize), &str> = HashMap::new();
  for (line_number, line) in input.lines().enumerate() {
    for cap in renum.captures_iter(line) {
      let capture = cap.name("number").unwrap();
      for col in capture.range() {
        number_map.insert(
          (line_number, col),
          (
            capture.as_str().parse().expect("NAN"),
            capture.range().collect(),
          ),
        );
      }
    }
    for cap in resym.captures_iter(line) {
      let capture = cap.name("symbol").unwrap();
      for col in capture.range() {
        symbol_map.insert((line_number, col), capture.as_str());
      }
    }
  }
  let mut list_parts: Vec<Part> = Vec::new();
  for ((x, y), symbol) in &symbol_map {
    let mut list_nums: Vec<i32> = Vec::new();
    for i in -1..=1 {
      let i = (*x as isize + i) as usize;
      for j in -1..=1 {
        let j = (*y as isize + j) as usize;
        if let Some(number) = number_map.get(&(i, j)) {
          list_nums.push(number.0);
          let numbers = &number.1.clone();
          for num in numbers {
            number_map.remove(&(i, *num));
          }
        }
      }
    }
    // Add the part with all the adjacent numbers at the end so we can do part 2
    list_parts.push(Part {
      symbol: symbol.chars().next().expect("char must exist"),
      value: list_nums,
    })
  }
  // So sad it was not usefull in the end
  let number_set: HashSet<_> = number_map.values().collect();
  for number in number_set {
    list_parts.push(Part {
      symbol: '.',
      value: vec![number.0],
    });
  }
  // Return
  list_parts
}

fn calculate_1(parts: &Vec<Part>) -> i32 {
  parts
    .iter()
    .filter(|part| part.symbol != '.')
    .map(|part| part.value.iter().sum::<i32>())
    .sum()
}

fn calculate_2(parts: &Vec<Part>) -> i32 {
  parts
    .iter()
    .filter(|part| part.symbol == '*' && part.value.len() == 2)
    .map(|part| part.value[0] * part.value[1])
    .sum()
}

#[cfg(test)]
mod test {
  const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
  use super::*;
  #[test]
  fn test1() {
    let tst = parse(INPUT);
    assert_eq!(calculate_1(&tst), 4361);
  }
  #[test]
  fn test2() {
    let tst = parse(INPUT);
    assert_eq!(calculate_2(&tst), 467835);
  }
}
