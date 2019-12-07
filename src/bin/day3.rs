extern crate aoc;

use std::num::NonZeroU32;

fn main() {
    let input = aoc::input!();

    let mut grid = Grid::default();
    let lines: (Vec<&str>, Vec<&str>) = {
        let mut lines = input.lines();
        (
            lines.next().unwrap().split(',').collect(),
            lines.next().unwrap().split(',').collect(),
        )
    };

    draw_lines(&mut grid, 1, &lines.0);
    let mut intersect = draw_lines(&mut grid, 2, &lines.1);
    intersect.sort_by_key(|p| p.point.0.abs() + p.point.1.abs());
    let first = intersect.iter().next().unwrap();

    println!(
        "Part 1 intersects at {:?} ({})",
        first,
        (first.point.0.abs() + first.point.1.abs())
    );

    intersect.sort_by_key(|p| p.line1_length + p.line2_length);
    let first = intersect.iter().next().unwrap();
    println!(
        "part 2 intersects at {:?} ({})",
        first,
        first.line1_length + first.line2_length
    );
}

#[test]
fn test_nearest_point_intersect() {
    let mut grid = Grid::default();
    draw_lines(&mut grid, 1, &["R8", "U5", "L5", "D3"]);
    let mut intersect = draw_lines(&mut grid, 2, &["U7", "R6", "D4", "L4"]);
    intersect.sort_by_key(|p| p.point.0.abs() + p.point.1.abs());
    let first = intersect.into_iter().next().unwrap();
    assert_eq!((3, 3), first.point);

    let mut grid = Grid::default();
    draw_lines(
        &mut grid,
        1,
        &["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
    );
    let mut intersect = draw_lines(
        &mut grid,
        2,
        &["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
    );
    intersect.sort_by_key(|p| p.point.0.abs() + p.point.1.abs());
    let first = intersect.into_iter().next().unwrap();
    assert_eq!((155, 4), first.point);

    let mut grid = Grid::default();
    draw_lines(
        &mut grid,
        1,
        &[
            "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
        ],
    );
    let mut intersect = draw_lines(
        &mut grid,
        2,
        &[
            "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
        ],
    );
    intersect.sort_by_key(|p| p.point.0.abs() + p.point.1.abs());
    let first = intersect.into_iter().next().unwrap();
    assert_eq!((124, 11), first.point);
}

#[test]
fn test_nearest_distance_intersect() {
    let mut grid = Grid::default();
    draw_lines(&mut grid, 1, &["R8", "U5", "L5", "D3"]);
    let mut intersect = draw_lines(&mut grid, 2, &["U7", "R6", "D4", "L4"]);
    intersect.sort_by_key(|p| p.line1_length + p.line2_length);
    let first = intersect.into_iter().next().unwrap();
    assert_eq!(30, first.line1_length + first.line2_length);

    let mut grid = Grid::default();
    draw_lines(
        &mut grid,
        1,
        &["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
    );
    let mut intersect = draw_lines(
        &mut grid,
        2,
        &["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
    );
    intersect.sort_by_key(|p| p.line1_length + p.line2_length);
    let first = intersect.into_iter().next().unwrap();
    assert_eq!(610, first.line1_length + first.line2_length);

    let mut grid = Grid::default();
    draw_lines(
        &mut grid,
        1,
        &[
            "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
        ],
    );
    let mut intersect = draw_lines(
        &mut grid,
        2,
        &[
            "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
        ],
    );
    intersect.sort_by_key(|p| p.line1_length + p.line2_length);
    let first = intersect.into_iter().next().unwrap();
    assert_eq!(410, first.line1_length + first.line2_length);
}

#[derive(Debug)]
struct DrawResult {
    point: (isize, isize),
    line1_length: usize,
    line2_length: usize,
}

fn draw_lines(grid: &mut Grid, line_num: usize, line: &[&str]) -> Vec<DrawResult> {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut distance: u32 = 0;
    let mut result = Vec::new();

    for instruction in line {
        let direction: char = instruction.chars().next().unwrap();
        let length = instruction[1..].parse::<usize>().unwrap();
        let diff_per_step = match direction {
            'U' => (0, 1),
            'R' => (1, 0),
            'D' => (0, -1),
            'L' => (-1, 0),
            _ => unreachable!(),
        };
        for _ in 0..length {
            x += diff_per_step.0;
            y += diff_per_step.1;
            distance += 1;
            let other_line_length = grid.add(line_num, distance, x, y);
            if let Some(other_line_length) = other_line_length {
                result.push(DrawResult {
                    point: (x, y),
                    line1_length: other_line_length.get() as usize,
                    line2_length: distance as usize,
                });
            }
        }
    }

    result
}

#[derive(Default)]
struct GridPoint {
    line1: Option<NonZeroU32>,
    line2: Option<NonZeroU32>,
}

#[derive(Default)]
struct Grid {
    topleft: Vec<Vec<GridPoint>>,
    topright: Vec<Vec<GridPoint>>,
    bottomleft: Vec<Vec<GridPoint>>,
    bottomright: Vec<Vec<GridPoint>>,
}

impl Grid {
    fn add(
        &mut self,
        line_num: usize,
        line_distance: u32,
        x: isize,
        y: isize,
    ) -> Option<NonZeroU32> {
        let area = self.get_area_mut(x, y);
        let x = x.abs() as usize;
        let y = y.abs() as usize;
        if x >= area.len() {
            area.resize_with(x + 1, Vec::new);
        }
        let row = &mut area[x];
        if y >= row.len() {
            row.resize_with(y + 1, GridPoint::default);
        }
        if line_num == 1 {
            row[y].line1 = NonZeroU32::new(line_distance);
            row[y].line2
        } else {
            row[y].line2 = NonZeroU32::new(line_distance);
            row[y].line1
        }
    }

    fn get_area_mut(&mut self, x: isize, y: isize) -> &mut Vec<Vec<GridPoint>> {
        match (x >= 0, y >= 0) {
            (true, true) => &mut self.topright,
            (false, true) => &mut self.topleft,
            (true, false) => &mut self.bottomright,
            (false, false) => &mut self.bottomleft,
        }
    }
}
