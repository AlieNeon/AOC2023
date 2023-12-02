use std::fs;

fn main() {
  const FILE_PATH: &str = "input.txt";
  println!(
    "Welcome to AOC2023 day 2, first we will read the file {}",
    FILE_PATH
  );
  let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

  println!("And we parse the file!");
  let data = parse(&contents);
  println!("First thing: {}", calculate_1(&data));
  println!("Second thing: {}", calculate_2(&data));
}

#[derive(Debug)]
struct Game {
  id: i32,
  rounds: Vec<(i32, i32, i32)>,
}

fn parse(input: &str) -> Vec<Game> {
  input
    .lines()
    .map(|line| {
      let mut game = line.split(":");
      let id: i32 = game
        .next()
        .expect("Must have a Game ID")
        .split(" ")
        .skip(1)
        .next()
        .expect("Must have an ID")
        .parse()
        .expect("NAN");

      let game: String = game.collect();
      let rounds = game
        .split(";")
        .map(|round| {
          let roundstr: Vec<_> = round.split(",").collect();
          let mut round = (0, 0, 0);
          for colour in roundstr {
            let colour: Vec<_> = colour.trim().split(" ").collect();
            match colour[1] {
              "red" => {
                round.0 = round.0 + colour[0].parse::<i32>().expect("NAN");
              }
              "green" => {
                round.1 = round.1 + colour[0].parse::<i32>().expect("NAN");
              }
              "blue" => {
                round.2 = round.2 + colour[0].parse::<i32>().expect("NAN");
              }
              err => {
                unreachable!("{}", err)
              }
            }
          }
          round
        })
        .collect();
      Game { id, rounds }
    })
    .collect()
}

const CONDITION: (i32, i32, i32) = (12, 13, 14);
fn calculate_1(games: &Vec<Game>) -> i32 {
  games
    .iter()
    .filter(|game| {
      game
        .rounds
        .iter()
        .all(|(r, g, b)| r <= &CONDITION.0 && g <= &CONDITION.1 && b <= &CONDITION.2)
    })
    .fold(0, |acc, game| acc + game.id)
}

fn calculate_2(games: &Vec<Game>) -> i32 {
  games
    .iter()
    .map(|game| {
      let mut max_cubes = (0, 0, 0);
      for round in &game.rounds {
        if round.0 > max_cubes.0 {
          max_cubes.0 = round.0
        }
        if round.1 > max_cubes.1 {
          max_cubes.1 = round.1
        }
        if round.2 > max_cubes.2 {
          max_cubes.2 = round.2
        }
      }
      max_cubes.0 * max_cubes.1 * max_cubes.2
    })
    .sum()
}

#[cfg(test)]
mod test {
  const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
  use super::*;
  #[test]
  fn test1() {
    let tst = dbg!(parse(INPUT));
    assert_eq!(calculate_1(&tst), 8);
  }
  #[test]
  fn test2() {
    let tst = parse(INPUT);
    assert_eq!(calculate_2(&tst), 2286);
  }
}
