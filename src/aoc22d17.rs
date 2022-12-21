
use crate::arr2d::Array2d;

// All coords are first X, then Y.
// X increases from left to right
// Y increases from bottom to top


fn make_rocks() -> [Array2d<bool>; 5] {
    // Each rock appears so that its left edge is two units away from the left
    // wall and its bottom edge is three units above the highest rock in the
    // room (or the floor, if there isn't one).
    // 
    // This means that the shapes should be left-bottom aligned 
    
    // ####
    let rock1 = Array2d::newu(true, 4, 1);

    // .#.
    // ###
    // .#.
    let mut rock2 = Array2d::newu(false, 3, 3);
    *rock2.get_mut((1,0)) = true;
    *rock2.get_mut((0,1)) = true;
    *rock2.get_mut((1,1)) = true;
    *rock2.get_mut((2,1)) = true;
    *rock2.get_mut((1,2)) = true;

    // ..#
    // ..#
    // ###
    let mut rock3 = Array2d::newu(false, 3, 3);
    *rock3.get_mut((0,0)) = true;
    *rock3.get_mut((1,0)) = true;
    *rock3.get_mut((2,0)) = true;
    *rock3.get_mut((2,1)) = true;
    *rock3.get_mut((2,2)) = true;

    // #
    // #
    // #
    // #
    let rock4 = Array2d::newu(true, 1, 4);
     
    // ##
    // ##
    let rock5 = Array2d::newu(true, 2, 2);
    [rock1, rock2, rock3, rock4, rock5]
}


fn parse(source: &str) -> Vec<isize> {
    let mut output = Vec::new();
    for c in source.chars() {
        match c {
            ' ' => { },
            '\n' => { },
            '<' => { output.push(-1); }
            '>' => { output.push( 1); }
            _ => {
                panic!("Unexpected character in input: {}", c);
            }
        }
    }
    output
}


fn place_rock(rock: &Array2d<bool>, state: &mut State) {
    let (rsx, rsy) = rock.size_i(); // Rock Size x|y
    // Increase shaft if necessary
    while state.rock_y + rsy > state.shaft.size_i().1 {
        state.shaft.add_row((0..state.shaft.size_i().0).map(|_| false));
    }

    // Place
    for ry in 0..rsy {
        for rx in 0..rsx {
            if *rock.get((rx,ry)) {
                let v = state.shaft.get_mut((state.rock_x+rx, state.rock_y+ry));
                if *v {
                    panic!("Placing rock over existing rock");
                }
                *v = true;
            }
        }
    }
}

fn can_move_to(x: isize, y: isize, rock: &Array2d<bool>, state: &State) -> bool {
    let (rsx, rsy) = rock.size_i(); // Rock Size x|y
    let (ssx, ssy) = state.shaft.size_i(); // Shaft Size x|y
    
    // Check bounds
    if x < 0 { return false; }
    if x + rsx > ssx { return false; }
    if y < 0 {
        if state.purged_rows > 0 {
            panic!("Hit bottom but we've purged rows");
        }
        return false;
    }

    // Check space
    for ry in 0..rsy {
        // No need to check above shaft
        if y + ry >= ssy { continue; }

        for rx in 0..rsx {
            if *rock.get((rx, ry)) && *state.shaft.get((x+rx, y+ry)) {
                return false;
            }
        }
    }

    // No problems!
    true
}

#[allow(dead_code)]
fn draw_shaft(shaft: &Array2d<bool>) {
    for y in (0..shaft.size_i().1).rev() {
        print!("|");
        for x in 0..shaft.size_i().0 {
            if *shaft.get((x,y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("|\n");
    }
    print!("+");
    for _ in 0..shaft.size_i().0 { print!("-"); }
    print!("+\n");
}


/// Tracks what locations are accessible and purges to that point
fn check_purge(state: &mut State) {
    // All locations are accessible at the top
    for a in &mut state.accessible {
        *a = true;
    }
    let mut inaccessible_line = None;
    'line_loop: for y in (2..state.shaft.size_i().1).rev() {
        // Grow for sideways movement
        for x in 0..(state.shaft.size_u().0-1) {
            state.accessible[x] = state.accessible[x] || state.accessible[x+1];
        }
        for x in (1..state.shaft.size_u().0).rev() {
            if x > 0 {
                state.accessible[x] = state.accessible[x] || state.accessible[x-1];
            }
        }
        // Block out solids
        for x in 0..state.shaft.size_i().0 {
            if *state.shaft.get((x,y)) {
                state.accessible[x as usize] = false;
            }
        }
        // Check if any accessible
        for a in &state.accessible {
            if *a {
                continue 'line_loop;
            }
        }

        // None are accessible!
        inaccessible_line = Some(y);
        break;
    }

    let remove_up_to =
        match inaccessible_line {
            Some(v) => v,
            None => {
                return;
            }
        };
    
    // PURGE!
    state.purged_rows += remove_up_to;
    state.shaft.drop_front(remove_up_to);
}

#[derive(Clone)]
struct State {
    // Where rocks are in the shaft
    shaft: Array2d<bool>,
    // Index of the current rock shape
    current_rock: usize,
    // Index of the current jet direcion
    current_dir: usize,
    // Position of the falling rock
    rock_x: isize,
    rock_y: isize,
    // A buffer for checking if things are accessible
    accessible: Vec<bool>,
    // Allow us to purge no longer relevant rows
    purged_rows: isize,
}

fn check_state_equivalent(a: &State, b: &State) -> bool {
    if a.current_rock % 5 != b.current_rock % 5 { return false; }
    if a.current_dir != b.current_dir { return false; }
    if a.rock_x != b.rock_x { return false; }
    if a.rock_y != b.rock_y { return false; }
    // Check shaft last as it's the most expensive
    if a.shaft != b.shaft { return false; }

    return true;
}

fn drop_rock(state: &mut State, rocks: &[Array2d<bool>; 5], dirs: &Vec<isize>, allow_purge: bool)
    -> isize
{
    loop {
        let dir = dirs[state.current_dir];
        state.current_dir = (state.current_dir + 1) % dirs.len();

        let rock = &rocks[state.current_rock % rocks.len()];
        if can_move_to(state.rock_x+dir, state.rock_y, rock, state) {
            state.rock_x += dir;
        }
        
        if can_move_to(state.rock_x, state.rock_y-1, rock, state) {
            state.rock_y -= 1;
        } else {
            place_rock(rock, state);
            if allow_purge {
                check_purge(state);
            }

            state.current_rock += 1;
            state.rock_x = 2;
            state.rock_y = state.shaft.size_i().1 + 3; // 3 units above bottom
            break;
        }
    }
    return state.shaft.size_i().1 + state.purged_rows
}

fn solve(input: &str, width: isize, n_rocks: usize, allow_purge: bool) -> isize {
    // Periodicity!
    //
    //  - Run for a while (10,000 rocks or so)
    //  - Remember the state
    //  - Advance, checking if the state looks the same as the 10,000th
    //  - Determine how many extra lines were added
    //  - Shazam!
    //
    let mut state = State {
        shaft: Array2d::new(false, width, 0),
        current_rock: 0,
        current_dir: 0,
        rock_x: 2,
        rock_y: 3,
        accessible: Vec::with_capacity(width as usize),
        purged_rows: 0,
    };

    let rocks = make_rocks();
    let dirs = parse(input);

    for _ in 0..width {
        state.accessible.push(false);
    }

    // If it's a small number of rocks, just drop em
    if n_rocks < 10000 {
        while state.current_rock < n_rocks {
            drop_rock(&mut state, &rocks, &dirs, allow_purge);
        }

        // draw_shaft(&state.shaft);
        return state.shaft.size_i().1 + state.purged_rows;
    }

    if !allow_purge {
        panic!("Must allow purge for large requests");
    }

    // Drop 10k rocks
    while state.current_rock < 10000 {
        drop_rock(&mut state, &rocks, &dirs, allow_purge);
    }
    let state2 = state.clone();
    
    // Look for a repeating pattern
    loop {
        if state.current_rock >= n_rocks {
            return state.shaft.size_i().1 + state.purged_rows;
        }
        
        drop_rock(&mut state, &rocks, &dirs, allow_purge);
        if check_state_equivalent(&state, &state2) {
            break;
        }
    }

    let cycle_period = state.current_rock - state2.current_rock;
    let cycle_rows = state.purged_rows - state2.purged_rows;

    let needed_rocks = n_rocks - state.current_rock;
    let num_cycles = needed_rocks / cycle_period;
    
    state.purged_rows += cycle_rows * num_cycles as isize;
    state.current_rock += cycle_period * num_cycles;
        
    println!(
        "Cycle exploited\n Period: {}\n New rock num: {}\n Remaining rocks: {}",
        cycle_period,
        state.current_rock,
        n_rocks - state.current_rock);
    while state.current_rock < n_rocks {
        drop_rock(&mut state, &rocks, &dirs, allow_purge);
    }

    return state.shaft.size_i().1 + state.purged_rows;
}


pub fn run() {
    let input =
        std::fs::read_to_string("data/d17.txt")
        .expect("Failed to read input");
    println!("{}", solve(&input, 7, 2022, true));
    println!("{}", solve(&input, 7, 1000000000000, true));
}



#[cfg(test)]
mod tests {
    use super::*;
    
    const EXAMPLE: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_example() {
        assert_eq!(solve(EXAMPLE, 7, 2022, false), 3068);
        assert_eq!(solve(EXAMPLE, 7, 2022, true), 3068);
    }

    #[test]
    fn test_example_actual() {
        let input =
            std::fs::read_to_string("data/d17.txt")
            .expect("Failed to read input");

        for v in [200, 250, 353, 354, 400] {
            println!("Test {}", v);
            assert_eq!(solve(&input, 7, v, false), solve(&input, 7, v, true));
        }

        assert_eq!(solve(&input, 7, 2022, false), 3067);
        assert_eq!(solve(&input, 7, 2022, true), 3067);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(solve(EXAMPLE, 7, 1000000000000, true), 1514285714288);
    }
}
