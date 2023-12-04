use std::{collections::HashMap, fs};

fn main() {
  const FILE_PATH: &str = "input.txt";
  println!(
    "Welcome to AOC2023 day NUMBER, first we will read the file {}",
    FILE_PATH
  );
  let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

  println!("And we parse the file!");
  let data = parse(&contents);
  println!("First thing: {}", calculate_1(&data));
  println!("Second thing: {}", calculate_2(&data));
}

#[derive(Debug)]
struct Lotto {
  id: i32,
  result: Vec<i32>,
  card: Vec<i32>,
}

fn parse(input: &str) -> Vec<Lotto> {
  input
    .lines()
    .map(|line| {
      let mut game = line.split(": ");
      let id = game
        .next()
        .expect("Must game")
        .split(" ")
        .filter(|c| c != &"")
        .skip(1)
        .next()
        .expect("Must be game num")
        .parse()
        .expect("NAN");
      let mut game = game.next().expect("must have game").split("|");
      let result = game
        .next()
        .expect("must have results")
        .trim()
        .split(" ")
        .filter(|c| c != &"")
        .map(|n| n.parse().expect("NAN"))
        .collect();
      let card = game
        .next()
        .expect("must have results")
        .trim()
        .split(" ")
        .filter(|c| c != &"")
        .map(|n| n.parse().expect("NAN"))
        .collect();

      Lotto { id, result, card }
    })
    .collect()
}

fn calculate_1(tickets: &Vec<Lotto>) -> i32 {
  tickets
    .iter()
    .map(
      |Lotto {
         id: _id,
         result,
         card,
       }| {
        let power = card.iter().filter(|n| result.contains(n)).count() as i32 - 1;
        if power < 0 {
          return 0;
        }
        2_i32.pow(power.try_into().unwrap())
      },
    )
    .sum()
}

fn calculate_2(tickets: &Vec<Lotto>) -> i32 {
  let mut counter = 0;
  let mut nextextra: HashMap<i32, i32> = HashMap::new();
  tickets.iter().for_each(|ticket| {
    let wins = ticket
      .card
      .iter()
      .filter(|n| ticket.result.contains(n))
      .count() as i32;
    let mult = nextextra.get(&ticket.id).unwrap_or(&0) + 1;
    counter += mult;
    for win in 1..=wins {
      let k = ticket.id + win;
      let curr = nextextra.get(&k).unwrap_or(&0);
      nextextra.insert(k, curr + mult);
    }
  });

  counter
}

#[cfg(test)]
mod test {
  const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
  use super::*;
  #[test]
  fn test1() {
    let tst = parse(INPUT);
    assert_eq!(calculate_1(&tst), 13);
  }
  #[test]
  fn test2() {
    let tst = parse(INPUT);
    assert_eq!(calculate_2(&tst), 30);
  }
}
