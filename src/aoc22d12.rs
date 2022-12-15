
type Distance = u16;

const Z_VAL: i8 = b'z' as i8 - b'a' as i8;
const S_VAL: i8 = b'S' as i8 - b'a' as i8;
const E_VAL: i8 = b'E' as i8 - b'a' as i8;

/// Two dimensional rectangular array of size (I x J)
///
/// First dimension is the fastest moving index, so:
///  - Fast to iterate along first dimension (I)
///  - Fast to expand along second dimension (J)
///
/// Uses isize so we can do arithmetic around coordinates
struct Array2d<T: Copy> {
    data: Vec<T>,
    size_i: usize,
    size_j: usize,
}

impl<T: Copy> Array2d<T> {
    fn new(initial: T, size_i: usize, size_j: usize) -> Array2d<T> {
        let mut data = Vec::with_capacity(size_i*size_j);
        for _ in 0..(size_i*size_j) {
            data.push(initial);
        }
        Array2d {
            data: data,
            size_i: size_i,
            size_j: size_j,
        }
    }
    
    fn new_empty() -> Array2d<T> {
        Array2d {
            data: Vec::new(),
            size_i: 0,
            size_j: 0,
        }
    }

    fn get(&self, (i, j): (isize, isize)) -> &T {
        &self.data[i as usize + self.size_i*j as usize]
    }

    fn get_mut(&mut self, (i, j): (isize, isize)) -> &mut T {
        &mut self.data[i as usize + self.size_i*j as usize]
    }
    
    /// Create a new row (add 1 to second dimension)
    /// 
    ///  - Fills with items in iterator
    ///  - Either size_i == size_j == 0 or iterator must provide exactly size_i
    ///    elements.
    fn add_row<I: std::iter::Iterator<Item=T>>(&mut self, iterator: I) {
        self.data.reserve(self.size_i);

        let mut count = 0;
        for item in iterator {
            count += 1;
            self.data.push(item);
        }
        if self.size_i == 0 && self.size_j == 0 {
            self.size_i = count;
        } else if count != self.size_i {
            panic!("Unexpect number of items added to Array2d");
        }
        self.size_j += 1;
    }
}


fn parse(source: &str) -> Array2d<i8>  {
    let mut height = Array2d::new_empty();
    for line in source.split("\n") {
        if line != "" {
            height.add_row(line.bytes().map(|x| (x as i8) - (b'a' as i8)));
        }
    }
    height
}

fn find_start_end(height: &mut Array2d<i8>) -> ((isize,isize),(isize,isize)) {
    let mut start = None;
    let mut end = None;
    for i in 0..(height.size_i as isize) {
        for j in 0..(height.size_j as isize) {
            let h = height.get_mut((i,j));
            if *h == S_VAL {
                *h = 0;
                if start == None {
                    start = Some((i,j));
                } else {
                    panic!("Found two starts");
                }
            } else if *h == E_VAL {
                *h = Z_VAL;
                if end == None {
                    end = Some((i,j));
                } else {
                    panic!("Found two ends");
                }
            }
        }
    }
    let start = start.expect("Couldn't find start");
    let end = end.expect("Couldn't find end");
    (start, end)
}


/// Depth first search
///
/// Given we can move to the current location in dist steps, look at all
/// cardinal directions around to see if it leads to the end
fn find_shortest_path(
        height: &Array2d<i8>,
        distances: &mut Array2d<Distance>,
        dist: Distance,
        mut best: Distance,
        current: (isize,isize),
        end: (isize,isize))
    -> Distance 
{
    // Check if we've gone further than required
    if dist >= best {
        return best;
    }

    // Check if we've arrived
    if current == end {
        return dist;
    }

    // Update the distance map
    {
        let d = distances.get_mut(current);
        if *d <= dist {
            // We can get there faster by another route
            return best;
        } else {
            // We've found the fastest route to the here
            *d = dist;
        }
    }

    // Search around
    let h = height.get(current);
    let (i,j) = current;
    for (di, dj) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let i_new = i + di;
        let j_new = j + dj;
        // Will we move off the map?
        if i_new < 0 { continue; }
        if j_new < 0 { continue; }
        if i_new as usize >= height.size_i { continue; }
        if j_new as usize >= height.size_j { continue; }
        // Try move there
        let new = (i_new, j_new);
        let h_new = *height.get(new);
        if h_new > h + 1 {
            continue; // Too high a jump
        }
        best = 
            find_shortest_path(
                height,
                distances,
                dist + 1,
                best,
                (i_new, j_new),
                end);
    }
    return best;
}



fn solvep1(input: &str) -> Distance {
    let mut height = parse(input);
    let (start, end) = find_start_end(&mut height);
    let mut distances = Array2d::new(Distance::MAX, height.size_i, height.size_j);
    let dist = find_shortest_path(
        &height,
        &mut distances,
        0,
        Distance::MAX,
        start,
        end);
    if dist == Distance::MAX {
        panic!("Couldn't find shortest path");
    }
    dist
}

/// Depth first search
///
/// Given we can move to the current location in dist steps, look at all
/// cardinal directions around to see if it leads to the end
///
/// We are just looking for a path to a level of 0, so no destination known
///
/// Rules for movement are opposite part 1 as we're backtracking
fn find_shortest_path_p2(
        height: &Array2d<i8>,
        distances: &mut Array2d<Distance>,
        dist: Distance,
        mut best: Distance,
        current: (isize,isize))
    -> Distance 
{
    // Check if we've gone further than required
    if dist >= best {
        return best;
    }

    // Check if we've arrived
    let h = *height.get(current);
    if h == 0 {
        return dist;
    }

    // Update the distance map
    {
        let d = distances.get_mut(current);
        if *d <= dist {
            // We can get there faster by another route
            return best;
        } else {
            // We've found the fastest route to the here
            *d = dist;
        }
    }

    // Search around
    let (i,j) = current;
    for (di, dj) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let i_new = i + di;
        let j_new = j + dj;
        // Will we move off the map?
        if i_new < 0 { continue; }
        if j_new < 0 { continue; }
        if i_new as usize >= height.size_i { continue; }
        if j_new as usize >= height.size_j { continue; }
        // Try move there
        let new = (i_new, j_new);
        let h_new = *height.get(new);
        if h > h_new + 1 {
            continue; // Too high a jump
        }
        best =
            find_shortest_path_p2(
                height,
                distances,
                dist + 1,
                best,
                (i_new, j_new));
    }
    return best;
}

fn solvep2(input: &str) -> Distance {
    let mut height = parse(input);
    let (_, end) = find_start_end(&mut height);
    let mut distances = Array2d::new(Distance::MAX, height.size_i, height.size_j);
    let dist = find_shortest_path_p2(
        &height,
        &mut distances,
        0,
        Distance::MAX,
        end);

    if dist == Distance::MAX {
        panic!("Couldn't find shortest path");
    }
    dist
}

pub fn run() {
    let input =
        std::fs::read_to_string("data/d12.txt")
        .expect("Failed to read input");
    println!("{}", solvep1(&input));
    println!("{}", solvep2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    const EXAMPLE: &'static str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_example() {
        assert_eq!(solvep1(EXAMPLE), 31);
        assert_eq!(solvep2(EXAMPLE), 29);
    }
}
