use std::{fs, ops::Range};

fn main() {
  const FILE_PATH: &str = "input.txt";
  println!(
    "Welcome to AOC2023 day 5, first we will read the file {}",
    FILE_PATH
  );
  let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

  println!("And we parse the file!");
  let (seeds, mappings) = parse(&contents);
  println!("First thing: {}", calculate_1(&seeds, &mappings));
  println!("Second thing: {}", calculate_2(&seeds, &mappings));
}

#[derive(Debug)]
struct Mapping {
  source_range: Range<i64>,
  diference: i64,
}

fn parse(input: &str) -> (Vec<i64>, Vec<Vec<Mapping>>) {
  (
    input
      .split("\n\n")
      .nth(0)
      .unwrap()
      .split(": ")
      .nth(1)
      .unwrap()
      .split(" ")
      .filter(|n| n != &"")
      .map(|n| n.parse().expect("NAN"))
      .collect(),
    input
      .split("\n\n")
      .skip(1)
      .map(|chunk| {
        chunk
          .lines()
          .skip(1)
          .map(|range| {
            let range: Vec<_> = range
              .split(" ")
              .filter(|n| n != &"")
              .map(|n| n.parse().expect("NAN"))
              .collect();
            Mapping {
              source_range: range[1]..(range[1] + range[2]),
              diference: range[0] - range[1],
            }
          })
          .collect()
      })
      .collect(),
  )
}

fn do_mapping(seed: i64, mapping: &Vec<Mapping>) -> i64 {
  for map in mapping {
    if map.source_range.contains(&seed) {
      return seed + map.diference;
    }
  }
  seed
}

fn do_mappings(seed: i64, mappings: &Vec<Vec<Mapping>>) -> i64 {
  let mut seed = seed;
  for mapping in mappings {
    seed = do_mapping(seed, mapping);
  }
  seed
}

fn calculate_1(seeds: &Vec<i64>, mappings: &Vec<Vec<Mapping>>) -> i64 {
  seeds
    .iter()
    .map(|seed| do_mappings(*seed, mappings))
    .min()
    .expect("Must have a min")
}

fn calculate_2(seeds: &Vec<i64>, mappings: &Vec<Vec<Mapping>>) -> i64 {
  let seed_ranges = seeds
    .chunks(2)
    .map(|seed_range| seed_range[0]..seed_range[0] + seed_range[1]);
  let mut min: i64 = i64::MAX;
  let mut new;
  let range = seed_ranges.flat_map(|it| it.clone());
  for seed in range {
    new = do_mappings(seed, mappings);
    if min > new {
      min = new;
    }
  }
  min
}

#[cfg(test)]
mod test {
  const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
  use super::*;
  #[test]
  fn test1() {
    let (seeds, mappings) = parse(INPUT);
    assert_eq!(calculate_1(&seeds, &mappings), 35);
  }
  #[test]
  fn test2() {
    let (seeds, mappings) = parse(INPUT);
    assert_eq!(calculate_2(&seeds, &mappings), 46);
  }
}
