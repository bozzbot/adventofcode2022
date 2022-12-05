// https://adventofcode.com/2022/day/2

///
/// A for Rock X     => 1
/// B for Paper Y    => 2
/// C for Scissors Z => 3
/// 
pub fn solve(_input: String) -> (String, String) {
    
    let rounds: Vec<&str>= _input.split("\n").collect();
    // println!("{:?}", rounds);

    let mut points1: u32 = 0;
    let mut points2: u32 = 0;

    for line in rounds {
        let points1_round: u32 = match line {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => 0,
        };
        points1 = points1 + points1_round;

        let points2_round: u32 = match line {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => 0,
        };
        points2 = points2 + points2_round;
    }
    

    (points1.to_string(), points2.to_string())
}

#[test]
fn verify_solve() {
    let base = "A Y
B X
C Z";

    let result = solve(base.to_string());

    assert_eq!("15", result.0);
    assert_eq!("12", result.1);
}