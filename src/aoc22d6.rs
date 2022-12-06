

fn any_same(buffer: &Vec<char>) -> bool {
    for i in 0..(buffer.len()-1) {
        for j in (i+1)..buffer.len() {
            if buffer[i] == buffer[j] {
                return true;
            }
        }
    }
    return false
}


fn solve(input: &str, length: usize) -> usize {
    let mut it = input.chars().enumerate();
    let mut buffer = Vec::with_capacity(length);
    for _ in 0..(length-1) {
        let (_, a) = it.next().unwrap();
        buffer.push(a);
    }
    buffer.push(' ');
    loop {
        let (index, val) = it.next().unwrap();
        buffer[length-1] = val;
        if !any_same(&buffer) {
            return index+1;
        }
        for i in 0..(length-1) {
            buffer[i] = buffer[i+1];
        }
    }
}

fn solvep1(input: &str) -> usize {
    solve(input, 4)
}

fn solvep2(input: &str) -> usize {
    solve(input, 14)
}

pub fn run() {
    let input = std::fs::read_to_string("data/d6.txt").expect("Failed to read input");
    println!("part1: {}", solvep1(&input));
    println!("part2: {}", solvep2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example_p1() {
        let examples = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)];

        for (s, v) in examples {
            assert_eq!(solvep1(s), v);
        }
    }
    
    #[test]
    fn test_example_p2() {
        let examples = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)];

        for (s, v) in examples {
            assert_eq!(solvep2(s), v);
        }
    }
}
