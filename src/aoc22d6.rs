

fn any_same(buffer: &Vec<char>) -> Option<usize> {
    for i in (0..(buffer.len()-1)).rev() {
        for j in ((i+1)..buffer.len()).rev() {
            if buffer[i] == buffer[j] {
                return Some(i+1);
            }
        }
    }
    return None
}

fn advance(
        it: &mut std::iter::Enumerate<std::str::Chars>,
        buffer: &mut Vec<char>,
        amount: usize)
    -> usize
{
    for i in 0..(buffer.len()-amount) {
        buffer[i] = buffer[i+amount];
    }

    let mut last_index = 0;
    for i in (buffer.len()-amount)..buffer.len() {
        let (index, a) = it.next().unwrap();
        last_index = index;
        buffer[i] = a;
    }
    return last_index;
}

fn solve(input: &str, length: usize) -> usize {
    let mut it = input.chars().enumerate();
    let mut buffer = Vec::with_capacity(length);
    buffer.push(' ');
    for _ in 0..(length-1) {
        let (_, a) = it.next().unwrap();
        buffer.push(a);
    }
    let mut safe_jump = 1;
    loop {
        let index = advance(&mut it, &mut buffer, safe_jump);
        match any_same(&buffer) {
            Some(val) => { safe_jump = val; },
            None => { return index+1; }
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
