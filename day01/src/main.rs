use std::{fs, ops::Add};

fn main() {
  const FILE_PATH: &str = "input.txt";
  println!(
    "Welcome to AOC2023 day 1, first we will read the file {}",
    FILE_PATH
  );
  let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

  println!("And we parse the file!");
  println!("First thing: {}", calculate(&parse(&contents)));
  println!(
    "Second thing: {}",
    calculate(&parse(&text2digits(&contents)))
  );
}

fn parse(input: &str) -> Vec<i32> {
  input
    .lines()
    .map(|line| {
      let digits = line.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();
      let len_digits = digits.len() - 1;
      let number: String = digits[0].to_string().add(&digits[len_digits].to_string());
      number.parse().expect("NAN")
    })
    .collect::<Vec<_>>()
}

const NUMTEXT: [(&str, &str); 13] = [
  ("e", "ee"),
  ("t", "tt"),
  ("o", "oo"),
  ("n", "nn"),
  ("onne", "1"),
  ("two", "2"),
  ("three", "3"),
  ("foour", "4"),
  ("five", "5"),
  ("six", "6"),
  ("seeveen", "7"),
  ("eight", "8"),
  ("ninne", "9"),
];
fn text2digits(input: &str) -> String {
  input
    .lines()
    .map(|line| {
      let mut line: String = line.to_string();
      for (name, num) in NUMTEXT {
        line = line.replacen(name, num, 99999);
      }
      line.add("\n")
    })
    .collect()
}

fn calculate(numbers: &Vec<i32>) -> i32 {
  numbers.iter().sum()
}

#[cfg(test)]
mod test {
  const INPUT1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

  const INPUT2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
  use super::*;
  #[test]
  fn test1() {
    let tst = parse(INPUT1);
    assert_eq!(calculate(&tst), 142);
  }
  #[test]
  fn test2() {
    let tst = parse(&text2digits(INPUT2));
    assert_eq!(calculate(&tst), 281);
  }
}
