
use crate::parse_utils::*;
use crate::arr2d::Array2d;

type FlowRate = isize;
type Time = isize;

#[derive(Debug)]
struct RoomRaw {
    flow: FlowRate,
    tunnels: Vec<usize>,
    name: String,
}

#[derive(Debug)]
struct ProblemRaw {
    total_flow_rate: FlowRate,
    aa_index: usize,
    rooms: Vec<RoomRaw>,
}


fn parse_room_code(it: &mut ByteIterator) -> String {
    let (_, a) = it.next().expect("expected two characters for room code");
    let (_, b) = it.next().expect("expected two characters for room code");
    std::str::from_utf8(&[a,b]).expect("characters for room code are invalid").to_string()
}

fn parse(source: &str) -> ProblemRaw {
    let mut rooms = Vec::new();

    let mut aa = None;
    let mut room_directory = std::collections::HashMap::new();
    let mut total_flow_rate = 0;

    for line in source.split("\n") {
        if line == "" { continue; }
        let mut it = get_byte_iterator(line);
        assert!(consume_sequence(&mut it, "Valve "));
        let room_code = parse_room_code(&mut it);
        assert!(consume_sequence(&mut it, " has flow rate="));
        let flow_rate = parse_number(line, &mut it);

        if room_directory.insert(room_code.clone(), rooms.len()) != None {
            panic!("Found two rooms with the code {}", room_code);
        }
        if room_code == "AA" {
            aa = Some(rooms.len());
        }
        total_flow_rate += flow_rate;
        rooms.push(RoomRaw { name: room_code, flow: flow_rate, tunnels: Vec::new() });
    }
    
    let mut index = 0;
    for line in source.split("\n") {
        if line == "" { continue; }
        let room = &mut rooms[index];

        let mut it = get_byte_iterator(line);
        assert!(consume_sequence(&mut it, "Valve "));
        parse_room_code(&mut it);
        assert!(consume_sequence(&mut it, " has flow rate="));
        parse_number(line, &mut it);
        assert!(consume_sequence(&mut it, "; tunnel"));

        if !consume_sequence(&mut it, "s lead to valves ") {
            assert!(consume_sequence(&mut it, " leads to valve "));
        }

        loop {
            let room_code = parse_room_code(&mut it);
            (*room).tunnels.push(
                match room_directory.get(&room_code) {
                    Some(v) => *v,
                    None => {
                        panic!("Unknown room code: '{}'", room_code);
                    }
                });
            if !consume_sequence(&mut it, ", ") {
                break;
            }
        }
        index += 1;
    }

    ProblemRaw {
        total_flow_rate: total_flow_rate,
        aa_index: aa.expect("Failed to find room 'AA'"),
        rooms: rooms,
    }
}


struct Problem {
    num_rounds: isize,
    total_flow_rate: FlowRate,
    aa_index: usize,
    rooms: Vec<FlowRate>,
    room_names: Vec<String>,
    adjacency: Array2d<Time>,
}

/// Remaps rooms to only have the initial room and non zero rooms
/// Returns:
///     - the new aa_room index
///     - a mapping from old to new, with dropped rooms as usize::MAX
///     - a new vector of flow rates for rooms
fn remap_rooms(raw: &ProblemRaw) -> (usize, Vec<usize>, Vec<FlowRate>, Vec<String>) {
    let mut remapping = Vec::with_capacity(raw.rooms.len());
    let mut new_rooms = Vec::new();
    let mut new_room_names = Vec::new();
    let mut new_aa_index = usize::MAX;
    for (idx, room) in raw.rooms.iter().enumerate() {
        if idx == raw.aa_index {
            new_aa_index = new_rooms.len();
        }
        if room.flow == 0 && idx != raw.aa_index {
            remapping.push(usize::MAX);
        } else {
            remapping.push(new_rooms.len());
            new_rooms.push(room.flow);
            new_room_names.push(room.name.clone());
        }
    }

    (new_aa_index, remapping, new_rooms, new_room_names)
}

/// Finds the shortest paths from index to all other nodes in the RAW problem
fn shortest_paths(index: usize, raw: &ProblemRaw) -> Vec<Time> {

    // output is the amount of time taken to go from here to there
    let mut output = Vec::with_capacity(raw.rooms.len());
    for i in 0..raw.rooms.len() {
        if i == index {
            output.push(0);
        } else {
            output.push(Time::MAX);
        }
    }

    // Use BFS to search the graph
    //
    // Basically, just look at neighbours and see if going from here to there
    // improves their score, and if so, try jumping on again
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(index);
    loop {
        match queue.pop_front() {
            Some(current) => {
                let current_time = output[current];

                for n in raw.rooms[current].tunnels.clone() {
                    let n_time = output[n];
                    if current_time + 1 < n_time {
                        output[n] = current_time + 1;
                        queue.push_back(n);
                    }
                }
            },
            None => { break; },
        }
    }

    output
}

/// Reduce down the problem, many rooms have zero flow and so lead to walking
/// around in circles not accomplishing anything.
/// Our input has 128 tunnels and 15 non-zero valves
/// Reducing gets us 16 choose 2 = 120 tunnels, but we can also avoid 
/// revisiting rooms that were already visited
fn reduce_problem(raw: &ProblemRaw, num_rounds: Time) -> Problem {
    let (new_aa_index, remapping, new_rooms, new_room_names) = remap_rooms(raw);

    let mut adj = Array2d::newu(Time::MAX, new_rooms.len(), new_rooms.len());
    for old in 0..remapping.len() {
        let new = remapping[old];
        if new == usize::MAX { continue; }

        let paths = shortest_paths(old, raw);
        for old2 in 0..remapping.len() {
            let new2 = remapping[old2];
            if new2 == usize::MAX { continue; }
            *adj.get_mutu((new, new2)) = paths[old2]
        }
    }

    println!("New adjacency:");
    for i in 0..new_rooms.len() {
        print!("  ");
        for j in 0..new_rooms.len() {
            print!("{:3} ", adj.getu((i,j)));
        }
        print!("\n");
    }

    Problem {
        num_rounds: num_rounds,
        total_flow_rate: raw.total_flow_rate,
        aa_index: new_aa_index,
        rooms: new_rooms,
        room_names: new_room_names,
        adjacency: adj,
    }
}


#[derive(Clone,Copy,Debug)]
struct Agent {
    target_room: usize,
    target_room_rate: FlowRate,
    arrival_time: Time,
}

#[derive(Clone,Copy,Debug)]
struct Solution<const N_AGENTS: usize> {
    agents: [Agent; N_AGENTS],
    current_time: Time,
    current_rate: FlowRate,
    pressure_released: FlowRate,
}

impl<const N_AGENTS: usize> Solution<N_AGENTS> {
    fn new(start_room: usize) -> Solution<N_AGENTS> {
        Solution {
            agents:
                [
                    Agent {
                        target_room: start_room,
                        target_room_rate: 0,
                        arrival_time: 0
                    };
                    N_AGENTS
                ],
            current_time: 0,
            current_rate: 0,
            pressure_released: 0,
        }
    }
}

fn valve_is_open<const N_AGENTS: usize>(
        valve: usize,
        open_valves_stacks: &mut [Vec<usize>; N_AGENTS])
    -> bool
{
    for stack in open_valves_stacks {
        if stack.contains(&valve) { return true; }
    }
    false
}

/// Performs a depth first search on the tunnel system. Returns the pressure released
fn depth_first_search<const N_AGENTS: usize>(
        mut best: FlowRate,
        solution: Solution<N_AGENTS>,
        open_valves_stacks: &mut [Vec<usize>; N_AGENTS],
        problem: &Problem)
    -> FlowRate
{
    // Which agent is first?
    let agent_i = {
        let mut best_agent = 0;
        let mut best_time = Time::MAX;
        for (i, agent) in solution.agents.iter().enumerate() {
            if agent.arrival_time < best_time {
                best_agent = i;
                best_time = agent.arrival_time;
            }
        }
        best_agent
    };

    let agent = &solution.agents[agent_i];
    
    // If we run out of rounds, then stop
    if agent.arrival_time >= problem.num_rounds {
        let dt = problem.num_rounds - solution.current_time;
        let pressure_released = solution.pressure_released + dt*solution.current_rate;
        return std::cmp::max(pressure_released, best);
    }

    // Advance time
    let dt = agent.arrival_time - solution.current_time;
    let solution = Solution {
        agents: solution.agents, // Just copy the agents
        current_time: agent.arrival_time,
        current_rate: solution.current_rate + agent.target_room_rate,
        pressure_released: solution.pressure_released + dt*solution.current_rate,
    };

    let remaining = problem.num_rounds - agent.arrival_time;
    // If we can't do better than the best, then stop
    if solution.pressure_released + remaining*problem.total_flow_rate < best {
        //return best;
    }

    // If we run out of valves to turn, stop
    if solution.current_rate == problem.total_flow_rate {
        let total_pressure  = solution.pressure_released + remaining*solution.current_rate;
        return std::cmp::max(total_pressure, best);
    }

    // What if we travelled and opened a valve?
    for new_room in 0..problem.rooms.len() {
        if valve_is_open(new_room, open_valves_stacks) {
            // Can't open already-openned valves
            continue;
        }

        let move_time = problem.adjacency.getu((agent.target_room, new_room));
        if solution.current_time + move_time + 1 >= problem.num_rounds {
            // We don't have time to go to this valve
            continue;
        }

        let mut new_agents = solution.agents;
        new_agents[agent_i] =
            Agent {
                target_room: new_room,
                target_room_rate: problem.rooms[new_room],
                arrival_time: agent.arrival_time + move_time + 1,
            };

        let new_solution = 
            Solution {
                agents: new_agents,
                ..solution
            };

        open_valves_stacks[agent_i].push(new_room);
        best = std::cmp::max(
            best,
            depth_first_search(
                best,
                new_solution,
                open_valves_stacks,
                problem));
        assert_eq!(open_valves_stacks[agent_i].pop(), Some(new_room));
    }
    
    // What if we stayed here and did nothing?
    let mut new_agents = solution.agents;
    new_agents[agent_i] =
        Agent {
            target_room: usize::MAX,
            target_room_rate: 0,
            arrival_time: Time::MAX,
        };

    let new_solution = 
        Solution {
            agents: new_agents,
            ..solution
        };

    best = std::cmp::max(
        best,
        depth_first_search(
            best,
            new_solution,
            open_valves_stacks,
            problem));

    best
}


fn solve<const N_AGENTS: usize>(input: &str, num_rounds: isize)
    -> FlowRate
    where [Vec<usize>; N_AGENTS]: Default
{
    let problem_raw = parse(input);
    let problem = reduce_problem(&problem_raw, num_rounds);

    let mut room_stacks = Default::default();
    
    let output = depth_first_search::<N_AGENTS>(
        0,
        Solution::new(problem.aa_index),
        &mut room_stacks,
        &problem);
    
    for room_stack in room_stacks {
        assert_eq!(room_stack.len(), 0);
    }

    output
}


pub fn run() {
    let input =
        std::fs::read_to_string("data/d16.txt")
        .expect("Failed to read input");
    println!("{}", solve::<1>(&input, 30));
    println!("{}", solve::<2>(&input, 26)); // 2514 is too low
}


#[cfg(test)]
mod tests {
    use super::*;
    
    const EXAMPLE: &'static str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_example() {
        assert_eq!(solve::<1>(EXAMPLE, 30), 1651);
    }
    #[test]
    fn test_example_p2() {
        assert_eq!(solve::<2>(EXAMPLE, 26), 1707);
    }
}
