// https://adventofcode.com/2022/day/1

pub fn solve(input: String) -> (String, String) {
    let elves: Vec<&str>= input.split("\n\n").collect();

    // Create Vector of Vector for a calories list for each elf
    let elves_list: Vec<Vec<u32>> = elves
        .iter()
        .map(|elf| elf.split("\n").map(|calorie| calorie.parse::<u32>().unwrap()).collect())
        .collect();

    // Sum all the values for each elf
    let mut elves_calories:Vec<u32> = elves_list
        .iter()
        .map(|list| list.iter().sum())
        .collect();

    // Sort descending 
    elves_calories.sort_by(|a, b| b.cmp(a));

    let top3_calories = &elves_calories[..3].iter().sum::<u32>();
    let max_calories = elves_calories.first().unwrap();

    // Return the solution to part1 and part2 in a tuple.
    (max_calories.to_string(), top3_calories.to_string())
}


#[test]
fn verify_test() {
    let base = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    let result = solve(base.to_string());

    assert_eq!("24000", result.0);
    assert_eq!("45000", result.1);
}