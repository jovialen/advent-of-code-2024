use std::env;
use std::fs;
use std::error::Error;
use std::io;
use std::collections::HashMap;

fn read_input() -> io::Result<String> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return Err(io::Error::other("Missing input file path in args"));
    }
    let input_path = &args[1];
    fs::read_to_string(input_path)
}

fn update_is_valid(update: &Vec<u32>, dependencies: &HashMap<u32, Vec<u32>>) -> bool {
    let sorted = topological_sort(update, dependencies.clone());
    update.iter().zip(sorted.iter()).all(|(x, y)| x == y)
}

fn topological_sort(nodes: &Vec<u32>, mut edges: HashMap<u32, Vec<u32>>) -> Vec<u32> {
    edges.values_mut().for_each(|edges| edges.retain(|v| nodes.contains(v)));
    edges.retain(|k, _| nodes.contains(k));

    let mut no_incoming: Vec<u32> = nodes.iter()
        .filter(|node| edges.get(&node).is_none_or(|incoming| !incoming.iter().any(|d| nodes.contains(d))))
        .map(|node| *node)
        .collect();
    
    let mut sorted = Vec::new();

    while let Some(node) = no_incoming.pop() {
        sorted.push(node);

        for (other_node, other_edges) in &mut edges {
            if let Some(index) = other_edges.iter().position(|x| *x == node) {
                other_edges.remove(index);
                
                if !other_edges.iter().any(|d| nodes.contains(d)) {
                    no_incoming.push(*other_node);
                }
            }
        }
    }

    sorted.reverse();
    sorted
}

fn parse_dependencies(dependencies: &str) -> HashMap<u32, Vec<u32>> {
    dependencies.lines()
        .filter_map(|dependency| dependency.split_once("|"))
        .filter_map(|(x, y)| Some((x.parse::<u32>().ok()?, y.parse::<u32>().ok()?)))
        .fold(HashMap::new(), |mut map: HashMap<u32, Vec<u32>>, (x, y)| {
            if let Some(vec) = map.get_mut(&x) {
                vec.push(y);
            } else {
                map.insert(x, vec![y]);
            }
            map
        })
}

fn parse_updates(updates: &str) -> Vec<Vec<u32>> {
    updates.lines()
        .map(|update| update.split(",")
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = read_input().expect("Usage: solution <input file>");

    let (raw_dependencies, raw_updates) = data.split_once("\n\n")
        .expect("data does not match expected pattern dependencies <linebreak> updates");

    let dependencies = parse_dependencies(raw_dependencies);
    let updates = parse_updates(raw_updates);

    let sum: u32 = updates.iter()
        .filter(|update| !update_is_valid(&update, &dependencies))
        .map(|update| topological_sort(&update, dependencies.clone()))
        .map(|update| update[update.len() / 2])
        .sum();

    println!("{}", sum);
    
    Ok(())
}
