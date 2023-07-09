use crate::day::Day;
use std::fmt::Display;

pub struct Day20;

pub struct Info {
    numbers: Vec<i32>,
}

impl Day<Info> for Day20 {
    fn parse_file(&self, file_content: String) -> Info {
        Info {
            numbers: file_content.lines().map(|x| x.parse().unwrap()).collect(),
        }
    }

    fn part_1(&self, data: &Info) -> i64 {
        let mut list: Vec<Node<i64>> = Vec::with_capacity(data.numbers.len());
        let len = data.numbers.len() as i64;

        for (i, num) in data.numbers.iter().enumerate() {
            let i = i as i64;
            list.push(Node {
                value: *num as i64,
                left: (i - 1).rem_euclid(len) as usize,
                right: (i + 1).rem_euclid(len) as usize,
            });
        }

        let zero_node = list.iter().position(|x| x.value == 0).unwrap();

        for i in 0..list.len() {
            let num = list.get(i).unwrap().value;
            move_node(&mut list, i, num);
        }

        let coord_1k = get_nth_right(&list, zero_node, 1000);
        let coord_2k = get_nth_right(&list, coord_1k, 1000);
        let coord_3k = get_nth_right(&list, coord_2k, 1000);

        (list.get(coord_1k).unwrap().value
            + list.get(coord_2k).unwrap().value
            + list.get(coord_3k).unwrap().value) as i64
    }

    fn part_2(&self, data: &Info) -> i64 {
        let mut list: Vec<Node<i64>> = Vec::with_capacity(data.numbers.len());
        let len_64 = data.numbers.len() as i64;

        let mut true_list: Vec<i64> = Vec::with_capacity(data.numbers.len());

        const DECRYPTION_KEY: i64 = 811589153;
        for (i, num) in data.numbers.iter().enumerate() {
            let true_num = (*num as i64) * DECRYPTION_KEY;
            true_list.push(true_num);
            let simp_num = true_num % len_64;
            println!("{} -> {} -> {}", num, true_num, simp_num);
            let i = i as i64;
            list.push(Node {
                value: simp_num,
                left: (i - 1).rem_euclid(len_64) as usize,
                right: (i + 1).rem_euclid(len_64) as usize,
            });
        }

        let zero_node = list.iter().position(|x| x.value == 0).unwrap();
        const NUM_MIXES: usize = 2;
        print_true_linked(&true_list, &list, zero_node);
        print_linked(&list, 0);
        println!();

        for m in 0..NUM_MIXES {
            for i in 0..list.len() {
                let num = list.get(i).unwrap().value;
                move_node(&mut list, i, num)
            }
            print!("{}: ", m + 1);
            print_true_linked(&true_list, &list, zero_node);
        }

        let coord_1k = get_nth_right(&list, zero_node, 1000);
        let coord_2k = get_nth_right(&list, coord_1k, 1000);
        let coord_3k = get_nth_right(&list, coord_2k, 1000);

        unsafe {
            true_list.get_unchecked(coord_1k)
                + true_list.get_unchecked(coord_2k)
                + true_list.get_unchecked(coord_3k)
        }
    }
}

fn print_linked<T: Display>(list: &Vec<Node<T>>, start_node: usize) {
    print!("[");
    let s_node = list.get(start_node).unwrap();
    print!("{}", s_node.value);
    let mut cur_node = s_node.right;
    while cur_node != start_node {
        let node = list.get(cur_node).unwrap();
        print!(", {}", node.value);
        cur_node = node.right;
    }

    println!("]");
}

fn print_true_linked<TRaw: Display, TTrue: Display>(
    true_list: &Vec<TTrue>,
    list: &Vec<Node<TRaw>>,
    start_node: usize,
) {
    print!("[");
    let s_node = list.get(start_node).unwrap();
    print!("{}", true_list.get(start_node).unwrap());
    let mut cur_node = s_node.right;
    while cur_node != start_node {
        let node = list.get(cur_node).unwrap();
        print!(", {}", true_list.get(cur_node).unwrap());
        cur_node = node.right;
    }

    println!("]");
}

fn remove<T>(list: &mut Vec<Node<T>>, node_index: usize) {
    let node = list.get(node_index).unwrap();
    let left_index = node.left;
    let right_index = node.right;
    let left_node = list.get_mut(left_index).unwrap();
    left_node.right = right_index;
    let right_node = list.get_mut(right_index).unwrap();
    right_node.left = left_index;
}

fn insert_right<T>(list: &mut Vec<Node<T>>, node_to_insert: usize, node_index: usize) {
    let left_node = list.get_mut(node_index).unwrap();
    let right_node_index = left_node.right;
    left_node.right = node_to_insert;

    let node = list.get_mut(node_to_insert).unwrap();
    node.left = node_index;
    node.right = right_node_index;

    let right_node = list.get_mut(right_node_index).unwrap();
    right_node.left = node_to_insert;
}

fn insert_left<T>(list: &mut Vec<Node<T>>, node_to_insert: usize, node_index: usize) {
    let right_node = list.get_mut(node_index).unwrap();
    let left_node_index = right_node.left;
    right_node.left = node_to_insert;

    let node = list.get_mut(node_to_insert).unwrap();
    node.right = node_index;
    node.left = left_node_index;

    let left_node = list.get_mut(left_node_index).unwrap();
    left_node.right = node_to_insert;
}

fn move_node<T>(list: &mut Vec<Node<T>>, node_index: usize, x: i64) {
    if x == 0 {
        return;
    }

    remove(list, node_index);
    if x > 0 {
        insert_right(
            list,
            node_index,
            get_nth_right(list, node_index, x as usize),
        );
    } else {
        insert_left(
            list,
            node_index,
            get_nth_left(list, node_index, -x as usize),
        );
    }
}

fn get_nth_right<T>(list: &Vec<Node<T>>, mut node_index: usize, n: usize) -> usize {
    for i in 0..n {
        node_index = list.get(node_index).unwrap().right;
    }

    node_index
}

fn get_nth_left<T>(list: &Vec<Node<T>>, mut node_index: usize, n: usize) -> usize {
    for i in 0..n {
        node_index = list.get(node_index).unwrap().left;
    }

    node_index
}

struct Node<T> {
    value: T,
    left: usize,
    right: usize,
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - [{}, {}]", self.value, self.left, self.right)
    }
}
