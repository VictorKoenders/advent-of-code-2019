extern crate aoc;

use std::collections::HashMap;

fn main() {
    let input = aoc::input!();
    let input = input.trim().split('\n').collect::<Vec<_>>();

    let orbits = get_orbit_map(&input);
    let mut count = 0;
    for (parent, children) in &orbits {
        for _ in children {
            count += 1 + count_parents(parent, &orbits);
        }
    }

    println!("Part 1: {}", count);
}

#[test]
fn test_count_orbits() {
    let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
"
    .trim()
    .split('\n')
    .collect::<Vec<_>>();

    let orbits = get_orbit_map(&input);
    println!("{:?}", orbits);
    let mut count = 0;
    for (parent, children) in &orbits {
        for _ in children {
            count += 1 + count_parents(parent, &orbits);
        }
    }
    assert_eq!(42, count);
}

fn count_parents(child: &str, orbits: &HashMap<&str, Vec<&str>>) -> usize {
    if let Some(parent) = orbits
        .iter()
        .find(|(_, children)| children.contains(&child))
        .map(|(key, _)| key)
    {
        1 + count_parents(parent, orbits)
    } else {
        0
    }
}

fn get_orbit_map<'a>(input: &'a [&'a str]) -> HashMap<&'a str, Vec<&'a str>> {
    let mut result = HashMap::new();
    for line in input {
        let mut split = line.split(')');
        let left = split.next().unwrap();
        let right = split.next().unwrap();

        result.entry(left).or_insert(Vec::new()).push(right);
    }

    result
}
