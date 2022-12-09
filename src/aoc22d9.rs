

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


fn solvep1(input: &str) -> usize {
    let mut visitations: HashSet<Vec2> = HashSet::new();
    
    let mut head = Vec2 {
        x: 0,
        y: 0,
    };
    let mut tail = head;
    visitations.insert(tail);

    for line in input.split("\n") {
        if line == "" { continue; }
        let (dir, dist) = parse_move(line);
        for _ in 0..dist {
            head.x += dir.x;
            head.y += dir.y;
            if (head.x - tail.x).abs() >= 2 || (head.y - tail.y).abs() >= 2 {
                if head.x > tail.x {
                    tail.x += 1;
                } else if head.x < tail.x {
                    tail.x -= 1;
                }
                if head.y > tail.y {
                    tail.y += 1;
                } else if head.y < tail.y {
                    tail.y -= 1;
                }
            }
            visitations.insert(tail);
        }
    }
    
    print_visitations(&visitations);
    visitations.len()
}


pub fn run() {
    let input = std::fs::read_to_string("data/d9.txt").expect("Failed to read input");
    println!("{}", solvep1(&input));
    // println!("{}", solvep2(&grid));
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
        assert_eq!(solvep1(&example), 13);
        assert_eq!(solvep2(&example), 1);
        let example2 =
            "R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20";
        assert_eq!(solvep2(&example), 36);
        assert!(false);
    }
}
