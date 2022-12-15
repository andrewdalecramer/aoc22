
type Item = u64;

struct Monkey {
    inspection_count: usize,
    items: Vec<Item>,
    operator: Operator,
    left_operand: Operand,
    right_operand: Operand,
    test_divisor: Item,
    throw_if_true: usize,
    throw_if_false: usize,
}

macro_rules! consume {
    ($it:ident, $token:pat_param, $msg: expr) => ({
        match $it.next() {
            Some($token) => { },
            Some(tok) => {
                panic!("Expected {} but received {}", $msg, tok);
            },
            None => {
                panic!("Expected {} but line ended unexpectedly", $msg);
            }
        }
    });
}

fn assert_eol(token: Option<&str>) {
    match token {
        Some(tok) => {
            panic!("Expected end of line, not '{}'", tok);
        },
        None => { },
    }
}

enum Operator {
    Mult,
    Plus,
}
use Operator::*;

#[derive(Clone,Copy)]
enum Operand {
    Old,
    Val(Item),
}
use Operand::*;

fn parse_operand(token: &str) -> Operand {
    if token == "old" {
        Old
    } else {
        Val(token.parse().expect("Failed to parse operand"))
    }
}

fn parse_operator(token: &str) -> Operator {
    match token {
        "+" => Plus,
        "*" => Mult,
        _ => {
            panic!("Unexpected operator {}", token);
        },
    }
}

fn get_number_at_end
    <T: std::str::FromStr + std::fmt::Debug>
    (prefix: &str, line: &str)
    -> T
    where <T as std::str::FromStr>::Err: std::fmt::Debug
{
    if &line[0..prefix.len()] != prefix {
        panic!("Expected prefix of '{:?}', but got '{:?}'", prefix, line);
    }
    line[prefix.len()+1..line.len()]
        .parse()
        .expect("Failed to parse number")
}


/// Determine the cycle number inside the current set
fn parse(source: &str) -> Vec<Monkey> {
    let mut src_it = source.split("\n").peekable();
    let mut monkey_num = 0;
    let mut monkeys = Vec::new();
    loop {
        match src_it.next() {
            Some(name_line) => {
                if name_line != format!("Monkey {}:", monkey_num) {
                    panic!("Unexpected form for monkey declaration");
                }
            },
            None => { panic!("Expected monkey declaration"); }
        }
        monkey_num += 1;

        let mut line_it = src_it.next().expect("Expected Starting items").split(" ");
        consume!(line_it, "", "<empty string>");
        consume!(line_it, "", "<empty string>");
        consume!(line_it, "Starting", "\"Starting\"");
        consume!(line_it, "items:", "\"items:\"");
        let mut monkey_items = Vec::new();
        loop {
            match line_it.next() {
                Some(val) => {
                    monkey_items.push(
                        if &val[val.len()-1..val.len()] == "," {
                            val[0..val.len()-1]
                                .parse()
                                .expect("Expected number for worry level")
                        } else {
                            val
                                .parse()
                                .expect("Expected number for worry level")
                        });
                },
                None => { break; }
            }
        }
        assert_eol(line_it.next());

        let mut line_it = src_it.next().expect("Expected operation items").split(" ");
        consume!(line_it, "", "<empty string>");
        consume!(line_it, "", "<empty string>");
        consume!(line_it, "Operation:", "\"Operation:\"");
        consume!(line_it, "new", "\"new\"");
        consume!(line_it, "=", "\"=\"");
        let left = parse_operand(line_it.next().expect("Expected left operand"));
        let op = parse_operator(line_it.next().expect("Expected operator"));
        let right = parse_operand(line_it.next().expect("Expected right operand"));
        assert_eol(line_it.next());

        let divisor = get_number_at_end(
            "  Test: divisible by",
            src_it.next().expect("Expected test line"));
        
        let if_true = get_number_at_end(
            "    If true: throw to monkey",
            src_it.next().expect("Expected throw line (true)"));
        
        let if_false = get_number_at_end(
            "    If false: throw to monkey",
            src_it.next().expect("Expected throw line (false)"));

        monkeys.push(
            Monkey {
                inspection_count: 0,
                items: monkey_items, 
                operator: op,
                left_operand: left,
                right_operand: right,
                test_divisor: divisor,
                throw_if_true: if_true,
                throw_if_false: if_false,
            });

        // Consume blank lines
        loop {
            match src_it.peek() {
                Some(&"") => { src_it.next(); },
                Some(_) => { break; }
                None => { return monkeys; }
            }
        }
    }

}

fn add_lcm_factor(lcm: Item, factor: Item) -> Item {
    if lcm % factor == 0 {
        lcm
    } else {
        lcm * factor
    }
}

fn get_monkeys_least_common_multiple(monkeys: &Vec<Monkey>) -> Item {
    let mut lcm = 1;
    for monkey in monkeys {
        lcm = add_lcm_factor(lcm, monkey.test_divisor);
        match monkey.left_operand {
            Old => { },
            Val(v) => { lcm = add_lcm_factor(lcm, v); }
        }
        match monkey.right_operand {
            Old => { },
            Val(v) => { lcm = add_lcm_factor(lcm, v); }
        }
    }
    lcm
}

fn evaluate_operand(operand: Operand, old_val: Item) -> Item {
    match operand {
        Old => old_val,
        Val(v) => v
    }
}

fn execute_round(monkeys: &mut Vec<Monkey>, relief: Item, lcm: Item) {
    for i in 0..monkeys.len() {
        monkeys[i].inspection_count += monkeys[i].items.len();
        while monkeys[i].items.len() > 0 {
            let old_val = monkeys[i].items.remove(0);
            let (new_val, target_monkey) = {
                // Inspect items
                let monkey = &monkeys[i];
                let left = evaluate_operand(monkey.left_operand, old_val);
                let right = evaluate_operand(monkey.right_operand, old_val);
                let new_val = match monkey.operator {
                    Mult => left * right,
                    Plus => left + right,
                };
                // Relief
                let new_val = (new_val / relief) % lcm;

                // Test
                let target_monkey = 
                    if new_val % monkey.test_divisor == 0 {
                        monkey.throw_if_true
                    } else {
                        monkey.throw_if_false
                    };
                (new_val, target_monkey)
            };
            monkeys[target_monkey].items.push(new_val);
        }
    }
}

fn solve(input: &str, relief: Item, rounds: usize) -> usize {
    let mut monkeys = parse(input);
    let lcm = get_monkeys_least_common_multiple(&monkeys);
    for _ in 0..rounds {
        execute_round(&mut monkeys, relief, lcm);
    }
    let mut best = [0; 2];
    for monkey in monkeys {
        let mut mic = monkey.inspection_count;
        for i in 0..2 {
            if mic > best[i] {
                let tmp = best[i];
                best[i] = mic;
                mic = tmp;
            }
        }
    }
    let mut output = 1;
    for v in best {
        output *= v;
    }
    output
}


pub fn run() {
    let input =
        std::fs::read_to_string("data/d11.txt")
        .expect("Failed to read input");
    println!("{}", solve(&input, 3, 20));
    println!("{}", solve(&input, 1, 10000));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    const EXAMPLE: &'static str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    #[test]
    fn test_example() {
        assert_eq!(solve(EXAMPLE, 3, 20), 10605);
        assert_eq!(solve(EXAMPLE, 1, 10000), 2713310158);
    }
}
