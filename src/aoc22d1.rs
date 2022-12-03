
const N: usize = 3;

pub fn new_elf(mut new_val: f64, top_elves: &mut [f64; N]) {
    for i in 0..N {
        let this_elf = top_elves[i];
        if new_val > this_elf {
            top_elves[i] = new_val;
            new_val = this_elf;
        }
    }
}

pub fn sum(top_elves: &[f64; N]) -> f64 {
    let mut val = 0.0;
    for elf in top_elves {
        val += elf;
    }
    val
}

pub fn run() {
    let src = std::fs::read_to_string("data/d1_actual.txt").expect("Failed to read input");

    // Assume no elves means zero calories (what is the maximum of the empty set?)
    let mut top_elves = [0.0; N];
    let mut this = 0.0; 
    for line in src.split("\n") {
        match line {
            "" => {
                new_elf(this, &mut top_elves);
                this = 0.0;
            },
            str_val => {
                let val =
                    str_val.parse::<f64>()
                    .expect("Expected only blank lines and numbers");
                this += val;
            }
        }
    }
    println!("The fattest hobbit has {} Calories", top_elves[0]);
    println!("The fattest {} hobbitses has {} Calories", N, sum(&top_elves));
}
