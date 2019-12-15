extern crate aoc;

use std::collections::VecDeque;

fn main() {
    let input = aoc::input!();
    let input = input.trim().split('\n').collect::<Vec<_>>();
    let orbits = Orbits::parse(&input);
    println!("Found {} nodes", orbits.items.len());
    println!("Part 1: {}", orbits.calculate_total_link_count());

    let path = orbits.find_fastest_path_between("YOU", "SAN");
    println!("{:?}", path);

    // we need to target santa's parent (-1), and the list includes both YOU (-1) and SAN (-1), so we subtract 3
    let moves_required = path.len() - 3;
    println!("Part 2: {}", moves_required);
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

    let orbits = Orbits::parse(&input);
    let count = orbits.calculate_total_link_count();

    assert_eq!(count, 42);
}

#[test]
fn test_orbit_path() {
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
K)YOU
I)SAN
"
    .trim()
    .split('\n')
    .collect::<Vec<_>>();

    let orbit = Orbits::parse(&input);
    let path = orbit.find_fastest_path_between("YOU", "SAN");
    println!("{:#?}", path);
    // we need to target santa's parent (-1), and the list includes both YOU (-1) and SAN (-1), so we subtract 3
    let moves_required = path.len() - 3;
    assert_eq!(moves_required, 4);
}

struct Orbits<'a> {
    items: Vec<OrbitInfo<'a>>,
}

struct OrbitInfo<'a> {
    name: &'a str,
    children: Vec<usize>,
    parents: Vec<usize>,
}

impl<'a> Orbits<'a> {
    pub fn parse(input: &'a [&'a str]) -> Self {
        let mut orbits = Orbits { items: Vec::new() };
        for line in input {
            let mut split = line.split(')');
            let left = split.next().unwrap();
            let right = split.next().unwrap();
            let left_index = orbits.find_orbit_by_name_or_insert(left);
            let right_index = orbits.find_orbit_by_name_or_insert(right);

            orbits.items[left_index].parents.push(right_index);
            orbits.items[right_index].children.push(left_index);
        }

        orbits
    }

    pub fn find_orbit_by_name(&self, name: &'a str) -> usize {
        self.items
            .iter()
            .position(|o| o.name == name)
            .unwrap_or_else(|| panic!("Could not find orbit of {:?}", name))
    }

    pub fn find_orbit_by_name_or_insert(&mut self, name: &'a str) -> usize {
        match self.items.iter().position(|o| o.name == name) {
            Some(i) => i,
            None => {
                self.items.push(OrbitInfo {
                    name,
                    children: Vec::new(),
                    parents: Vec::new(),
                });
                self.items.len() - 1
            }
        }
    }

    pub fn calculate_total_link_count(&self) -> usize {
        let mut count = 0;
        for i in 0..self.items.len() {
            count += self.link_count_of_index(i);
        }
        count
    }

    fn link_count_of_index(&self, index: usize) -> usize {
        let mut count = 0;
        for parent in &self.items[index].parents {
            count += 1 + self.link_count_of_index(*parent);
        }
        count
    }

    pub fn find_fastest_path_between(&self, start: &'a str, end: &'a str) -> Vec<&'a str> {
        let start_index = self.find_orbit_by_name(start);
        let end_index = self.find_orbit_by_name(end);
        let mut iteration_count = 0;

        let mut path_list = VecDeque::new();
        path_list.push_back(vec![start_index]);
        while let Some(path) = path_list.pop_front() {
            let orbit_index = *path.last().unwrap();
            let orbit = &self.items[orbit_index];

            for other in orbit.parents.iter().chain(orbit.children.iter()).cloned() {
                if other == end_index {
                    println!("Found path in {} iterations", iteration_count,);
                    let result = path
                        .into_iter()
                        .chain(Some(other))
                        .map(|i| self.items[i].name)
                        .collect();
                    return result;
                }
                iteration_count += 1;
                // Only add this new node to the path if it is not visited yet
                if !path.contains(&other) {
                    let mut new_path = path.clone();
                    new_path.push(other);
                    path_list.push_back(new_path);
                }
            }
        }

        panic!(
            "Could not find path between {:?} ({}) and {:?} ({})",
            start, start_index, end, end_index
        );
    }
}
