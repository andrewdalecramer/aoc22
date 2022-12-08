


// This one is ugly as because I rushed it. Tried to get the best time I could.
// 
// It works so I'm going to leave it

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut output = Vec::new();
    for line in input.split("\n") {
        if line == "" { continue; }
        let mut output_line = Vec::new();
        for c in line.bytes() {
            output_line.push((c - b'0')+1);
        }
        output.push(output_line);
    }
    output
}

fn make_seen(grid: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let mut seen = Vec::new();
    for _ in 0..grid.len() {
        let mut newnew = Vec::new();
        for _ in 0..grid[0].len() {
            newnew.push(false);
        }
        seen.push(newnew);
    }
    // top down
    for i in 0..grid[0].len() {
        let mut prev_height = 0;
        for j in 0..grid.len() {
            let v = grid[j][i];
            if v > prev_height {
                seen[j][i] = true;
                prev_height = v;
            }
        }
    }
    for i in 0..grid[0].len() {
        let mut prev_height = 0;
        for j in (0..grid.len()).rev() {
            let v = grid[j][i];
            if v > prev_height {
                seen[j][i] = true;
                prev_height = v;
            }
        }
    }
    for j in 0..grid.len() {
        let mut prev_height = 0;
        for i in 0..grid[0].len() {
            let v = grid[j][i];
            if v > prev_height {
                seen[j][i] = true;
                prev_height = v;
            }
        }
    }
    for j in 0..grid.len() {
        let mut prev_height = 0;
        for i in (0..grid[0].len()).rev() {
            let v = grid[j][i];
            if v > prev_height {
                seen[j][i] = true;
                prev_height = v;
            }
        }
    }
    seen
}

fn solvep1(grid: &Vec<Vec<u8>>) -> usize {
    let seen = make_seen(grid);
    let mut count = 0;
    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            if seen[j][i] {
                count += 1;
            }
        }
    }
    count
}


fn calc_scenic(grid: &Vec<Vec<u8>>, ii: usize, jj: usize) -> usize {
    let my_height = grid[jj][ii];
    let mut score = 1;

    let mut prev_height = 0;
    let mut count = 0;
    for i in (ii+1)..grid[0].len() {
        let v = grid[jj][i];
        count += 1;
        if v>= my_height { break; }
    }
    if jj == 3 && ii ==2 { println!("{}", count); }
    score = score * count;

    let mut prev_height = 0;
    let mut count = 0;
    for i in (0..ii).rev() {
        let v = grid[jj][i];
        count += 1;
        if v>= my_height { break; }
    }
    if jj == 3 && ii ==2 { println!("{}", count); }
    score = score * count;

    let mut prev_height = 0;
    let mut count = 0;
    for j in (jj+1)..grid[0].len() {
        let v = grid[j][ii];
        count += 1;
        if v>= my_height { break; }
    }
    if jj == 3 && ii ==2 { println!("{}", count); }
    score = score * count;

    let mut prev_height = 0;
    let mut count = 0;
    for j in (0..jj).rev() {
        let v = grid[j][ii];
        count += 1;
        if v>= my_height { break; }
    }
    if jj == 3 && ii ==2 { println!("{}", count); }
    score = score * count;

    score
}

fn make_scenic(grid: &Vec<Vec<u8>>) -> Vec<Vec<usize>> {
    let mut scenic = Vec::new();
    for _ in 0..grid.len() {
        let mut newnew = Vec::new();
        for _ in 0..grid[0].len() {
            newnew.push(0);
        }
        scenic.push(newnew);
    }
    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            scenic[j][i] = calc_scenic(grid, i, j);
        }
    }
    scenic
}

fn solvep2(grid: &Vec<Vec<u8>>) -> usize {
    let seen = make_seen(grid);
    let scenic = make_scenic(grid);
    println!("{:?}", scenic);
    let mut best = 0;
    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            if scenic[j][i] > best {
                best = scenic[j][i];
            }
        }
    }
    best
}

pub fn run() {
    let input = std::fs::read_to_string("data/d8.txt").expect("Failed to read input");
    let grid = parse_input(&input);
    println!("{}", solvep1(&grid));
    println!("{}", solvep2(&grid)); // 2100 is not right
        //println!("{}", root.listing());
    //    println!("part1: {}", solvep1(&root));
        //println!("part2: {}", solvep2(&root));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example() {
        let example = 
            "30373
25512
65332
33549
35390";
        let grid = parse_input(example);
        assert_eq!(solvep1(&grid), 21);
        assert_eq!(solvep2(&grid), 8);
        assert!(false);
    }
}
