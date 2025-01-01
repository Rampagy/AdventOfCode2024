use std::fs;
use std::time::Instant;
use std::collections::{HashSet, HashMap};


#[allow(non_snake_case)]
fn main() {
    let contents: String = fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let mut now: Instant;
    let mut elapsed: std::time::Duration;

    now = Instant::now();
    let part1: u64 = part1(contents.clone());
    elapsed = now.elapsed();

    println!("part 1: {} ({:.2?})", part1, elapsed);

    now = Instant::now();
    let part2: String = part2(contents.clone());
    elapsed = now.elapsed();

    println!("part 2: {} ({:.2?})", part2, elapsed);
}


#[allow(non_snake_case)]
fn dfs(graph: &HashMap<String, HashSet<String>>, parent: String, original: &String, depth: u64, prev_starts_with_t: bool, 
        paths: &mut Vec<HashSet<String>>, mut came_from: HashMap<String, String>) -> ()
    {
    if depth > 0 {
        for child in graph.get(&(parent.clone())).unwrap() {
            if *child != parent && *child != *original {
                let starts_with_t: bool = if !prev_starts_with_t { child.starts_with('t') } else { true };
                came_from.insert(child.to_string(), parent.to_string());
                dfs(graph, child.to_string(), original, depth-1, starts_with_t, paths, came_from.clone());
            }
        }
    } else {
        // check if any of the children of parent are 'orignal'
        if graph.get(&parent).unwrap().contains(&original.clone()) && prev_starts_with_t {
            // potential solution (length of 3)
            // trace the path back
            let mut path: HashSet<String> = HashSet::default();
            let mut current: String = parent;
            path.insert(original.to_string());
            while current != original.to_string() {
                path.insert(current.clone());
                current = came_from.get(&current.to_string()).unwrap().clone().to_string();
            }

            let mut already_exists: bool = false;
            for found_path in paths.clone() {
                let mut count: u64 = 0;
                for point in path.clone() {
                    if found_path.contains(&point) {
                        count += 1;
                    }
                }

                if count >= path.len() as u64 {
                    already_exists = true;
                    break;
                }
            }

            // check to make sure this path doesn't already exist before adding it
            if !already_exists {
                // add the path to a mutably refrenced vector of hashsets
                paths.push(path);
            }
        }
    }

    return;
}


#[allow(non_snake_case)]
fn part1(contents: String) -> u64 {
    let mut networks: HashMap<String, HashSet<String>> = HashMap::default();

    for (_row_num, line) in contents.lines().enumerate() {
        let nodes: Vec<String> = line.split('-')
                                    .map(|x| x.to_string())
                                    .collect();

        if let Some(network) = networks.get_mut(nodes.first().unwrap()) {
            // if the first node is in the network, insert the last as a connection to the first
            network.insert(nodes.last().unwrap().to_string());

            if let Some(network1) = networks.get_mut(nodes.last().unwrap()) {
                // if the last node is in the network, insert the first as a connection to the last
                network1.insert(nodes.first().unwrap().to_string());
            }
        }

        if let Some(network) = networks.get_mut(nodes.last().unwrap()) {
            // if the last node is in the network, insert the first as a connection to the last
            network.insert(nodes.first().unwrap().to_string());

            if let Some(network1) = networks.get_mut(nodes.first().unwrap()) {
                // if the last node is in the network, insert the first as a connection to the last
                network1.insert(nodes.last().unwrap().to_string());
            }
        }

        if !networks.contains_key(nodes.first().unwrap()) {
            let mut last_set:  HashSet<String> = HashSet::new();
            last_set.insert(nodes.last().unwrap().to_string());
            networks.insert(nodes.first().unwrap().to_string(), last_set);
        }

        if !networks.contains_key(nodes.last().unwrap()) {
            let mut first_set: HashSet<String> = HashSet::new();
            first_set.insert(nodes.first().unwrap().to_string());
            networks.insert(nodes.last().unwrap().to_string(), first_set);
        }
    }

    // traverse graph and see if it can get back to the original parent at depth=3 (2 if zero indexed)
    let mut paths: Vec<HashSet<String>> = Vec::new();
    let came_from: HashMap<String, String> = HashMap::default();
    for k in networks.keys() {
        dfs(&networks, k.to_string(), k, 2, k.starts_with('t'), &mut paths, came_from.clone());
    }

    return paths.len() as u64;
}



#[allow(non_snake_case)]
fn part2(contents: String) -> String {
    let mut networks: HashSet<(String,String)> = HashSet::default();
    let mut computers: Vec<String> = Vec::new();

    for (_row_num, line) in contents.lines().enumerate() {
        let nodes: Vec<String> = line.split('-')
                                    .map(|x| x.to_string())
                                    .collect();

        // push the pair into the networks set (bidirectional, both directions)
        networks.insert((nodes.first().unwrap().to_string(), nodes.last().unwrap().to_string()));
        networks.insert((nodes.last().unwrap().to_string(), nodes.first().unwrap().to_string()));

        for node in nodes {
            if !computers.contains(&node) {
                computers.push(node);
            }
        }
    }

    // initialize a groups that contains each computer to start
    let mut groups: Vec<Vec<String>> = Vec::new();
    for c in computers.clone() {
        groups.push(vec![c]);
    }

    // go thorugh each group and see if the computer has a connection to every other computer in the group
    // if so add it to the group
    for (_computer_idx, computer) in computers.clone().iter().enumerate() {
        let mut need_to_create_new_group: bool = true;
        for (group_idx, group) in groups.clone().iter().enumerate() {
            if !group.contains(&computer) {
                // if the group does not contain the computer check to see if it should be added
                let mut add_to_group: bool = true;
                for group_computer in group.clone() {
                    if !networks.contains(&(group_computer.clone(), computer.clone())) && !networks.contains(&(computer.clone(), group_computer.clone())) {
                        // set flag and break, it can't be added to the group
                        add_to_group = false;
                        break;
                    }
                }

                if add_to_group {
                    groups[group_idx].push(computer.clone());
                    need_to_create_new_group = false;
                }
            } else {
                need_to_create_new_group = false;
            }
        }

        if need_to_create_new_group {
            groups.push(vec![computer.clone()]);
        }
    }

    let mut biggest_group: Vec<String> = Vec::new();
    for group in groups {
        if group.len() > biggest_group.len() {
            biggest_group = group;
        }
    }

    // sort alphabetically
    biggest_group.sort();

    return biggest_group.join(",");
}


#[cfg(test)] #[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_part1a() {
        let contents: String = fs::read_to_string("src/test1a.txt").expect("Should have been able to read the file");
        assert_eq!(part1(contents.clone()), 7);
    }

    #[test]
    fn test_part2a() {
        let contents: String = fs::read_to_string("src/test2a.txt").expect("Should have been able to read the file");
        assert_eq!(part2(contents.clone()), "co,de,ka,ta".to_string());
    }
}