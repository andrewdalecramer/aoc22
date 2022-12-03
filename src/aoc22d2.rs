


#[derive(Clone,Copy,Debug)]
enum Play {
    Rock, Paper, Scissors
}
use Play::*;


fn play_from_val(a: i32) -> Play {
    match a {
        0 => Rock,
        1 => Paper,
        2 => Scissors,
        _ => { panic!("Bad play"); }
    }
}

fn val_of(a: Play) -> i32 {
    match a {
        Rock => 0,
        Paper => 1,
        Scissors => 2,
    }
}

fn score_of_game(other: Play, me: Play) -> i32 {
    match (val_of(me) - val_of(other)+3)%3 {
        0 => 3,
        1 => 6,
        2 => 0,
        _ => { panic!("Who puts a bloody remainder operator in a language, really?!"); }
    }
}

fn score_of_play(a: Play) -> i32 {
    match a {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}


fn parse_play(line: usize, s: &str) -> Play {
    match s {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        c => { panic!("Invalid play: {} (line {})", c, line); }
    }
}

fn parse_outcome(line: usize, s: &str) -> i32 {
    match s {
        "X" => -1,
        "Y" =>  0,
        "Z" =>  1,
        c => { panic!("Invalid play: {} (line {})", c, line); }
    }
}


fn resolve_game_pt1(line: usize, s: &str) -> i32 {
    let other = parse_play(line, &s[0..1]);
    let me = parse_play(line, &s[2..3]);
    score_of_play(me) + score_of_game(other, me)
}

fn resolve_game_pt2(line: usize, s: &str) -> i32 {
    let other = parse_play(line, &s[0..1]);
    let outcome = parse_outcome(line, &s[2..3]);
    let me = play_from_val((val_of(other) + outcome +3) % 3);

    score_of_play(me) + score_of_game(other, me)
}

pub fn run() {
    // 9407 is incorrect, too low :/
    let src = std::fs::read_to_string("data/d2_actual.txt").expect("Failed to read input");

    let mut score = 0;
    let mut line_count = 0;
    for line in src.split("\n") {
        if line.len() == 0 { continue; }
        line_count += 1;
        score += resolve_game_pt2(line_count, line);
    }

    println!("Strategy results in {} points after {} rounds", score, line_count);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_of_game() {
        assert_eq!(score_of_game(Rock, Paper), 6);
        assert_eq!(score_of_game(Paper, Rock), 0);
        assert_eq!(score_of_game(Scissors, Rock), 6);
        assert_eq!(score_of_game(Rock, Scissors), 0);
    }

    #[test]
    fn test_resolve_game() {
        assert_eq!(resolve_game_pt1(0, "A Y"), 8);
        assert_eq!(resolve_game_pt1(0, "B X"), 1);
        assert_eq!(resolve_game_pt1(0, "C Z"), 6);
    }
    
    #[test]
    fn test_resolve_game2() {
        assert_eq!(resolve_game_pt2(0, "A Y"), 4);
        assert_eq!(resolve_game_pt2(0, "B X"), 1);
        assert_eq!(resolve_game_pt2(0, "C Z"), 7);
    }
}


