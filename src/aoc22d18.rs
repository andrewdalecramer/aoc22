
use crate::parse_utils::*;
use crate::arr_nd::ArrayND;


fn add_coords(a: &[isize; 3], b: &[isize; 3]) -> [isize; 3] {
    [a[0]+b[0], a[1]+b[1], a[2]+b[2]]
}

fn inside(coord: &[isize; 3], size: &[isize; 3]) -> bool {
    for i in 0..3 {
        if coord[i] < 0 { return false; }
        if coord[i] >= size[i] { return false; }
    }
    return true;
}

const NEIGHBOURS: [[isize;3]; 6] = [
    [-1,  0,  0],
    [ 1,  0,  0],
    [ 0, -1,  0],
    [ 0,  1,  0],
    [ 0,  0, -1],
    [ 0,  0,  1]
];


fn multi_max<T: Ord+Copy, const N: usize>(mut a: [T; N], b: &[T;N]) -> [T;N] {
    for i in 0..N {
        a[i] = std::cmp::max(a[i], b[i]);
    }
    a
}

fn parse(source: &str) -> ArrayND<bool,3> {
    let mut coords = Vec::new();
    for line in source.split("\n") {
        if line == "" { continue; }
        let mut it = get_byte_iterator(line);
        let a = parse_number(line, &mut it);
        assert!(consume_sequence(&mut it, ","));
        let b = parse_number(line, &mut it);
        assert!(consume_sequence(&mut it, ","));
        let c = parse_number(line, &mut it);
        coords.push([a,b,c]);
    }

    let mut size = coords.iter().fold([0,0,0], multi_max);
    for s in &mut size { *s+=1; }
    let mut output = ArrayND::new(false, size);
    for coord in coords {
        *output.get_mut(&coord) = true;
    }
    output
}

fn flood_fill_holes(
        start: [isize; 3],
        blocks: &ArrayND<bool,3>,
        new_blocks: &mut ArrayND<bool,3>)
{
    if !inside(&start, &blocks.size_i()) { return; }

    // Don't fill if the original has a block there
    // or if the new one already has it tagged as empty
    if *blocks.get(&start) || !new_blocks.get(&start) { return; }

    *new_blocks.get_mut(&start) = false;

    for n in &NEIGHBOURS {
        flood_fill_holes(add_coords(&start, n), blocks, new_blocks);
    }
}

fn remove_pockets(blocks: ArrayND<bool,3>) -> ArrayND<bool,3> {
    let mut output = ArrayND::newu(true, blocks.size_u());
    
    let size = blocks.size_i();
    for i in 0..size[0] {
        for j in 0..size[1] {
            flood_fill_holes([i,j,0], &blocks, &mut output);
            flood_fill_holes([i,j,size[2]-1], &blocks, &mut output);
        }
    }
    for j in 0..size[0] {
        for k in 0..size[1] {
            flood_fill_holes([0,j,k], &blocks, &mut output);
            flood_fill_holes([size[0]-1,j,k], &blocks, &mut output);
        }
    }
    for i in 0..size[0] {
        for k in 0..size[1] {
            flood_fill_holes([i,0,k], &blocks, &mut output);
            flood_fill_holes([i,size[1]-1,k], &blocks, &mut output);
        }
    }
    output
}


fn solve(input: &str, extern_only: bool) -> usize {
    let blocks = parse(input);
    let blocks = if extern_only { remove_pockets(blocks) } else { blocks };

    let size = blocks.size_i();
    let mut faces = 0;
    for i in 0..size[0] {
        for j in 0..size[1] {
            for k in 0..size[2] {
                let coord = [i,j,k];
                if *blocks.get(&coord) {
                    for n in &NEIGHBOURS {
                        let ind = add_coords(&coord, n);
                        if inside(&ind, &size) {
                            if !blocks.get(&ind) {
                                faces += 1;
                            }
                        } else {
                            faces += 1;
                        }
                    }
                }
            }
        }
    }
    faces
}


pub fn run() {
    let input =
        std::fs::read_to_string("data/d18.txt")
        .expect("Failed to read input");
    println!("{}", solve(&input, false));
    println!("{}", solve(&input, true));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    const EXAMPLE11: &'static str = "1,1,1";
    const EXAMPLE12: &'static str = "";
    const EXAMPLE21: &'static str = "1,1,1\n2,1,1";
    const EXAMPLE22: &'static str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_example() {
        assert_eq!(solve(EXAMPLE11, false), 6);
        assert_eq!(solve(EXAMPLE12, false), 0);
        assert_eq!(solve(EXAMPLE21, false), 10);
        assert_eq!(solve(EXAMPLE22, false), 64);
    }
    
    #[test]
    fn test_example_p2() {
        assert_eq!(solve(EXAMPLE11, true), 6);
        assert_eq!(solve(EXAMPLE12, true), 0);
        assert_eq!(solve(EXAMPLE21, true), 10);
        assert_eq!(solve(EXAMPLE22, true), 58);
    }
}
