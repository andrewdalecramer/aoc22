

use crate::arr2d::Array2d;

use std::cmp::max;

type Coord = (isize, isize);

type It<'a> = std::iter::Peekable<std::iter::Enumerate<std::str::Bytes<'a>>>;



fn is_digit(byte: u8) -> bool {
    b'0' <= byte && byte <= b'9'
}


fn parse_number(src: &str, it: &mut It) -> isize {
    let start = match it.peek() {
        Some((index, val)) => {
            if is_digit(*val) {
                *index
            } else {
                panic!("parse_number couldn't find the number");
            }
        },
        None => {
            panic!("parse_number couldn't find the number");
        },
    };
    it.next();

    let end = loop {
        match it.peek() {
            Some((index, val)) => {
                if !is_digit(*val) {
                    break *index;
                }
                it.next();
            },
            None => {
                break usize::MAX;
            }
        }
    };
    
    let substr = 
        if end == usize::MAX {
            &src[start..]
        } else {
            &src[start..end]
        };

    substr.parse().expect("Failed to parse number")
}


/// Consumes sequence from iterator it
///
/// If sequence is consumed, return true
/// If first byte doesn't match, return false
/// If other byte doesn't match, panic
fn consume_sequence(it: &mut It, sequence: &str) -> bool {
    let mut seq_it = sequence.bytes();
    let seq_byte = seq_it.next().expect("sequence must be at least one character long");
    match it.peek() {
        Some((_, byte)) => {
            if *byte != seq_byte {
                return false;
            }
            it.next();
        },
        None => {
            return false;
        }
    };

    loop {
        let seq_byte = match seq_it.next() {
            Some(v) => v,
            None => { return true; },
        };
        let it_byte = match it.next() {
            Some((_, byte)) => byte,
            None => {
                panic!("Unexpected end of input consuming sequence \"{}\"", sequence);
            }
        };
        if seq_byte != it_byte {
            panic!("Unexpected byte {} consuming sequence \"{}\"", it_byte, sequence);
        }
    }
}

fn parse_coord(src: &str, it: &mut It) -> Coord {
    let i = parse_number(src, it);
    if !consume_sequence(it, ",") {
        panic!("Missing ',' in coordinate");
    }
    let j = parse_number(src, it);
    (i,j)
}

fn parse_line(line: &str) -> Vec<Coord> {
    let mut output = Vec::new();
    let mut it = line.bytes().enumerate().peekable();
    output.push(parse_coord(&line, &mut it));

    loop {
        if !consume_sequence(&mut it, " -> ") {
            return output;
        }
        output.push(parse_coord(&line, &mut it));
    }
}

fn parse(source: &str) -> Vec<Vec<Coord>> {
    let mut output = Vec::new();
    for line in source.split("\n") {
        let line = line.trim();
        if line == "" { continue; }
        output.push(parse_line(line));
    }
    output
}


fn max_coord((x0, y0): Coord, (x1, y1): &Coord) -> Coord {
    (max(x0, *x1), max(y0, *y1))
}


fn get_direction(current: isize, target: isize) -> isize {
    if current < target { 1 }
    else if current > target { -1 }
    else { 0 }
}

/// Takes a world and fills in along a line
///
/// A line is a series of coordinates
fn apply_line(line: &Vec<Coord>, world: &mut Array2d<bool>) {
    if line.len() == 0 {
        panic!("That ain't a line...");
    }
    let mut it = line.iter();
    let (mut i, mut j) = it.next().unwrap();
    *world.get_mut((i,j)) = true;
    loop {
        match it.next() {
            Some((ti, tj)) => {
                let di = get_direction(i, *ti);
                let dj = get_direction(j, *tj);
                if di * dj != 0 {
                    panic!("Expected lines to only move in one dimension at a time");
                }
                while i != *ti || j != *tj {
                    i += di;
                    j += dj;
                    *world.get_mut((i,j)) = true;
                }
            },
            None => { return; }
        }
    }
}


/// Drop a piece of sand from i,j into world. If lands, set location to occupied
///
/// Returns true if landed, false if exited world
fn drop_sand((mut i, mut j): Coord, world: &mut Array2d<bool>) -> bool {
    let (sz_i, sz_j) = world.size_i();
    loop {
        let pj = j + 1;
        // Down
        let pi = i;
        if pi < 0 || pi >= sz_i || pj < 0 || pj >= sz_j { return false; }
        if !world.get((pi,pj)) { i = pi; j = pj; continue; }
        // Down-left
        let pi = i-1;
        if pi < 0 || pi >= sz_i || pj < 0 || pj >= sz_j { return false; }
        if !world.get((pi,pj)) { i = pi; j = pj; continue; }
        // Down-right
        let pi = i+1;
        if pi < 0 || pi >= sz_i || pj < 0 || pj >= sz_j { return false; }
        if !world.get((pi,pj)) { i = pi; j = pj; continue; }
        // Cannot fall further
        *world.get_mut((i,j)) = true;
        return true;
    }
}

fn solve(input: &str) -> usize {
    let lines = parse(input);

    let (max_i, max_j) = lines.iter().fold((0,0), |acc, x| x.iter().fold(acc, max_coord));

    let mut world = Array2d::new(false, max_i as usize + 1, max_j as usize + 1);
    
    for line in &lines {
        apply_line(line, &mut world);
    }

    let mut count = 0;
    while drop_sand((500,0), &mut world) {
        count += 1;
    }
    count
}

/// Drop a piece of sand from i,j into world. If lands, set location to occupied
///
/// Returns true if landed, false if exited world
fn drop_sand_p2((mut i, mut j): Coord, world: &mut Array2d<bool>) -> bool {
    let (sz_i, sz_j) = world.size_i();

    if *world.get((i,j)) {
        return false;
    }

    loop {
        let pj = j + 1;
        // Down
        let pi = i;
        if !(pi < 0 || pi >= sz_i || pj < 0 || pj >= sz_j) {
            if !world.get((pi,pj)) { i = pi; j = pj; continue; }
        }
        // Down-left
        let pi = i-1;
        if !(pi < 0 || pi >= sz_i || pj < 0 || pj >= sz_j) {
            if !world.get((pi,pj)) { i = pi; j = pj; continue; }
        }
        // Down-right
        let pi = i+1;
        if !(pi < 0 || pi >= sz_i || pj < 0 || pj >= sz_j) {
            if !world.get((pi,pj)) { i = pi; j = pj; continue; }
        }
        // Cannot fall further
        *world.get_mut((i,j)) = true;
        return true;
    }
}

fn solve_p2(input: &str) -> usize {
    let lines = parse(input);

    let (max_i, max_j) = lines.iter().fold((0,0), |acc, x| x.iter().fold(acc, max_coord));

    let mut world = Array2d::new(
        false,
        // Bit of a hack here, what if there are lines running to the left edge?
        (max_i + 1 + max_j) as usize, // Make space to the right for sand piles
        max_j as usize + 2); // Make space at the bottom before the floor
    
    for line in &lines {
        apply_line(line, &mut world);
    }

    let mut count = 0;
    while drop_sand_p2((500,0), &mut world) {
        count += 1;
    }
    count
}

pub fn run() {
    let input =
        std::fs::read_to_string("data/d14.txt")
        .expect("Failed to read input");
    println!("{}", solve(&input));
    println!("{}", solve_p2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    const EXAMPLE: &'static str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_example() {
        assert_eq!(solve(EXAMPLE), 24);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(solve_p2(EXAMPLE), 93);
    }
}
