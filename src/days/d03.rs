// https://adventofcode.com/2022/day/3

fn duplicate_char(part1: &str, part2: &str) -> Option<char> {
    part1.chars().enumerate().find_map(|(_, c)| {
        part2.chars()
            .enumerate()
            .find(|(_, other)| c == *other)
            .map(|_| (c))
    })
}

#[test]
fn verify_duplicate_char() {
    let mut result = duplicate_char("ABCabcAyzu", "qqqqAqqqq");
    assert_eq!("A", result.unwrap().to_string());
    result = duplicate_char("aAOIJDsadf", "qqqqqqqaqqq");
    assert_eq!("a", result.unwrap().to_string());
    result = duplicate_char("CrZsJsPPZsGz", "wwsLwLmpwMDw");
    assert_eq!("s", result.unwrap().to_string());
}

fn get_priority(letter: char) -> usize {
    // Calculate points for character
    // a-z: 1 - 26
    // A-Z: 27 - 52
    let priority = match letter.is_uppercase() {
        true => letter as usize - 38,
        false => letter as usize - 96,
    };
    priority
}

#[test]
fn verify_get_priority() {
    let mut result = get_priority('p');
    assert_eq!("16", result.to_string());
    result = get_priority('L');
    assert_eq!("38", result.to_string());
    result = get_priority('P');
    assert_eq!("42", result.to_string());
    result = get_priority('v');
    assert_eq!("22", result.to_string());
}

pub fn solve(_input: String) -> (String, String) {
    let rucksacks: Vec<&str>= _input.split("\n").collect();
    // println!("{:?}", rucksacks);

    let priority1_sum = rucksacks.iter().map(|rucksack_ptr| {
        let rucksack = rucksack_ptr.to_string();
        let half_size = rucksack.len()/2;

        let item = duplicate_char(&rucksack[..half_size],&rucksack[half_size..]).unwrap();
        return get_priority(item);
    }).sum::<usize>();

    let priority2_sum = rucksacks.chunks(3).map(|tripple| {
        for item in tripple[0].chars() {
            if tripple[1].contains(item) && tripple[2].contains(item) {
                return get_priority(item);
            }
        }
        panic!("Couldn't find comon item.");
    }).sum::<usize>();
    
    (priority1_sum.to_string(), priority2_sum.to_string())
}

#[test]
fn verify_solve() {
    let base = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    let result = solve(base.to_string());

    assert_eq!("157", result.0);
    assert_eq!("70", result.1);
}