use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    // parse input - we assume it fits into memory
    let input_raw = read_to_string("input/task.txt").expect("File not found!");

    // assume file adheres to format
    // instruction string is first line
    let instruct = input_raw.lines().next().expect("File empty.");
    let instruct_len = instruct.chars().count();

    // adjacency map to represent graph
    let mut adj: HashMap<String, (String, String)> = HashMap::new();

    for line in input_raw.lines().skip(2) {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        // Check if the substrings match the expected format
        assert!(tokens.len() == 4 && tokens[1] == "=");
        // Extract the node and the two adjacent nodes
        let node = tokens[0];
        let left = tokens[2].trim_matches(',').trim_matches('(');
        let right = tokens[3].trim_matches(')');

        adj.insert(node.to_string(), (left.to_string(), right.to_string()));
    }

    let mut step = 0;
    let mut cur = "AAA";
    let goal = "ZZZ";

    while cur != goal {
        let (left, right) = adj.get(cur).expect("File malformatted");

        match instruct
            .chars()
            .nth(step % instruct_len)
            .expect("Has to exist due to modulo!")
        {
            'L' => cur = left,
            'R' => cur = right,
            _ => panic!("File malformatted!"),
        }
        step += 1;
    }

    println!("TOOK {} STEPS", step);
}
