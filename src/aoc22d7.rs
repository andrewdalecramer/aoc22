
struct DirObj<'a> {
    listed: bool,
    subs: Vec<Obj<'a>>,
}


enum ObjType<'a> {
    File(usize),
    Dir(DirObj<'a>)
}
use ObjType::*;


struct Obj<'a> {
    name: &'a str,
    obj: ObjType<'a>,
}

impl<'a> Obj<'a> {
    fn new_dir(name: &'a str) -> Obj<'a> {
        Obj {
            name: name,
            obj: Dir(DirObj {
                listed: false,
                subs: Vec::new(),
            }),
        }
    }
    
    fn new_file(name: &'a str, size: usize) -> Obj<'a> {
        Obj {
            name: name,
            obj: File(size)
        }
    }

    fn listing_help(&self, indent: &String, output: &mut dyn std::io::Write) {
        // Argh this is more complex than I thought it would be!
        match &self.obj {
            File(size) => {
                write!(
                    output,
                    "{}- {} (file, size={})\n",
                    &indent, self.name, size).unwrap();
            },
            Dir(dir_obj) => {
                write!(
                    output,
                    "{}- {} (dir)\n",
                    &indent, self.name).unwrap();
                let my_indent = indent.clone() + "  ";
                for obj in &dir_obj.subs {
                    obj.listing_help(&my_indent, output);
                }
            }
        }
    }

    fn listing(&self) -> String {
        let mut buffer = Vec::new();
        self.listing_help(&"".to_string(), &mut buffer);
        std::str::from_utf8(&buffer).unwrap().to_string()
    }
}

fn get_cwd_help<'a, 'b>(
        level: usize,
        current_path: &Vec<&str>,
        current_obj: &'b mut Obj<'a>)
    -> &'b mut DirObj<'a>
{
    match &mut current_obj.obj {
        File(_) => { panic!("Not a file"); },
        Dir(dir_obj) => {
            if level == current_path.len() {
                return dir_obj;
            } else {
                let level_name = current_path[level];
                for obj in &mut dir_obj.subs {
                    if obj.name == level_name {
                        return get_cwd_help(level + 1, current_path, obj);
                    }
                }
                panic!(
                    "Couldn't find subdirectory {:?} from path {:?}",
                    level_name,
                    current_path);
            }
        }
    }
}

fn get_cwd<'a, 'b>(current_path: &Vec<&str>, root: &'b mut Obj<'a>)
    -> &'b mut DirObj<'a>
{
    get_cwd_help(0, current_path, root)
}

fn parse_input<'a>(input: &'a str) -> Obj<'a> {
    let mut root = Obj::new_dir("/");

    let mut current_path = Vec::<&str>::new();
    let mut it = input.split("\n").enumerate().peekable();
    loop {
        match it.next() {
            Some((_, "")) => { },
            Some((line_num, "$ ls")) => {
                let current_dir = get_cwd(&current_path, &mut root);
                if current_dir.listed {
                    panic!("Listed same dir twice ({})", line_num);
                }
                current_dir.listed = true;
                loop {
                    match it.peek() {
                        Some((_line_num, line)) => {
                            if line == &"" {
                                break;
                            }
                            let mut line_it = line.split(" ");
                            let type_bit = line_it.next().unwrap();
                            if type_bit == "$" {
                                break;
                            }
                            let name_bit = line_it.next().unwrap();
                            if type_bit == "dir" {
                                current_dir.subs.push(Obj::new_dir(name_bit));
                            } else {
                                current_dir.subs.push(Obj::new_file(
                                        name_bit, type_bit.parse().unwrap()));
                            }
                            it.next();
                        }
                        None => {
                            break;
                        }
                    }
                }
            },
            Some((_line_num, line)) => {
                let mut line_it = line.split(" ");
                assert_eq!(line_it.next().unwrap(), "$");
                assert_eq!(line_it.next().unwrap(), "cd");
                let folder = line_it.next().unwrap();
                match folder {
                    ".." => { current_path.pop().unwrap(); },
                    "/" => { current_path.clear(); },
                    val => { current_path.push(val); },
                }
            },
            None => { break; },
        }
    }
    root
}


/// Returns (solution_value, total_size)
fn walk_p1(obj: &Obj) -> (usize, usize) {
    match &obj.obj {
        File(size) => {
            (0, *size)
        },
        Dir(dir_obj) => {
            let mut total_sol = 0;
            let mut total_self = 0;
            for obj in &dir_obj.subs {
                let (sol_bit, sz_bit) = walk_p1(obj);
                total_sol += sol_bit;
                total_self += sz_bit;
            }
            if total_self <= 100000 {
                (total_sol+total_self, total_self)
            } else {
                (total_sol, total_self)
            }
        },
    }
}

fn solvep1(root: &Obj) -> usize {
    let (sol, _) = walk_p1(root);
    sol
}

/// Returns (solution_value, total_size)
fn walk_p2(obj: &Obj, target: usize) -> (usize, usize) {
    match &obj.obj {
        File(size) => {
            (usize::MAX, *size)
        },
        Dir(dir_obj) => {
            let mut best = usize::MAX;
            let mut total_self = 0;
            for obj in &dir_obj.subs {
                let (sol_bit, sz_bit) = walk_p2(obj, target);
                // sz_bit can't be the solution as it could be a File
                if sol_bit >= target && sol_bit < best {
                    best = sol_bit;
                }
                total_self += sz_bit;
            }
            if total_self >= target && total_self < best {
                best = total_self;
            }
            (best, total_self)
        }
    }
}

fn solvep2(root: &Obj) -> usize {
    let (_, total_used) = walk_p1(root);
    let total_free = 70000000 - total_used;
    let target = 30000000 - total_free;
    println!("Target: {}", target);
    let (sol, _) = walk_p2(root, target);
    sol
}

pub fn run() {
    let input = std::fs::read_to_string("data/d7.txt").expect("Failed to read input");
    let root = parse_input(&input);
    println!("{}", root.listing());
    println!("part1: {}", solvep1(&root));
    println!("part2: {}", solvep2(&root));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example() {
        let example = 
            "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let listing = "- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)
";
        let root = parse_input(example);
        println!("{}", root.listing());
        assert_eq!(root.listing(), listing);
        assert_eq!(solvep1(&root), 95437);
        assert_eq!(solvep2(&root), 24933642);
    }
}
