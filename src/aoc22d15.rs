
use crate::parse_utils::*;
// use crate::arr2d::Array2d;

type Coord = (isize, isize);

#[derive(Clone,Copy,Debug)]
struct Sample {
    sensor: Coord,
    beacon: Coord,
}

/// A set of integers.
/// Represented by sorted non-adjacent and non-overlapping intervals
/// Ranges are _inclusive_
type IntegerSet = Vec<(isize, isize)>;

fn add_interval(start: isize, end: isize, set: &mut IntegerSet) {
    if start > end {
        panic!("Input interval is invalid ({} > {})", start, end);
    }
    // Adds an interval to the set
    for i in 0..set.len() {
        let (is,ie) = set[i];

        // Input interval is ahead of this index
        if ie + 1 < start {
            continue;
        }

        // Input interval is before this index
        if end + 1 < is {
            set.insert(i, (start, end));
            return;
        }

        // Input interval must be overlapping with i, does it overlap the next?
        // Advance from i such that any intervals ahead of j are disconnected
        // from the input interval.
        let mut j = i;
        while j+1 < set.len() {
            let (jp1s, _) = set[j+1];
            if end + 1 < jp1s {
                // j + 1 is ahead of the input set, so stop here
                break;
            }
            j += 1;
        }
        // Input interval overlaps all intervals from i through j
        let (_, je) = set[j];
        let new_start = std::cmp::min(is, start);
        let new_end = std::cmp::max(je, end);
        set[i] = (new_start, new_end);
        if j > i {
            set.drain(i+1..j+1);
        }
        return;
    }

    // Never overlapped any intervals, must be larger than all
    set.push((start, end))
}


fn parse(source: &str) -> Vec<Sample> {
    let mut output = Vec::new();
    for line in source.split("\n") {
        if line == "" { continue; }
        let mut it = get_byte_iterator(line);
        assert!(consume_sequence(&mut it, "Sensor at x="));
        let sx = parse_number(line, &mut it);
        assert!(consume_sequence(&mut it, ", y="));
        let sy = parse_number(line, &mut it);
        assert!(consume_sequence(&mut it, ": closest beacon is at x="));
        let bx = parse_number(line, &mut it);
        assert!(consume_sequence(&mut it, ", y="));
        let by = parse_number(line, &mut it);
        assert_eq!(it.next(), None);
        output.push(Sample { sensor: (sx,sy), beacon: (bx, by) });
    }
    output
}

fn add_sample(row: isize, sample: &Sample, set: &mut IntegerSet) {
    let (sx,sy) = sample.sensor;
    let (bx,by) = sample.beacon;

    let exclusion_distance = (sx-bx).abs() + (sy-by).abs();
    let sensor_distance_to_row = (row - sy).abs();
    let remaining_steps = exclusion_distance - sensor_distance_to_row;
    // Interval width will always be odd. If the row can be reached in a steps,
    // and if b steps remain, then the interval will be [a-b, a+b]
    if remaining_steps >= 0 {
        add_interval(sx - remaining_steps, sx + remaining_steps, set);
    }
}

fn solve(input: &str, row: isize) -> isize {
    let samples = parse(input);
    let mut blocked_values = Vec::new();
    for sample in &samples {
        add_sample(row, sample, &mut blocked_values);
    }
    
    // Collect all the beacons locations
    let mut beacons = Vec::new();
    for sample in &samples {
        let (bx,by) = &sample.beacon;
        if *by == row && !beacons.contains(bx) {
            beacons.push(*bx);
        }
    }

    let mut size = 0;
    for (s,e) in blocked_values {
        size += e - s + 1;
        // Account for locations we know beacons are at
        for x in &beacons {
            if s <= *x && *x <= e {
                size -= 1;
            }
        }
    }
    size
}


fn solve_p2(input: &str, max_val: isize) -> isize {
    let samples = parse(input);
    let mut blocked_values = Vec::new();
    let mut output = None;
    for row in 0..max_val {
        blocked_values.clear();
        for sample in &samples {
            add_sample(row, sample, &mut blocked_values);
        }
    
        for (s,e) in &blocked_values {
            if *e < 0 { continue; }
            if *s > max_val { continue; }
            // Really should test this to see if the width of the gap really 
            // is just 1.
            if *e < max_val {
                if output != None {
                    panic!("Found two answers");
                }
                output = Some((e+1, row));
            }
        }
    }
    match output {
        Some((x,y)) => y + x*4000000,
        None => {
            panic!("Found no answers");
        }
    }
}

pub fn run() {
    let input =
        std::fs::read_to_string("data/d15.txt")
        .expect("Failed to read input");
    println!("{}", solve(&input, 2000000));
    println!("{}", solve_p2(&input, 4000000));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    const EXAMPLE: &'static str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_example() {
        assert_eq!(solve(EXAMPLE, 10), 26);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(solve_p2(EXAMPLE, 20), 56000011);
    }
}
