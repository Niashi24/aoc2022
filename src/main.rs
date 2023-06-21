use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io;
use regex::Regex;
use pathfinding::directed::bfs;

fn main() {
    // create info struct from file
    let info = parse_file_to_info("day16.txt").unwrap();
    dbg!(&info.valves);
    println!("{:08b}", info.usable_valves);
    // create optimized valve info struct from info struct
    let info = create_valve_info(info);
    
    use std::time::Instant;
    // Start timer
    let now = Instant::now();
    
    let mut memo = HashMap::new();
    // Do part 1
    let sol_1 = part_1(State::new(), &info, &mut memo);
    
    // End timer
    let elapsed = now.elapsed();
    
    println!("Part 1: {}", sol_1);
    println!("Elapsed Time: {:.2?}", elapsed);
    
    println!("States visited: {}", memo.len());
    
    // Not needed for solution but used for debugging
    // Get path from start to end state
    let path = bfs::bfs(
        &State::new(),
        |state: &State| {
            let mut vec = Vec::new();

            // Go to and open connecting valves
            for i in 1..info.valves.len() {
                let i = i as u8;
                if state.can_move_to(&info, i) {
                    vec.push(state.move_and_open(&info, i));
                }
            }

            vec
        },
        |x| {  // Stop when hit pressure matching target
            x.pressure == sol_1
        }
    );
    dbg!(path);
}

fn part_1(state: State, info: &ValveInfo, memo: &mut HashMap<State, u16>) -> u16 {
    // already visited this state
    if let Some(result) = memo.get(&state).copied() {
        return result;
    }

    let mut result = state.pressure;
    
    // Go to connecting valves (depth first search)
    for i in 1..info.valves.len() {
        let i = i as u8;
        if state.can_move_to(info, i) {
            result = result.max(part_1(state.move_and_open(info, i), info, memo));
        }
    }

    // all possible connecting valves have been opened
    memo.insert(state, result);

    result
}

// Parses the file
fn parse_file_to_info(file: &str) -> io::Result<Info> {
    let file_content = fs::read_to_string(file)?;
    // get rid of unneeded info in each line
    let lines: Vec<String> = file_content.lines().map(raw_line_to_inter).collect();
    let mut file_content = lines.join("\n");

    // Find and replace valve id's with numbers
    for (i, line) in lines.iter().enumerate() {
        file_content = file_content.replace(&line[..2], &i.to_string());
    }

    let valves: Vec<Valve> = file_content.split("\n").map(num_line_to_valve).collect();
    
    let mut usable_valves = 0;
    for (i, valve) in valves.iter().enumerate() {
        if valve.flow != 0 {
            usable_valves |= 1 << i;
        }
    }

    Ok(Info {
        valves,
        usable_valves,
        limit: 30
    })
}
// Removes the surrounding stuff
// "Valve FY has flow rate=17; tunnels lead to valves GG, KJ"
// to "FY 17 GG KJ"
fn raw_line_to_inter(str: &str) -> String {
    let pattern = r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)";
    let re = Regex::new(pattern).unwrap();
    
    let captures = re.captures(str).unwrap();
    
    let valve = captures.get(1).unwrap().as_str();
    let flow = captures.get(2).unwrap().as_str();
    let connections: String = captures.get(3).unwrap().as_str().split(", ").collect::<Vec<&str>>().join(" ");
    
    format!("{} {} {}", valve, flow, connections)
}
// Parses a num line to a Valve
// "4 20 44 32" becomes a Valve with 20 flow and connections to valves 44 and 32
fn num_line_to_valve(str: &str) -> Valve {
    let spl: Vec<&str> = str.split(" ").collect();
    Valve {
        valve_id: spl[0].to_string().parse().unwrap(),
        flow: spl[1].to_string().parse().unwrap(),
        connections: spl[2..].iter().map(|x| x.to_string().parse().unwrap()).collect(),
    }
}

fn create_valve_info(info: Info) -> ValveInfo {
    let valve_connections = create_graph(&info);  // create the graph with time b/w valves
    let limit = info.limit;
    let usable_valves = info.usable_valves;
    let mut valves = info.valves;
    valves.retain(|x| {
        (usable_valves >> x.valve_id) & 1 == 1  // with flow rate > 0
            || x.valve_id == 0                  // start position
    });

    ValveInfo {
        valve_connections,
        valves,
        limit
    }
}

fn create_graph(info: &Info) -> Vec<Vec<u8>> {
    let mut graph: Vec<Vec<u8>> = Vec::new();
    for i in 0..info.valves.len() {
        let i = i as u8;
        if i != 0 && !info.is_usable_valve(i) {
            continue;
        }

        let mut start_to_end = Vec::new();
        for j in 0..info.valves.len() {
            let j = j as u8;
            if j != 0 && !info.is_usable_valve(j) {
                continue;
            }
            if i == j {
                start_to_end.push(0);
            } else {
                start_to_end.push(bfs::bfs(
                    &i,
                    |x| {
                        let mut neighbors = Vec::new();

                        for valve in &info.valves.get(*x as usize).unwrap().connections {
                            neighbors.push(valve.clone());
                        }

                        neighbors
                    },
                    |x| *x == j,
                ).unwrap().len() as u8 - 1);
            }
        }
        graph.push(start_to_end);
    }

    graph
}

struct Valve {
    valve_id: u8,
    flow: u16,
    connections: Vec<u8>
}

impl std::fmt::Debug for Valve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{} - {}psi/mn - [{}]", self.valve_id, self.flow, self.connections.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "))
    }
}

#[derive(Debug)]
struct Info {
    valves: Vec<Valve>,
    usable_valves: u128,
    limit: u8
}

impl Info {
    #[inline]
    fn is_usable_valve(&self, valve_id: u8) -> bool {
        (self.usable_valves >> valve_id) & 1 == 1
    }
}

#[derive(Debug)]
struct ValveInfo {
    valve_connections: Vec<Vec<u8>>,
    valves: Vec<Valve>,
    limit: u8
}

impl ValveInfo {
    #[inline]
    fn get_move_cost(&self, from: u8, to: u8) -> u8 {
        self.valve_connections.get(from as usize).unwrap().get(to as usize).unwrap().clone()
    }
    
    #[inline]
    fn get_total_pressure_at_time(&self, time: u8, valve_id: u8) -> u16 {
        self.valves.get(valve_id as usize).unwrap().flow * ((self.limit - time - 1) as u16)
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    location: u8,       // current valve location
    time: u8,           // time of state
    pressure: u16,      // accumulated lifetime pressure of open valves
    open_valves: u128,  // bit mask of open valves
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}mn - #{} - {:08b} - {}psi", self.time, self.location, self.open_valves, self.pressure)
    }
}

impl State {
    #[inline]
    fn new() -> State {
        State {
            location: 0,
            time: 0,
            pressure: 0,
            open_valves: 0,
        }
    }
    
    #[inline]
    fn move_and_open(&self, info: &ValveInfo, valve_id: u8) -> State {
        let move_cost = info.get_move_cost(self.location, valve_id);
        State {
            time: self.time + move_cost + 1,  // time to move + 1 to open valve
            location: valve_id,
            pressure: self.pressure + info.get_total_pressure_at_time(self.time + move_cost, valve_id),
            open_valves: self.open_valves | (1 << valve_id)
        }
    }
    
    #[inline]
    fn has_valve_open(&self, valve_id: u8) -> bool {
        self.open_valves >> valve_id & 1 == 1
    }
    
    #[inline]
    fn can_move_to(&self, info: &ValveInfo, valve_id: u8) -> bool {
        self.location != valve_id
            && !self.has_valve_open(valve_id)
            && self.time + info.get_move_cost(self.location, valve_id) + 1 < info.limit // +1 because has to spend a turn to open
    }
}
