use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let file_content = fs::read_to_string("day16e.txt")?;
    
    use std::time::Instant;
    
    let now = Instant::now();
    // create optimized valve info struct from file content
    let mut info = parse::create_valve_info(file_content);
    // stop timer
    let elapsed = now.elapsed();
    println!("Parsed file into ValveInfo struct");
    println!("Elapsed Time: {:.2?}", elapsed);
    println!();
    
    let now = Instant::now();
    // Do part 1
    let sol_1 = part_1::part_1(part_1::State::new(), &info);
    // stop timer
    let elapsed = now.elapsed();
    
    println!("Part 1: {}", sol_1);
    println!("Elapsed Time: {:.2?}", elapsed);
    println!();

    let now = Instant::now();
    info.limit = 26;
    // Do part 1
    let sol_2 = part_2::part_2(part_2::State::new(), &info);
    // stop timer
    let elapsed = now.elapsed();

    println!("Part 2: {}", sol_2);
    println!("Elapsed Time: {:.2?}", elapsed);
    
    Ok(())
}

mod parse {
    use pathfinding::directed::bfs;
    use regex::Regex;
    use crate::{Valve, ValveInfo};

    pub fn create_valve_info(file: String) -> ValveInfo {
        let info = parse_file_to_info(file);

        let valve_connections = create_graph(&info);  // create the graph with time b/w valves
        let limit = info.limit;
        let usable_valves = info.usable_valves;
        let mut valves = info.valves;
        valves.retain(|x| {
            (usable_valves >> x.valve_id) & 1 == 1  // with flow rate > 0
                || x.valve_id == 0                  // start position
        });
        // Unoptimized struct for each valve
        ValveInfo {
            valve_connections,
            valves,
            limit
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

    // Parses the file
    fn parse_file_to_info(file: String) -> Info {
        let mut file_content = file;

        // Code assumed the first line in the file was the starting point when it's actually Valve AA
        // More work to rewrite the entire program so I just swap the lines
        let mut aa_line_index = 0;
        for (i, line) in file_content.lines().enumerate() {
            if &line[..8] == "Valve AA" {
                aa_line_index = i;
                break;
            }
        }
        file_content = swap_lines(&file_content, 0, aa_line_index);

        // get rid of unneeded info in each line
        let lines: Vec<String> = file_content.lines().map(raw_line_to_inter).collect();
        file_content = lines.join("\n");

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

        Info {
            valves,
            usable_valves,
            limit: 30
        }
    }

    fn swap_lines(string: &str, i: usize, n: usize) -> String {
        let mut lines: Vec<&str> = string.lines().collect();

        if i >= lines.len() || n >= lines.len() {
            return String::from(string); // Return original string if line indices are out of bounds
        }

        let temp = lines[i];
        lines[i] = lines[n];
        lines[n] = temp;

        lines.join("\n")
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

}

mod part_2 {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};
    use crate::ValveInfo;

    #[derive(Clone, Eq, PartialEq, Hash)]
    pub struct State {
        p_location: u8,
        e_location: u8,
        time: u8,
        pressure: u16,
        open_valves: u128,
        p_timer: u8,
        e_timer: u8,
    }
    
    impl Display for State {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}mn - P: #{} {}, E: #{} {} - {}psi - {:.08b}", 
                   self.time, 
                   self.p_location, self.p_timer,
                   self.e_location, self.e_timer,
                   self.pressure, 
                   self.open_valves)
        }
    }
    
    impl State {
        pub fn new() -> State {
            State {
                p_location: 0,
                e_location: 0,
                time: 0,
                pressure: 0,
                open_valves: 0,
                p_timer: 0,
                e_timer: 0,
            }
        }

        #[inline]
        fn has_valve_open(&self, valve_id: u8) -> bool {
            self.open_valves >> valve_id & 1 == 1
        }
        #[inline]
        pub fn player_can_move(&self, info: &ValveInfo, valve_id: u8) -> bool {
            self.p_location != valve_id
                && !self.has_valve_open(valve_id)
                && self.time + info.get_move_cost(self.p_location, valve_id) + 1 < info.limit
        }
        #[inline]
        pub fn elephant_can_move(&self, info: &ValveInfo, valve_id: u8) -> bool {
            self.e_location != valve_id
                && !self.has_valve_open(valve_id)
                && self.time + info.get_move_cost(self.e_location, valve_id) + 1 < info.limit
        }
        
        pub fn wait(&self) -> State {  // need to wait one minute when opening a valve
            State {
                time: self.time + 1,
                p_timer: self.p_timer - 1,
                e_timer: self.e_timer - 1,
                ..*self
            }
        }
        
        pub fn move_player(&self, info: &ValveInfo, valve_id: u8) -> State {
            let move_cost = info.get_move_cost(self.p_location, valve_id);
            let elapsed_time = move_cost.min(self.e_timer);
            
            State {
                p_location: valve_id,
                e_location: self.e_location,
                time: self.time + elapsed_time,
                pressure: self.pressure + info.get_total_pressure_at_time(self.time + move_cost, valve_id),
                open_valves: self.open_valves | (1 << valve_id),
                p_timer: move_cost - elapsed_time + 1,
                e_timer: self.e_timer - elapsed_time,
            }
        }
        
        pub fn move_elephant(&self, info: &ValveInfo, valve_id: u8) -> State {
            let move_cost = info.get_move_cost(self.e_location, valve_id);
            let elapsed_time = move_cost.min(self.p_timer);
            
            State {
                p_location: self.p_location,
                e_location: valve_id,
                time: self.time + elapsed_time,
                pressure: self.pressure + info.get_total_pressure_at_time(self.time + move_cost, valve_id),
                open_valves: self.open_valves | (1 << valve_id),
                p_timer: self.p_timer - elapsed_time,
                e_timer: move_cost - elapsed_time + 1,
            }
        }
        
        pub fn move_player_and_elephant(&self, info: &ValveInfo, p_id: u8, e_id: u8) -> State {
            let p_cost = info.get_move_cost(self.p_location, p_id);
            let e_cost = info.get_move_cost(self.e_location, e_id);
            
            let elapsed_time = p_cost.min(e_cost);
            
            State {
                p_location: p_id,
                e_location: e_id,
                time: self.time + elapsed_time,
                pressure: self.pressure +
                    info.get_total_pressure_at_time(self.time + p_cost, p_id) +
                    info.get_total_pressure_at_time(self.time + e_cost, e_id),
                open_valves: self.open_valves | (1 << p_id) | (1 << e_id),
                p_timer: p_cost - elapsed_time + 1,
                e_timer: e_cost - elapsed_time + 1,
            }
        }
    }
    
    pub fn part_2(
        state: State,
        info: &ValveInfo
    ) -> u16 {
        
        let mut result = state.pressure;
        
        match (state.p_timer, state.e_timer) {
            (0, 0) => {
                // Have both player and elephant move
                let mut both_moved = false;
                for i in 1..info.valves.len() {  // player destinations
                    let i = i as u8;
                    if !state.player_can_move(info, i) {
                        continue;
                    }
                    for j in 1..info.valves.len() {  // elephant destinations
                        let j = j as u8;
                        if !state.elephant_can_move(info, j) || i == j {
                            continue;
                        }
                        
                        both_moved = true;
                        result = result.max(part_2(state.move_player_and_elephant(info, i, j), info));
                    }
                }
                // what if player/elephant can't move but elephant/player can?
                if !both_moved {
                    // Have only player move
                    for i in 1..info.valves.len() {
                        let i = i as u8;

                        if state.player_can_move(info, i) {
                            result = result.max(part_2(state.move_player(info, i), info));
                        }
                    }

                    // Have only elephant move
                    for i in 1..info.valves.len() {
                        let i = i as u8;

                        if state.elephant_can_move(info, i) {
                            result = result.max(part_2(state.move_elephant(info, i), info));
                        }
                    }
                }
            },
            (0, _) => {
                // Have only player move
                
                for i in 1..info.valves.len() {
                    let i = i as u8;
                    
                    if state.player_can_move(info, i) {
                        result = result.max(part_2(state.move_player(info, i), info));
                    }
                }
            },
            (_, 0) => {
                // Have only elephant move
                for i in 1..info.valves.len() {
                    let i = i as u8;
                    
                    if state.elephant_can_move(info, i) {
                        result = result.max(part_2(state.move_elephant(info, i), info));
                    }
                }
            },
            // (1, 1) => {  // both currently opening valves
            //     
            // },
            (1, _) => {
                result = result.max(part_2(state.wait(), info));
            },
            (_, 1) => {
                result = result.max(part_2(state.wait(), info));
            }
            (_, _) => {
                panic!("Should never have both not at 0 or 1");
            }
        };
        
        result
    }
}

mod part_1 {
    use std::fmt::Formatter;
    use crate::ValveInfo;

    #[derive(Clone, Eq, PartialEq, Hash)]
    pub struct State {
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
        pub fn new() -> State {
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

    pub fn part_1(state: State, info: &ValveInfo) -> u16 {

        let mut result = state.pressure;

        // Go to connecting valves and open them (depth first search)
        for i in 1..info.valves.len() {
            let i = i as u8;
            if state.can_move_to(info, i) {
                result = result.max(part_1(state.move_and_open(info, i), info));
            }
        }

        // all possible connecting valves have been opened

        result
    }
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
pub struct ValveInfo {
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
