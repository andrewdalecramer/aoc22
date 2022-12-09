

use std::collections::HashSet;

#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}


// Turns a text move command to a direction vector and a distance
fn parse_move(line: &str) -> (Vec2, usize) {
    let mut it = line.split(" ");
    
    let dir = match it.next() {
        Some("U") => Vec2 {x: 0, y: 1},
        Some("D") => Vec2 {x: 0, y:-1},
        Some("L") => Vec2 {x:-1, y: 0},
        Some("R") => Vec2 {x: 1, y: 0},
        Some(s) => {
            panic!("Unexpected direction: \"{}\"", s);
        },
        None => {
            panic!("Missing direction");
        },
    };

    let dist =
        it.next()
        .expect("Missing distance")
        .parse()
        .expect("Failed to parse distance");
    (dir, dist)
}

fn print_visitations(visitations: &HashSet<Vec2>) {
    let mut xmin = 0;
    let mut xmax = 0;
    let mut ymin = 0;
    let mut ymax = 0;
    for Vec2 {x, y} in visitations {
        if *x > xmax {
            xmax = *x;
        } if *x < xmin {
            xmin = *x;
        }
        if *y > ymax {
            ymax = *y;
        } if *y < ymin {
            ymin = *y;
        }
    }
    for y in (ymin..(ymax+1)).rev() {
        for x in xmin..(xmax+1) {
            if visitations.contains(&Vec2{x:x, y:y}) {
                print!("#")
            } else {
                print!(".")
            }
        }
        print!("\n");
    }
}


fn solve(input: &str, length: usize) -> usize {
    let mut visitations: HashSet<Vec2> = HashSet::new();
    
    let mut rope: Vec<Vec2> = Vec::with_capacity(length);
    for _ in 0..length {
        rope.push(Vec2 {
            x: 0,
            y: 0,
        });
    }
    visitations.insert(rope[length-1]);

    for line in input.split("\n") {
        if line == "" { continue; }
        let (dir, dist) = parse_move(line);
        for _ in 0..dist {
            rope[0].x += dir.x;
            rope[0].y += dir.y;
            for i in 0..(length-1) {
                if (rope[i].x - rope[i+1].x).abs() >= 2
                    || (rope[i].y - rope[i+1].y).abs() >= 2
                {
                    if rope[i].x > rope[i+1].x {
                        rope[i+1].x += 1;
                    } else if rope[i].x < rope[i+1].x {
                        rope[i+1].x -= 1;
                    }
                    if rope[i].y > rope[i+1].y {
                        rope[i+1].y += 1;
                    } else if rope[i].y < rope[i+1].y {
                        rope[i+1].y -= 1;
                    }
                }
            }
            visitations.insert(rope[length-1]);
        }
    }
    
    print_visitations(&visitations);
    visitations.len()
}


pub fn run() {
    let input = std::fs::read_to_string("data/d9.txt").expect("Failed to read input");
    println!("{}", solve(&input, 2));
    println!("{}", solve(&input, 10));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example() {
        let example = 
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(solve(&example, 2), 13);
        assert_eq!(solve(&example, 10), 1);
        let example2 =
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(solve(&example2, 10), 36);
        // assert!(false);
    }
}
