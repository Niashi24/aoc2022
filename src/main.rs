use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io;
use regex::Regex;
// use pathfinding::directed::dfs;

fn main() {
    
    let info = parse_file_to_info("day16e.txt").unwrap();
    
    let mut memo = HashMap::new();
    let start = State {
        location: 0,
        time: 0,
        pressure: 0,
        open_valves: 0,
    };
    
    // let path = dfs::dfs(start, |x| successors(x, &info), |x| x.pressure == 1651).unwrap();
    // 
    // for state in path {
    //     println!("{}mn - #{} - {:08b} - {}atm", state.time, state.location, state.open_valves, state.pressure);
    // }
    use std::time::Instant;
    let now = Instant::now();
    
    println!("Part 1: {}", solution(start, &info, &mut memo));
    println!("Elapsed Time: {:.2?}", now.elapsed());
    
    println!("States visited: {}", memo.len());
    
    let final_states = memo.keys().find(|x| x.pressure == 1651).unwrap();
    println!("Final state: {}", final_states);
    
    // dbg!(memo.keys().take(20).collect::<Vec<&State>>());
}

fn parse_file_to_info(file: &str) -> io::Result<Info> {
    let file_content = fs::read_to_string(file)?;
    let lines: Vec<String> = file_content.lines().map(raw_line_to_inter).collect();
    let mut file_content = lines.join("\n");

    for (i, line) in lines.iter().enumerate() {
        file_content = file_content.replace(&line[..2], &i.to_string());
    }
    
    let valves: Vec<Valve> = file_content.split("\n").map(num_line_to_valve).collect();

    let mut usable_valves: u64 = 0;
    for (i, valve) in valves.iter().enumerate() {
        if valve.flow != 0 {
            usable_valves |= (1 << i);
        }
    }
    
    // println!("{}", file_content);

    Ok(Info {
        valves,
        usable_valves,
        limit: 30
    })
}

fn successors(state: &State, info: &Info) -> Vec<State> {
    if state.time == info.limit - 1 {
        return vec![];
    }
    
    let mut successors = Vec::new();

    if info.is_usable_valve(state.location)
        && !state.has_valve_open(state.location) {
        successors.push(state.step_and_open_valve(info, state.location));
    }
    
    for connection in &info.valves.get(state.location as usize).unwrap().connections {
        successors.push(state.step_and_move(info, connection.clone()));
        // result = result.max(solution(state.step_and_move(info, connection.clone()), info, memo));
    }
    
    successors
}

fn solution(state: State, info: &Info, memo: &mut HashMap<State, u16>) -> u16 {
    
    if let Some(result) = memo.get(&state).copied() {
        return result;
    }
    
    if state.time == info.limit {
        let result = state.pressure;
        memo.insert(state, result);
        // if memo.len() & 131071 == 131071 {
        //     println!("{}", memo.len());
        // }
        return result;
    }
    
    let mut result = 0;
    // Open current valve (only if != 0)
    if info.is_usable_valve(state.location)
        && !state.has_valve_open(state.location) {
        result = result.max(solution(state.step_and_open_valve(info, state.location), info, memo));
    }
    // Go to connecting valves
    for connection in &info.valves.get(state.location as usize).unwrap().connections {
        result = result.max(solution(state.step_and_move(info, connection.clone()), info, memo));
    }
    
    memo.insert(state, result);
    
    result
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

fn num_line_to_valve(str: &str) -> Valve {
    let spl: Vec<&str> = str.split(" ").collect();
    Valve {
        flow: spl[1].to_string().parse().unwrap(),
        connections: spl[2..].iter().map(|x| x.to_string().parse().unwrap()).collect(),
    }
}

struct Valve {
    flow: u16,
    connections: Vec<u8>
}

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
    
    #[inline]
    fn get_total_pressure(&self, info: &Info, valve_id: u8) -> u16 {
        info.valves.get(valve_id as usize).unwrap().flow * ((info.limit - self.time - 1) as u16)
    }

    #[inline]
    fn step_and_open_valve(&self, info: &Info, valve_id: u8) -> State {
        State {
            time: self.time + 1,
            pressure: self.pressure + self.get_total_pressure(info, valve_id),
            open_valves: self.open_valves | (1 << valve_id),
            ..*self
        }
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
    fn has_valve_open(&self, valve_id: u8) -> bool {
        (self.open_valves >> valve_id) & 1 == 1
    }
}

