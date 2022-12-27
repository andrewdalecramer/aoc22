
use crate::parse_utils::*;
//use crate::arr2d::Array2d;


fn parse(source: &str) -> Vec<isize> {
    let mut it = get_byte_iterator(source);
    let mut output = Vec::new();
    consume_white_space(&mut it);
    loop {
        match it.peek() {
            Some(_) => {
                output.push(parse_number(source, &mut it));
            },
            None => {
                return output;
            }
        }
        consume_white_space(&mut it);
    }
}


fn mult(input: &Vec<isize>) -> Vec<isize> {
    let mut output = Vec::with_capacity(input.len());

    for v in input {
        output.push(*v * 811589153);
    }
    output
}

// Take the number at index within output, and switch it to the left
// Returns the new position
// If the resultant position is at index 0, instead it is placed at end
fn switch_down<T: Copy>(index: usize, output: &mut Vec<T>) -> usize {
    if index == 1 {
        let val = output.remove(index);
        output.push(val);
        return output.len() - 1;
    }

    if index == 0 {
        let val = output.remove(index);
        let val2 = output.pop().expect("Unexpected small input");
        output.push(val);
        output.push(val2);
        return output.len() - 2;
    }
    
    let val = output[index];
    let val2 = output[index-1];
    output[index] = val2;
    output[index-1] = val;
    return index-1;
}

// Take the number at index within output, and switch it to the right 
// Returns the new position
fn switch_up<T: Copy>(index: usize, output: &mut Vec<T>) -> usize {
    if index == output.len()-1 {
        let val = output.pop().unwrap();
        output.insert(1, val);
        return 1;
    }

    // What if we land at end, does that wrap? Stupid rules don't say

    let val = output[index];
    let val2 = output[index+1];
    output[index] = val2;
    output[index+1] = val;
    return index+1;
}

fn find<T: PartialEq>(val: T, vec: &Vec<T>) -> usize {
    for i in 0..vec.len() {
        if vec[i] == val {
            return i;
        }
    }
    panic!("Didn't find value");
}

#[allow(dead_code)]
fn mix_old(input: &Vec<isize>) -> Vec<isize> {
    let mut output = input.clone();
    let mut order = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        order.push(i);
    }

    for i in 0..input.len() {
        let mut index = find(i, &order);
        let mut val = output[index];
        let ilen = input.len() as isize - 1;
        let n = val / ilen;
        val -= n*ilen;

        if val < 0 {
            for _ in 0..(-val) {
                switch_down(index, &mut order);
                index = switch_down(index, &mut output);
            }
        } else if val > 0 {
            for _ in 0..val {
                switch_up(index, &mut order);
                index = switch_up(index, &mut output);
            }
        }
    }
    output
}

fn modulo(a: isize, m: isize) -> isize {
    let n = a / m;
    if a < 0 {
        a - n*m + m
    } else {
        a - n*m
    }
}

fn mix(input: &Vec<isize>, n: usize) -> Vec<isize> {
    let mut output = input.clone();
    let mut order = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        order.push(i);
    }

    for _ in 0..n {
        for i in 0..input.len() {
            let index = find(i, &order);

            order.remove(index);
            let val = output.remove(index);


            // Index is one to the right of the old position
            let new_pos = modulo(index as isize + val, output.len() as isize) as usize;
            order.insert(new_pos, i);
            output.insert(new_pos, val);
        }
    }
    output
}

fn solve(input: &str) -> isize {
    let file = parse(input);
    let new = mix(&file, 1);
    let zero_index = find(0, &new);
    let a = new[(zero_index+1000) % new.len()];
    let b = new[(zero_index+2000) % new.len()];
    let c = new[(zero_index+3000) % new.len()];
    println!("{} {} {}", a,b,c);
    a+b+c
}


fn solve2(input: &str) -> isize {
    let file = mix(&mult(&parse(input)), 10);

    let zero_index = find(0, &file);
    let a = file[(zero_index+1000) % file.len()];
    let b = file[(zero_index+2000) % file.len()];
    let c = file[(zero_index+3000) % file.len()];
    println!("{} {} {}", a,b,c);
    a+b+c
}


pub fn run() {
    let input =
        std::fs::read_to_string("data/d20.txt")
        .expect("Failed to read input");
    println!("{}", solve(&input));
    println!("{}", solve2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    const EXAMPLE: &'static str = "1
2
-3
3
-2
0
4";
    
    #[test]
    fn test_example() {
        assert_eq!(solve(EXAMPLE), 3);
    }
    
    #[test]
    fn test_example2() {
        assert_eq!(
            mult(&parse(EXAMPLE)),
            vec![811589153, 1623178306, -2434767459, 2434767459, -1623178306, 0, 3246356612]);
        assert_eq!(solve2(EXAMPLE), 1623178306);
    }
}
