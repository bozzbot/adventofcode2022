// https://adventofcode.com/2022/day/5

const SPACING: usize = 4;

pub fn solve(_input: String) -> (String, String) {
    let (initial_stack, commands) = _input.split_once("\n\n").unwrap();
    // println!("{:?}", initial_stack);

    let line = initial_stack.lines().rev().take(1).collect::<String>();
    let number_stacks = 1 + ((line.len() - 2) / SPACING);
    let stacks_height: usize = initial_stack.matches("[").count();
    // println!("{:?}", line);
    // println!("{:?}", initial_stack);
    // println!("{}", number_stacks);
    // println!("{}", stacks_height);

    let mut stacks = vec![vec![' '; stacks_height]; number_stacks];

    // Build initial stack by iterating over the lines and assign the characters to the stacks
    initial_stack
        .lines().rev().skip(1).enumerate()
        .for_each(|(index, line)| {
        line.char_indices().skip(1)
            .step_by(SPACING).for_each(|(i, c)| stacks[i/SPACING][index] = c);
    });

    // Remove all "empty" slots
    for stack in stacks.iter_mut() {
        stack.retain(|c| c.is_alphabetic());
    }

    let mut stacks2 = stacks.clone();

    // println!("{:?}", stacks);

    commands.lines().for_each(|command| {
        // Vector with [amout, stack from, stack to]
        let numbers: Vec<usize> = command.split_ascii_whitespace().filter_map(|number| number.parse::<usize>().ok()).collect();
        // println!("{:?}", numbers);
        // println!("{:?}", stacks);
        let current_length = stacks[numbers[1] - 1].len();
        let mut tmp = stacks[numbers[1] - 1].split_off(current_length - numbers[0]);
        tmp.reverse();
        // println!("{:?}", tmp);
        stacks[numbers[2] - 1].append(&mut tmp);
    });

    // println!("{:?}", stacks);

    let solution1 = stacks.iter().filter_map(|stack| stack.last()).collect::<String>();

    // TODO: Improve this lazy copy and paste by moving to a fn
    commands.lines().for_each(|command| {
        // Vector with [amout, stack from, stack to]
        let numbers: Vec<usize> = command.split_ascii_whitespace().filter_map(|number| number.parse::<usize>().ok()).collect();
        // println!("{:?}", numbers);
        // println!("{:?}", stacks2);
        let current_length = stacks2[numbers[1] - 1].len();
        let mut tmp = stacks2[numbers[1] - 1].split_off(current_length - numbers[0]);
        // tmp.reverse();
        // println!("{:?}", tmp);
        stacks2[numbers[2] - 1].append(&mut tmp);
    });

    let solution2 = stacks2.iter().filter_map(|stack| stack.last()).collect::<String>();

    (solution1, solution2)
}

#[test]
fn verify_solve() {
    let base = 
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    let result = solve(base.to_string());

    assert_eq!("CMZ", result.0);
    assert_eq!("MCD", result.1);
}