
fn parse_range(string: &str) -> (usize, usize) {
    let mut r_it = string.split("-");
    let s =
        r_it.next()
        .expect("Missing first range value")
        .parse()
        .expect("Failed to parse first range_value");
    let e =
        r_it.next()
        .expect("Missing second range value")
        .parse()
        .expect("Failed to parse second range_value");
    (s,e)
}

fn parse_line(s: &str) -> (usize, usize, usize, usize) {
    let mut ab = s.split(",");
    let (a_s, a_e) = parse_range(ab.next().expect("Missing first range"));
    let (b_s, b_e) = parse_range(ab.next().expect("Missing second range"));
    (a_s, a_e, b_s, b_e)
}


fn solvep1(input: &str) -> usize {
    let mut result = 0;
    for line in input.split("\n") {
        if line == "" { continue; }
        let (a_s, a_e, b_s, b_e) = parse_line(line);
        if (a_s >= b_s && a_e <= b_e) || (a_s <= b_s && a_e >= b_e) {
            result += 1;
        }
    }
    result
}

fn solvep2(input: &str) -> usize {
    let mut result = 0;
    for line in input.split("\n") {
        if line == "" { continue; }
        let (a_s, a_e, b_s, b_e) = parse_line(line);
        // Ranges overlap if both starts are before both ends
        if a_s <= b_e && b_s <= a_e {
            result += 1;
        }
    }
    result
}

pub fn run() {
    let input = std::fs::read_to_string("data/d4.txt").expect("Failed to read input");
    println!("part1: {}", solvep1(&input));
    println!("part2: {}", solvep2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example_p1() {
        let example_input =
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(solvep1(example_input), 2);
        assert_eq!(solvep2(example_input), 4);
    }
    
    #[test]
    fn test_example_p2() {
    }
}
