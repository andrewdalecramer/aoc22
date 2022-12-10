

/// Determine the cycle number inside the current set
fn cycle_mod(cycle: isize) -> isize {
    (cycle + 20) % 40
}

fn solve(input: &str) -> isize {
    let mut output = 0;
    let mut x_val = 1;
    let mut cycle = 0;
    for line in input.split("\n") {
        if line == "" { continue; }

        // Parse and move cycle forward
        let old_cycle = cycle;
        let mut tok_it = line.split(" ");
        let cmd = tok_it.next().expect("Expected at least one token");
        let delta = match cmd {
            "addx" => {
                cycle += 2;
                tok_it.next()
                    .expect("Expected at least one token")
                    .parse()
                    .expect("Failed to parse addx instruction")
            },
            "noop" => {
                cycle += 1;
                0
            },
            token => {
                panic!("Unexpected command {}", token);
            }
        };
        assert!(matches!(tok_it.next(), None), "Extra token found");

        // Check passed point
        //  - when the number rolls around that's a decrease and a set complete
        if cycle_mod(cycle) < cycle_mod(old_cycle) {
            // Round to closest 20 so we get the cycle number when it actually
            // rolled over rather than the current exact cycle
            output += (cycle / 20)*20 * x_val;
        }
        // Change register
        x_val += delta;
    }
    output
}


fn check_draw(x_val: isize, cycle: isize) {
    let cycle_pos = cycle % 40;
    if x_val <= cycle_pos && cycle_pos <= x_val + 2 {
        print!("#");
    } else {
        print!(" ");
    }
    if cycle % 40 == 0 {
        print!("\n");
    }
}

fn solve2(input: &str) {
    let mut x_val = 1;
    let mut cycle = 0;
    for line in input.split("\n") {
        if line == "" { continue; }

        // Parse and move cycle forward
        let mut tok_it = line.split(" ");
        let cmd = tok_it.next().expect("Expected at least one token");
        let delta = match cmd {
            "addx" => {
                cycle += 1;
                check_draw(x_val, cycle);
                cycle += 1;
                check_draw(x_val, cycle);
                tok_it.next()
                    .expect("Expected at least one token")
                    .parse()
                    .expect("Failed to parse addx instruction")
            },
            "noop" => {
                cycle += 1;
                check_draw(x_val, cycle);
                0
            },
            token => {
                panic!("Unexpected command {}", token);
            }
        };
        assert!(matches!(tok_it.next(), None), "Extra token found");

        // Change register
        x_val += delta;
    }
}

pub fn run() {
    let input =
        std::fs::read_to_string("data/d10.txt")
        .expect("Failed to read input");
    println!("{}", solve(&input));
    solve2(&input);
}


#[cfg(test)]
mod tests {
    use super::*;
    
    const EXAMPLE: &'static str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    #[test]
    fn test_example() {
        assert_eq!(solve(EXAMPLE), 13140);
    }
}
