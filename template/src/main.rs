use std::fs;

fn main() {
    const FILE_PATH: &str = "input.txt";
    println!("Welcome to AOC2023 day NUMBER, first we will read the file {}", FILE_PATH);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    println!("And we parse the file!");
    //let mut data = parse(&contents);
    //println!("First thing: {}", calculate_1(&data));
    //println!("Second thing: {}", calculate_2(&data));

}


#[cfg(test)]
mod test {
    const INPUT: &str = r#""#;
    use super::*;
/*     #[test]
    fn test1() {
        let tst = parse(INPUT);
        assert_eq!(calculate_1(&tst), 24000);
    } */
  /*   #[test]
    fn testtemplate2() {
        let tst = parse(INPUT);
        assert_eq!(calculate_2(&tst), 24000);
    } */
}