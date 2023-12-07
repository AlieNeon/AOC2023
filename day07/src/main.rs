use std::{collections::HashMap, fmt::Debug, fs};

fn main() {
  const FILE_PATH: &str = "input.txt";
  println!(
    "Welcome to AOC2023 day 7, first we will read the file {}",
    FILE_PATH
  );
  let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

  println!("And we parse the file!");
  let data = parse(&contents);
  println!("First thing: {}", calculate_1(&data));
  println!("Second thing: {}", calculate_2(&data));
}

const CARD_ORDER: [char; 14] = [
  'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'C',
];
#[derive(PartialEq, Eq, Ord, Clone)]
struct Card {
  card: char,
}
impl PartialOrd for Card {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    let cmp = CARD_ORDER.iter().position(|&r| r == self.card).unwrap() as i8
      - CARD_ORDER.iter().position(|&r| r == other.card).unwrap() as i8;
    if cmp == 0 {
      Some(std::cmp::Ordering::Equal)
    } else if cmp > 0 {
      Some(std::cmp::Ordering::Greater)
    } else {
      Some(std::cmp::Ordering::Less)
    }
  }
}
impl Debug for Card {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&format!("{}", &self.card))
  }
}

fn get_hand_type(hand: &Vec<Card>) -> Vec<u32> {
  let mut counts: HashMap<char, u32> = HashMap::new();
  for Card { card } in hand {
    let count = counts.get(&card).unwrap_or(&0) + 1;
    counts.insert(*card, count);
  }

  let joker = counts.get(&'C').unwrap_or(&0).clone();
  counts.remove(&'C');

  let mut counts: Vec<_> = counts.values().collect();

  counts.sort();

  let counts: Vec<u32> = counts.iter().rev().take(2).map(|n| **n).collect();

  vec![
    counts.get(0).unwrap_or(&0) + joker,
    *counts.get(1).unwrap_or(&0),
  ]
}

#[derive(Eq, Debug, Ord, Clone)]
struct Hand {
  hand: Vec<Card>,
  bid: u32,
}
impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    let self_counts = get_hand_type(&self.hand);
    let other_counts = get_hand_type(&other.hand);

    if self_counts > other_counts {
      return Some(std::cmp::Ordering::Less);
    }
    if self_counts < other_counts {
      return Some(std::cmp::Ordering::Greater);
    }
    self.hand.partial_cmp(&other.hand)
  }
}
impl PartialEq for Hand {
  fn eq(&self, other: &Self) -> bool {
    self.hand == other.hand
  }
}

fn parse(input: &str) -> Vec<Hand> {
  input
    .lines()
    .map(|line| {
      let mut line = line.split_ascii_whitespace();
      Hand {
        hand: line
          .next()
          .unwrap()
          .chars()
          .map(|c| Card { card: c })
          .collect(),
        bid: line.next().unwrap().parse().unwrap(),
      }
    })
    .collect()
}

fn calculate_1(hands: &Vec<Hand>) -> u32 {
  let mut hands: Vec<Hand> = hands.clone();
  hands.sort();
  hands
    .iter()
    .rev()
    .enumerate()
    .map(|(n, card)| card.bid * (n as u32 + 1))
    .sum()
}

fn calculate_2(hands: &Vec<Hand>) -> u32 {
  let mut hands: Vec<Hand> = hands
    .clone()
    .iter()
    .map(|hand| Hand {
      hand: hand
        .hand
        .iter()
        .map(|c| -> Card {
          if c.card == 'J' {
            Card { card: 'C' }
          } else {
            c.clone()
          }
        })
        .collect(),
      bid: hand.bid,
    })
    .collect();
  hands.sort();

  hands
    .iter()
    .rev()
    .enumerate()
    .map(|(n, card)| card.bid * (n as u32 + 1))
    .sum()
}

#[cfg(test)]
mod test {
  const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
  use super::*;
  #[test]
  fn test1() {
    let tst = parse(INPUT);
    assert_eq!(calculate_1(&tst), 6440);
  }
  #[test]
  fn test2() {
    let tst = parse(INPUT);
    assert_eq!(calculate_2(&tst), 5905);
  }
}
