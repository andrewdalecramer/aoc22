
fn get_next_non_empty_line<'a>(it: &'a mut std::str::Split<&str>) -> Option<&'a str> {
    loop {
        match it.next() {
            Some(s) => {
                if s != "" {
                    return Some(s);
                } else {
                    continue;
                }
            },
            None => { return None; }
        }
    }
}

type Stacks = Vec<Vec<char>>;
struct Order {
    from: usize,
    to: usize,
    amount: usize,
}

fn count_box_stacks(line: &str) -> usize {
    let mut last_label = 0;
    for label in line.split(" ") {
        if label == "" { continue; }
        let val = label.parse().expect(&format!("Failed to parse label '{}'", label));
        if val != last_label + 1 {
            panic!("Labels were not sequential");
        }
        last_label = val;
    }
    last_label
}

fn parse_crate(s: &str) -> Option<char> {
    let mut iter = s.chars();
    match iter.next() {
        Some(' ') => {
            assert_eq!(iter.next().expect("Unexpect end of crate input"), ' ');
            assert_eq!(iter.next().expect("Unexpect end of crate input"), ' ');
            return None;
        },
        Some('[') => {
            let val = iter.next();
            assert_eq!(iter.next().expect("Unexpect end of crate input"), ']');
            return val;
        },
        Some(_) => {
            panic!("Unexpected character parsing crate");
        }
        None => {
            panic!("Empty string given to parse_crate");
        }
    }
}
    
fn parse_crate_stacks(lines: Vec<String>, stack_count: usize) -> Stacks {
    let mut stacks: Stacks = vec![Vec::new(); stack_count];
    for line in lines.iter().rev() {
        let n_cols = (line.len() + 1)/4;
        assert_eq!(n_cols*4-1, line.len(), "Unexpected line length parsing stacks");
        assert!(n_cols <= stack_count, "Unexpectedly long line length parsing stacks");
        for i in 0..n_cols {
            match parse_crate(&line[(i*4)..(i*4+3)]) {
                Some(krate) => { stacks[i].push(krate); },
                None => { },
            }
        }
    }
    stacks
}

fn parse_start(it: &mut std::str::Split<&str>) -> Stacks {
    let mut lines: Vec<String> = Vec::new();

    let stack_count = loop {
        match get_next_non_empty_line(it) {
            Some(line) => {
                if &line[0..4] == " 1  " {
                    break count_box_stacks(line);
                }
                lines.push(line.to_string());
            },
            None => {
                panic!("Unexpected end of input, didn't find stack label line");
            }
        }
    };

    parse_crate_stacks(lines, stack_count)
}

fn print_stacks(stacks: &Stacks) {
    let biggest_stack = stacks.iter().fold(0, |acc, x| std::cmp::max(x.len(), acc));

    for height in (0..biggest_stack).rev() {
        for stack in stacks {
            match stack.get(height) {
                Some(val) => { print!("[{}] ", val); },
                None => { print!("    "); },
            }
        }
        print!("\n");
    }

    for i in 0..stacks.len() {
        print!(" {:<3}", i+1);
    }
    print!("\n");
}

fn parse_order(line: &str) -> Option<Order> {
    if line == "" {
        None
    } else {
        let mut it = line.split(" ");
        assert_eq!(it.next().expect("Unexpected end of input reading order"), "move");
        let amt = 
            it.next()
            .expect("Unexpected end of input reading order")
            .parse()
            .expect("Failed to parse amount");
        assert_eq!(it.next().expect("Unexpected end of input reading order"), "from");
        let from: usize = 
            it.next()
            .expect("Unexpected end of input reading order")
            .parse()
            .expect("Failed to parse from");
        assert_eq!(it.next().expect("Unexpected end of input reading order"), "to");
        let to: usize = 
            it.next()
            .expect("Unexpected end of input reading order")
            .parse()
            .expect("Failed to parse to");
        Some(Order { 
            from: from - 1,
            to: to - 1,
            amount: amt,
        })
    }
}


fn apply_order(order: Order, stacks: &mut Stacks) {
    for _ in 0..order.amount {
        match stacks[order.from].pop() {
            Some(krate) => {
                stacks[order.to].push(krate);
            },
            None => {
                println!(
                    "Asked to move crate from {} to {} but stack is empty:",
                    order.from + 1,
                    order.to + 1);
                print_stacks(stacks);
                panic!("Stopping");
            }
        }
    }
}

fn solvep1(input: &str) -> String {
    let mut input_it = input.split("\n");
    let mut stacks = parse_start(&mut input_it);

    println!("----START----");
    print_stacks(&stacks);

    for order in input_it.map(|x| parse_order(x)) {
        match order {
            Some(order) => {
                apply_order(order, &mut stacks);
            },
            None => {},
        }
    }
    println!("---- END ----");
    print_stacks(&stacks);
    println!("----     ----");
    let mut output = "".to_string();
    for stack in stacks {
        output.push(stack[stack.len()-1]);
    }
    output
}


fn apply_order_p2(order: Order, stacks: &mut Stacks) {
    let from_size = stacks[order.from].len();
    for i in 0..order.amount {
        let from_index = from_size - order.amount + i;
        let krate = stacks[order.from][from_index];
        stacks[order.to].push(krate);
    }
    for _ in 0..order.amount {
        stacks[order.from].pop();
    }
}


fn solvep2(input: &str) -> String {
    let mut input_it = input.split("\n");
    let mut stacks = parse_start(&mut input_it);

    println!("----START----");
    print_stacks(&stacks);

    for order in input_it.map(|x| parse_order(x)) {
        match order {
            Some(order) => {
                apply_order_p2(order, &mut stacks);
            },
            None => {},
        }
    }
    println!("---- END ----");
    print_stacks(&stacks);
    println!("----     ----");
    let mut output = "".to_string();
    for stack in stacks {
        output.push(stack[stack.len()-1]);
    }
    output
}

pub fn run() {
    let input = std::fs::read_to_string("data/d5.txt").expect("Failed to read input");
    println!("part1: {}", solvep1(&input));
    println!("part2: {}", solvep2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example_p1() {
        let example_input =
            "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(solvep1(example_input), "CMZ");
        assert_eq!(solvep2(example_input), "MCD");
    }
}
