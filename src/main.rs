use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io;
use regex::Regex;
use pathfinding::directed::bfs;
use pathfinding::directed::astar;

fn main() {
    
    let info = parse_file_to_info("input.txt").unwrap();
    let info = create_valve_info(info);
    
    // dbg!(&info);
    
    let mut memo = HashMap::new();
    let start = State {
        location: 0,
        time: 0,
        pressure: 0,
        open_valves: 0,
    };
    
    use std::time::Instant;
    let now = Instant::now();
    
    let sol = solution_2(start, &info, &mut memo);
    let elapsed = now.elapsed();
    
    println!("Part 1: {}", sol);
    println!("Elapsed Time: {:.2?}", elapsed);
    
    println!("States visited: {}", memo.len());
    
    let final_states = memo.keys().find(|x| x.pressure == sol).unwrap();
    println!("Final state: {}", final_states);
    let successors = |state: &State| {
        let mut vec = Vec::new();

        if state.can_open_valve(&info, state.location) {
            vec.push(state.step_and_open_valve_2(&info, state.location));
        }

        // Go to connecting valves
        for i in 1..info.valves.len() {
            let i = i as u8;
            if state.can_move_to(&info, i) {
                vec.push(state.step_and_move_2(&info, i));
            }
        }

        vec
    };
    
    // Not needed for solution but used for debugging
    let path = bfs::bfs(
        &State::new(),
        successors,
        |x| {
            x.pressure == sol
        }
    );
    dbg!(path);
}

fn create_valve_info(info: Info) -> ValveInfo {
    let valve_connections = create_graph(&info);  // create the graph of the 
    let limit = info.limit;
    let usable_valves = info.usable_valves.clone();
    let mut valves = info.valves;
    valves.retain(|x| {  // only retain valves with flow rate > 0 and the start position
        (usable_valves >> x.valve_id) & 1 == 1
            || x.valve_id == 0
    });
    
    ValveInfo {
        valve_connections,
        valves,
        limit
    }
}

fn solution_2(state: State, info: &ValveInfo, memo: &mut HashMap<State, u16>) -> u16 {
    if let Some(result) = memo.get(&state).copied() {
        return result;
    }

    let mut result = state.pressure;
    // Open current valve
    if state.can_open_valve(info, state.location) { 
        result = result.max(solution_2(state.step_and_open_valve_2(info, state.location), info, memo));
    }
    
    // Go to connecting valves
    for i in 1..info.valves.len() {
        let i = i as u8;
        if state.can_move_to(info, i) {
            result = result.max(solution_2(state.step_and_move_2(info, i), info, memo));
        }
    }

    memo.insert(state, result);

    result
}

fn solution(state: State, info: &Info, memo: &mut HashMap<State, u16>) -> u16 {
    
    if let Some(result) = memo.get(&state).copied() {
        return result;
    }
    
    if state.time == info.limit {
        let result = state.pressure;
        memo.insert(state, result);
        return result;
    }
    
    let mut result = 0;
    // Open current valve
    if info.is_usable_valve(state.location)  // only if valve flow != 0
        && !state.has_valve_open(state.location) {  // only if hasn't already opened the valve
        result = result.max(solution(state.step_and_open_valve(info, state.location), info, memo));
    }
    // Go to connecting valves
    for connection in &info.valves.get(state.location as usize).unwrap().connections {
        result = result.max(solution(state.step_and_move(info, connection.clone()), info, memo));
    }
    
    memo.insert(state, result);
    
    result
}

// Parses the file
fn parse_file_to_info(file: &str) -> io::Result<Info> {
    let file_content = fs::read_to_string(file)?;
    let lines: Vec<String> = file_content.lines().map(raw_line_to_inter).collect();
    let mut file_content = lines.join("\n");

    for (i, line) in lines.iter().enumerate() {
        file_content = file_content.replace(&line[..2], &i.to_string());
    }
    
    // println!("{}", file_content);

    let valves: Vec<Valve> = file_content.split("\n").map(num_line_to_valve).collect();

    let mut usable_valves: u64 = 0;
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
    
    // dbg!(str);
    
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

#[derive(Debug)]
struct Valve {
    valve_id: u8,
    flow: u16,
    connections: Vec<u8>
}

#[derive(Debug)]
struct Info {
    valves: Vec<Valve>,
    usable_valves: u64,
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
    location: u8,
    time: u8,
    pressure: u16,
    open_valves: u64,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}mn - #{} - {:08b} - {}atm", self.time, self.location, self.open_valves, self.pressure)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}mn - #{} - {:08b} - {}atm", self.time, self.location, self.open_valves, self.pressure)
    }
}

impl State {
    fn new() -> State {
        State {
            location: 0,
            time: 0,
            pressure: 0,
            open_valves: 0,
        }
    }
    
    #[inline]
    fn get_total_pressure(&self, info: &Info, valve_id: u8) -> u16 {
        info.valves.get(valve_id as usize).unwrap().flow * ((info.limit - self.time - 1) as u16)
    }
    
    #[inline]
    fn get_total_pressure_2(&self, info: &ValveInfo, valve_id: u8) -> u16 {
        info.get_total_pressure_at_time(self.time, valve_id)
        // info.valves.get(valve_id as usize).unwrap().flow * ((info.limit - self.time - 1) as u16)
    }
    
    #[inline]
    fn step_and_open_valve_2(&self, info: &ValveInfo, valve_id: u8) -> State {
        // let total_pressure = self.get_total_pressure_2(info, valve_id);
        let state =
        State {
            time: self.time + 1,
            pressure: self.pressure + self.get_total_pressure_2(info, valve_id),
            open_valves: self.open_valves | (1 << valve_id),
            ..*self
        };

        // println!("Open before and after: \n{}\n{}", self, state);
        // println!("Pressure gained: {}", total_pressure);
        state
    }
    
    fn move_and_open(&self, info: &ValveInfo, valve_id: u8) -> State {
        let move_cost = info.get_move_cost(self.location, valve_id);
        State {
            time: self.time + move_cost + 1,
            location: valve_id,
            pressure: self.pressure + info.get_total_pressure_at_time(self.time + move_cost, valve_id),  //self.get_total_pressure_2(info, valve_id)
            open_valves: self.open_valves | (1 << valve_id)
        }
    }

    #[inline]
    fn step_and_open_valve(&self, info: &Info, valve_id: u8) -> State {
        let state =
        State {
            time: self.time + 1,
            pressure: self.pressure + self.get_total_pressure(info, valve_id),
            open_valves: self.open_valves | (1 << valve_id),
            ..*self
        };

        println!("Open before and after: \n{}\n{}", self, state);
        
        state
    }
    
    #[inline]
    fn step_and_move(&self, _: &Info, valve_id: u8) -> State {
        State {
            time: self.time + 1,
            location: valve_id,
            ..*self
        }
    }
    
    #[inline]
    fn step_and_move_2(&self, info: &ValveInfo, valve_id: u8) -> State {
        let state = 
        State {
            time: self.time + info.get_move_cost(self.location, valve_id),
            location: valve_id,
            ..*self
        };
        
        // println!("Move before and after: \n{}\n{}", self, state);
        
        state
    }
    
    #[inline]
    fn has_valve_open(&self, valve_id: u8) -> bool {
        self.open_valves >> valve_id & 1 == 1
    }
    
    #[inline]
    fn can_open_valve(&self, _: &ValveInfo, valve_id: u8) -> bool {
        valve_id != 0
            && !self.has_valve_open(valve_id)
    }
    
    #[inline]
    fn can_move_to(&self, info: &ValveInfo, valve_id: u8) -> bool {
        self.location != valve_id
            && !self.has_valve_open(valve_id)
            && self.time + info.get_move_cost(self.location, valve_id) + 1 < info.limit // +1 because has to spend a turn to open
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