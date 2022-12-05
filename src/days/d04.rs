// https://adventofcode.com/2022/day/4

pub fn solve(_input: String) -> (String, String) {
    let comparisons: Vec<&str>= _input.lines().collect();
    // println!("{:?}", comparisons);

    let overlaps1 = comparisons.iter().map(|comparison| {
        let sections: Vec<Vec<usize>> = comparison.split(",").map(|sections| {
            let section = sections.split("-").map(|v| v.parse::<usize>().unwrap()).collect();
            section
        }).collect();
        // println!("{:?}", sections);

        if (sections[0][0] >= sections[1][0] && sections[0][1] <= sections[1][1]) 
            || (sections[0][0] <= sections[1][0] && sections[0][1] >= sections[1][1]) {
            return 1;
        }
        return 0;
        
    }).sum::<usize>();
    
    println!("{:?}", overlaps1);

    let overlaps2 = comparisons.iter().map(|comparison| {
        let sections: Vec<Vec<usize>> = comparison.split(",").map(|sections| {
            let section = sections.split("-").map(|v| v.parse::<usize>().unwrap()).collect();
            section
        }).collect();
        // println!("{:?}", sections);

        if (sections[0][0] >= sections[1][0] && sections[0][0] <= sections[1][1]) 
            || (sections[0][1] >= sections[1][0] && sections[0][1] <= sections[1][1])
            || (sections[1][0] >= sections[0][0] && sections[1][0] <= sections[0][1])
            || (sections[1][1] >= sections[0][0] && sections[1][1] <= sections[0][1]) {
            return 1;
        }
        return 0;
        
    }).sum::<usize>();

    println!("{:?}", overlaps2);


    (overlaps1.to_string(), overlaps2.to_string())
}

#[test]
fn verify_solve() {
    let base = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    let result = solve(base.to_string());

    assert_eq!("2", result.0);
    assert_eq!("4", result.1);
}