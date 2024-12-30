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

fn update_is_valid(first: u32, rest: &[u32], dependencies: &HashMap<u32, Vec<u32>>) -> bool {
    if rest.is_empty() {
        return true;
    }

    if let Some(dependencies) = dependencies.get(&first) {
        for x in rest {
            if dependencies.contains(x) {
                return false;
            }
        }
    }

    update_is_valid(rest[0], &rest[1..], dependencies)
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = read_input().expect("Usage: solution <input file>");

    let (dependencies, updates) = data.split_once("\n\n").expect("data does not match expected pattern dependencies <linebreak> updates");

    let parsed_dependencies: HashMap<_, _> = dependencies.lines()
        .filter_map(|dependency| dependency.split_once("|"))
        .filter_map(|(x, y)| Some((x.parse::<u32>().ok()?, y.parse::<u32>().ok()?)))
        .fold(HashMap::new(), |mut map: HashMap<u32, Vec<u32>>, (x, y)| {
            if let Some(vec) = map.get_mut(&y) {
                vec.push(x);
            } else {
                map.insert(y, vec![x]);
            }
            map
        });

    let sum: u32 = updates.lines()
        .map(|update| update.split(",").filter_map(|x| x.parse::<u32>().ok()).collect::<Vec<_>>())
        .filter(|update| update_is_valid(update[0], &update[1..], &parsed_dependencies))
        .map(|update| update[update.len() / 2])
        .sum();

    println!("{}", sum);
    
    Ok(())
}