



const N_TYPES: usize = 52;

#[derive(Clone,Copy,Debug)]
struct Rucksack {
    comps: [[usize; N_TYPES]; 2],
}

fn empty_rucksack() -> Rucksack {
    Rucksack { comps: [[0; N_TYPES]; 2] }
}


fn read_rucksack(line: &str) -> Rucksack  {
    let mut output = empty_rucksack();

    if line.len() % 2 != 0 {
        panic!("Odd number of characters on a line");
    }
//       abcdefghijklmnopqrstuvwxyz
    let comp_size = line.len() / 2;
    let mut i = 0;
    for c in line.bytes() {
        let index = 
            if c < b'A' {
                panic!("Unexpected character {}", c);
            } else if c <= b'Z' {
                c - b'A' + (b'z'-b'a') + 1
            } else if c < b'a' {
                panic!("Unexpected character {}", c);
            } else if c <= b'z' {
                c - b'a'
            } else {
                panic!("Unexpected character {}", c);
            } as usize;

        let cindex = if i < comp_size { 0 } else { 1 };

        output.comps[cindex][index] += 1;
        i += 1;
    }
    output
}


fn find_common_val(r: &Rucksack) -> usize {
    let mut item_num = usize::MAX;
    for i in 0..N_TYPES {
        if r.comps[0][i] > 0 && r.comps[1][i] > 0 {
            if item_num != usize::MAX {
                panic!("Expected only one type to be common");
            }
            item_num = i;
        }
    }
    item_num
}

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

fn priority_from_val(i: usize) -> usize { i + 1 }

fn solvep2(input: &str) -> usize {
    let mut result = 0;

    let mut it = input.split("\n");
    loop {
        let mut rucksacks = [empty_rucksack(); 3];
        for i in 0..3 {
            match get_next_non_empty_line(&mut it) {
                Some(s) => { rucksacks[i] = read_rucksack(s); }
                None => {
                    if i == 0 {
                        return result;
                    } else {
                        panic!("Unexpected parity of rucksacks (ended on {})", i);
                    }
                }
            }
        }

        let mut item_num = usize::MAX;
        for i in 0..N_TYPES {
            if
                (rucksacks[0].comps[0][i]>0 || rucksacks[0].comps[1][i]>0) &&
                (rucksacks[1].comps[0][i]>0 || rucksacks[1].comps[1][i]>0) &&
                (rucksacks[2].comps[0][i]>0 || rucksacks[2].comps[1][i]>0)
            {
                if item_num != usize::MAX {
                    panic!("Expected only one type to be common");
                }
                item_num = i;
            }
        }
        if item_num == usize::MAX {
            panic!("Failed to find common item\n{:?}", rucksacks);
        }
        let p = priority_from_val(item_num);
        result += p;
    }
}

fn solvep1(input: &str) -> usize {
    let mut result = 0;
    for line in input.split("\n") {
        if line == "" { continue; }
        let r = read_rucksack(line);
        let c = find_common_val(&r);
        let p = priority_from_val(c);
        result += p;
    }
    result
}


pub fn run() {
    let input = std::fs::read_to_string("data/d3.txt").expect("Failed to read input");
    println!("Sum of priorities is {}", solvep1(&input));
    println!("Sum of priorities for the badges is {}", solvep2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_read() {
        let r = read_rucksack("pq");
        println!("{:?}", r.comps[0]);
        assert_eq!(r.comps[0][15], 1);
        assert_eq!(r.comps[1][16], 1);
    }

    #[test]
    fn test_example_p1() {
        let example_input =
"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(solvep1(example_input), 157);
    }
    
    #[test]
    fn test_example_p2() {
        let example_input =
"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(solvep2(example_input), 70);
    }
}
