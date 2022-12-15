
use std::cmp::Ordering;

type Num = u8;

type It<'a> = std::iter::Peekable<std::str::Bytes<'a>>;

#[derive(Clone,Debug,PartialEq)]
enum Element {
    Item(Num),
    List(Vec<Element>),
}


fn parse_number(iterator: &mut It) -> Num {
    let mut num = 0;
    loop {
        match iterator.peek() {
            Some(b']') | Some(b',') => {
                return num;
            },
            Some(v) => {
                if *v < b'0' || *v > b'9' {
                    panic!("Unexpected character parsing number {}", v);
                }
                num = 10*num + (v - b'0');
                iterator.next();
            },
            None => {
                panic!("Unexpected end of line parsing number");
            }
        }
    }
}

fn parse_list(iterator: &mut It) -> Vec<Element> {
    assert_eq!(
        iterator
            .next()
            .expect("Expected [ when parsing list"),
        b'[',
        "Expected [ when parsing list");
    
    let mut output = Vec::new();
    loop {
        match iterator.peek() {
            Some(b']') => {
                iterator.next();
                return output;
            }
            Some(_) => {
                output.push(parse_element(iterator));
                if iterator.peek() == Some(&b',') {
                    iterator.next();
                }
            },
            None => {
                panic!("Unexpected end of line (expected element or ])");
            }
        }
    }
}

fn parse_element(iterator: &mut It) -> Element {
    match iterator.peek() {
        Some(b'[') => {
            Element::List(parse_list(iterator))
        },
        Some(v) if b'0' <= *v && *v <= b'9' => {
            Element::Item(parse_number(iterator))
        },
        Some(v) => {
            panic!("Unexpected byte {} (expected element)", v);
        },
        None => {
            panic!("Unexpected end of line (expected element)");
        }
    }
}

fn parse_line(line: &str) -> Element {
    let mut it = line.bytes().peekable();
    let output = parse_list(&mut it);
    match it.next() {
        Some(_) => {
            panic!("Unexpected content at end of line");
        },
        None => { },
    }
    Element::List(output)
}

fn packets_in_order(p1: &Element, p2: &Element) -> Ordering {
    match p1 {
        Element::Item(p1num) => match p2 {
            Element::Item(p2num) => {
                if *p1num < *p2num  {
                    return Ordering::Less;
                } else if *p1num > *p2num {
                    return Ordering::Greater;
                } else {
                    return Ordering::Equal;
                }
            },
            Element::List(p2list) => {
                if p2list.len() == 0 {
                    return Ordering::Greater;
                }
                return packets_in_order(
                    &Element::List(vec![Element::Item(*p1num)]),
                    &Element::List(p2list.to_vec()));
            }
        },
        Element::List(p1list) => match p2 {
            Element::Item(p2num) => {
                if p1list.len() == 0 {
                    return Ordering::Less;
                }
                return packets_in_order(
                    &Element::List(p1list.to_vec()),
                    &Element::List(vec![Element::Item(*p2num)]));
            },
            Element::List(p2list) => {
                for (p1val, p2val) in std::iter::zip(p1list, p2list) {
                    match packets_in_order(p1val, p2val) {
                        Ordering::Equal => {},
                        not_same => { return not_same; }
                    }
                }
                if p1list.len() < p2list.len() {
                    return Ordering::Less;
                } else if p1list.len() > p2list.len() {
                    return Ordering::Greater;
                } else {
                    return Ordering::Equal;
                }
            },
        }
    }
}

fn solve(input: &str) -> usize {
    let mut src_it = input.split("\n");

    let mut index = 1;
    let mut output = 0;
    loop {
        let line1 = match src_it.next() {
            Some(v) => v,
            None => { break; }
        };
        if line1 == "" {
            assert_eq!(src_it.next(), None);
            break;
        }
        let p1 = parse_line(line1);
        let p2 = parse_line(src_it.next().expect("Expected packet"));

        if packets_in_order(&p1,&p2) != Ordering::Greater {
            output += index;
        }
        index += 1;

        match src_it.next() {
            Some("") => { },
            Some(_) => {
                panic!("Expected empty line or EOF after pair of packets");
            },
            None => { break; }
        }
    }
    output
}


fn fold_func(acc: usize, (index, element): (usize, &Element)) -> usize {
    match element {
        Element::Item(_) => acc,
        Element::List(l) => {
            if l.len() != 1 {
                acc
            } else {
                match &l[0] {
                    Element::Item(_) => acc,
                    Element::List(l2) => {
                        if l2.len() != 1 {
                            acc
                        } else {
                            if l2[0] == Element::Item(2) || l2[0] == Element::Item(6) {
                                acc*(index+1)
                            } else {
                                acc
                            }
                        }
                    }
                }
            }
        }
    }
}

fn solvep2(input: &str) -> usize {
    let mut src_it = input.split("\n");

    let mut packets = Vec::new();
    loop {
        let line1 = match src_it.next() {
            Some(v) => v,
            None => { break; }
        };
        if line1 == "" {
            assert_eq!(src_it.next(), None);
            break;
        }
        packets.push(parse_line(line1));
        packets.push(parse_line(src_it.next().expect("Expected packet")));

        match src_it.next() {
            Some("") => { },
            Some(_) => {
                panic!("Expected empty line or EOF after pair of packets");
            },
            None => { break; }
        }
    }
    packets.push(Element::List(vec![Element::List(vec![Element::Item(2)])]));
    packets.push(Element::List(vec![Element::List(vec![Element::Item(6)])]));
    packets.sort_by(packets_in_order);

    packets.iter().enumerate().fold(1, fold_func)
}

pub fn run() {
    let input =
        std::fs::read_to_string("data/d13.txt")
        .expect("Failed to read input");
    println!("{}", solve(&input));
    println!("{}", solvep2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    const EXAMPLE: &'static str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_example() {
        assert_eq!(solve(EXAMPLE), 13);
        assert_eq!(solvep2(EXAMPLE), 140);
    }
}
