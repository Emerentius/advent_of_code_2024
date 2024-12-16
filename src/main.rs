use itertools::Itertools;
use nalgebra::{Matrix2, Vector2};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};
use structopt::StructOpt;

fn parse_num(string: &str) -> u64 {
    string.parse().unwrap()
}

fn day1(part: Part) {
    let location_ids = include_str!("day1_input.txt");
    let (mut left, mut right) = location_ids
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("   ").unwrap();
            (left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap())
        })
        .collect::<(Vec<_>, Vec<_>)>();

    match part {
        Part::One => {
            left.sort();
            right.sort();
            let difference = left
                .iter()
                .zip(&right)
                // the problem description is actually missing what we need to do if the difference
                // is negative.
                .map(|(l, r)| (r - l).abs())
                .sum::<i64>();
            println!("{}", difference)
        }
        Part::Two => {
            fn count_occurences(list: Vec<i64>) -> HashMap<i64, i64> {
                let mut counts = HashMap::new();
                for n in list {
                    *counts.entry(n).or_insert(0) += 1;
                }
                counts
            }

            let left_counts = count_occurences(left);
            let right_counts = count_occurences(right);
            let similarity_score = left_counts
                .into_iter()
                .map(|(num, count)| num * count * right_counts.get(&num).cloned().unwrap_or(0))
                .sum::<i64>();
            println!("{similarity_score}");
        }
    }
}

fn day2(part: Part) {
    let reports = include_str!("day2_input.txt");
    let reports: Vec<Vec<_>> = reports
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    fn report_is_safe(report: &[i64]) -> bool {
        let mut monotonically_increasing = true;
        let mut monotonically_decreasing = true;
        // let mut is_slowly_changing = true;

        for (n1, n2) in report.iter().copied().tuple_windows() {
            match n2 - n1 {
                -3..=-1 => monotonically_increasing = false,
                0 => return false, // not changing
                1..=3 => monotonically_decreasing = false,
                _ => return false, // changing too quickly
            }
        }

        monotonically_increasing || monotonically_decreasing
    }

    match part {
        Part::One => {
            println!(
                "{}",
                reports
                    .into_iter()
                    .filter(|report| report_is_safe(report))
                    .count()
            );
        }
        Part::Two => {
            let n_safe = reports
                .into_iter()
                .filter(|report| {
                    (0..report.len())
                        .map(|skip_num| {
                            let mut partial_report = report.clone();
                            partial_report.remove(skip_num);
                            partial_report
                        })
                        .any(|partial_report| report_is_safe(&partial_report))
                })
                .count();

            println!("{n_safe}");
        }
    }
}

fn day3(part: Part) {
    let code = include_str!("day3_input.txt");

    match part {
        Part::One => {
            let regex = lazy_regex::regex!(r"mul\((\d{1,3}),(\d{1,3})\)");
            let mut sum = 0;
            for m in regex.captures_iter(code) {
                // assuming recursive calls like mul(mul(1,1),2) are not valid
                let num1 = m.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let num2 = m.get(2).unwrap().as_str().parse::<i64>().unwrap();
                sum += num1 * num2;
            }
            println!("{sum}");
        }
        Part::Two => {
            let regex = lazy_regex::regex!(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))");
            let mut sum = 0;
            let mut is_enabled = true;
            for m in regex.captures_iter(code) {
                let whole_match_text = m.get(1).unwrap().as_str();
                if whole_match_text.starts_with("mul") {
                    if is_enabled {
                        // assuming recursive calls like mul(mul(1,1),2) are not valid
                        let num1 = m.get(2).unwrap().as_str().parse::<i64>().unwrap();
                        let num2 = m.get(3).unwrap().as_str().parse::<i64>().unwrap();
                        sum += num1 * num2;
                    }
                } else {
                    is_enabled = whole_match_text == "do()";
                }
            }
            println!("{sum}");
        }
    }
}

fn day4(part: Part) {
    let letter_matrix = include_str!("day4_input.txt");
    let letter_matrix = letter_matrix
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    match part {
        Part::One => {
            let n_rows = letter_matrix.len() as i64;
            let n_cols = letter_matrix[0].len() as i64;
            let mut count = 0;
            let directions =
                itertools::iproduct!(-1i64..=1, -1i64..=1).filter(|&dir| dir != (0, 0));
            for row in 0..n_rows {
                for col in 0..n_cols {
                    for (dx, dy) in directions.clone() {
                        let word_found = (0..4)
                            .map(|n_steps| (row + n_steps * dy, col + n_steps * dx))
                            .take_while(|(row, col)| {
                                (0..n_rows).contains(row) && (0..n_cols).contains(col)
                            })
                            .map(|(row, col)| letter_matrix[row as usize][col as usize])
                            .eq("XMAS".chars());
                        if word_found {
                            count += 1;
                        }
                    }
                }
            }
            println!("{count}");
        }
        Part::Two => {
            let n_rows = letter_matrix.len();
            let n_cols = letter_matrix[0].len();

            let mut x_mas_count = 0;

            for row in 1..n_rows - 1 {
                for col in 1..n_cols - 1 {
                    if letter_matrix[row][col] != 'A' {
                        continue;
                    }

                    let diagonal_contains_mas = |cell1: (usize, usize), cell2| {
                        let mut n_m = 0;
                        let mut n_s = 0;

                        for (row, col) in [cell1, cell2] {
                            n_m += (letter_matrix[row][col] == 'M') as u8;
                            n_s += (letter_matrix[row][col] == 'S') as u8;
                        }
                        n_m == 1 && n_s == 1
                    };

                    if diagonal_contains_mas((row - 1, col - 1), (row + 1, col + 1))
                        && diagonal_contains_mas((row - 1, col + 1), (row + 1, col - 1))
                    {
                        x_mas_count += 1;
                    }
                }
            }

            println!("{x_mas_count}");
        }
    }
}

fn day5(part: Part) {
    let input = include_str!("day5_input.txt");
    let (rules_input, update_page_lists) = input.split_once("\n\n").unwrap();
    let mut rules = HashMap::new();
    for (before, after) in rules_input.lines().map(|line| {
        let mut nums = line.split('|').map(parse_num);
        (nums.next().unwrap(), nums.next().unwrap())
    }) {
        rules.entry(before).or_insert(HashSet::new()).insert(after);
    }

    let update_page_lists = update_page_lists
        .lines()
        .map(|line| line.split(',').map(parse_num).collect_vec())
        .collect_vec();

    let mut sorted_lists = vec![];
    let mut unsorted_lists = vec![];
    for page_list in update_page_lists {
        // what is the middle page, if list doesn't have oddly numbered count?
        assert!(page_list.len() % 2 == 1);

        let mut already_seen = HashSet::new();
        let correctly_ordered = page_list.iter().all(|&page| {
            already_seen.insert(page);
            let rule = rules.get(&page);
            rule.map_or(true, |pages_after| already_seen.is_disjoint(pages_after))
        });
        if correctly_ordered {
            sorted_lists.push(page_list);
        } else {
            unsorted_lists.push(page_list);
        }
    }

    match part {
        Part::One => {
            let middle_page_sum = sorted_lists
                .into_iter()
                .map(|list| list[list.len() / 2])
                .sum::<u64>();
            println!("{middle_page_sum}");
        }
        Part::Two => {
            let mut middle_page_sum = 0;
            for page_list in unsorted_lists {
                // one should actually distinguish different elements of the page, but I assume
                // they are unique
                let mut not_yet_used = page_list.iter().cloned().collect::<HashSet<_>>();
                let mut output = vec![];

                while let Some(some_page) = not_yet_used.iter().cloned().next() {
                    topologic_sort(&mut output, &mut not_yet_used, &rules, some_page);
                }

                middle_page_sum += output[output.len() / 2];
            }

            println!("{middle_page_sum}");

            // DFS approach
            // assuming cycles and page duplicates are impossible
            fn topologic_sort(
                output: &mut Vec<u64>,
                unused_yet: &mut HashSet<u64>,
                page_rules: &HashMap<u64, HashSet<u64>>,
                page: u64,
            ) {
                if !unused_yet.contains(&page) {
                    return;
                }

                // The order here is (I believe) the inverse of what is actually demanded,
                // but it doesn't matter to get the middle page and I'm lazy.
                for &page_after in page_rules.get(&page).into_iter().flatten() {
                    topologic_sort(output, unused_yet, page_rules, page_after);
                }
                unused_yet.remove(&page);
                output.push(page);
            }
        }
    }
}

fn day6(part: Part) {
    let input = include_str!("day6_input.txt");
    let blocked_cells = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect_vec())
        .collect_vec();

    let width = blocked_cells[0].len() as i32;

    let (starting_idx, _) = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .enumerate()
        .find(|&(_, c)| c == '^')
        .unwrap();

    let starting_idx = starting_idx as i32;
    let pos = Vector2::new(starting_idx % width, starting_idx / width);

    // outputs (guard_route_is_cyclic, field of visited cells)
    fn simulate_guard(
        blocked_cells: &Vec<Vec<bool>>,
        starting_pos: Vector2<i32>,
    ) -> (bool, Vec<Vec<bool>>) {
        let height = blocked_cells.len() as i32;
        let width = blocked_cells[0].len() as i32;
        let mut visited_cells = vec![vec![false; width as usize]; height as usize];

        let rotate_right_90_deg = Matrix2::new(0, -1, 1, 0);
        let mut velocity = Vector2::new(0, -1);
        let mut already_visited = HashSet::new();

        let mut pos = starting_pos;
        // Problem guarantees the guard leaves the area. Otherwise, we'd need to check for cycles.
        loop {
            visited_cells[pos.y as usize][pos.x as usize] = true;
            if !already_visited.insert((pos, velocity)) {
                // cycle detected
                break (true, visited_cells);
            }
            let next_pos = pos + velocity;
            if !((0..width).contains(&next_pos.x) && (0..height).contains(&next_pos.y)) {
                break (false, visited_cells);
            } else if blocked_cells[next_pos.y as usize][next_pos.x as usize] {
                velocity = rotate_right_90_deg * velocity;
            } else {
                pos = next_pos;
            }
        }
        // printout the fields visited
        // for (row, row_visited) in visited_cells.iter().enumerate() {
        //     for (col, &is_visited) in row_visited.iter().enumerate() {
        //         let ch = if blocked_cells[row][col] {
        //             '#'
        //         } else if is_visited {
        //             'X'
        //         } else {
        //             '.'
        //         };
        //         print!("{}", ch);
        //     }
        //     println!()
        // }
    }

    let (_, undisturbed_route_visited) = simulate_guard(&blocked_cells, pos);

    match part {
        Part::One => {
            let n_visited = undisturbed_route_visited
                .into_iter()
                .flatten()
                .filter(|&visited| visited)
                .count();
            println!("{n_visited}");
        }
        Part::Two => {
            let obstruction_candidates = undisturbed_route_visited
                .into_iter()
                .enumerate()
                .flat_map(|(row_nr, row)| {
                    row.into_iter()
                        .enumerate()
                        .filter_map(move |(col_nr, visited)| {
                            if visited && !(row_nr == pos.y as usize && col_nr == pos.x as usize) {
                                Some((row_nr, col_nr))
                            } else {
                                None
                            }
                        })
                });

            let mut blocked_cells = blocked_cells;
            let mut n_possible_cycles = 0;
            for (row, col) in obstruction_candidates {
                blocked_cells[row][col] = true;

                if simulate_guard(&blocked_cells, pos).0 {
                    n_possible_cycles += 1;
                }

                blocked_cells[row][col] = false;
            }

            println!("{n_possible_cycles}");
        }
    }
}

fn day7(part: Part) {
    let input = include_str!("day7_input.txt");
    let equations = input
        .lines()
        .map(|line| {
            let (desired_result, numbers) = line.split_once(": ").unwrap();
            let desired_result = parse_num(desired_result);
            let numbers = numbers.split_whitespace().map(parse_num).collect_vec();
            (desired_result, numbers)
        })
        .collect_vec();

    // DFS of all possibilities
    fn result_can_be_reached(
        desired_result: u64,
        intermediate_result: u64,
        remaining_numbers: &[u64],
    ) -> bool {
        if let Some((&next_num, rest)) = remaining_numbers.split_first() {
            for operation in [std::ops::Add::add, std::ops::Mul::mul] {
                let next_result = operation(intermediate_result, next_num);
                if result_can_be_reached(desired_result, next_result, rest) {
                    return true;
                }
            }
            false
        } else {
            return desired_result == intermediate_result;
        }
    }

    match part {
        Part::One => {
            let mut total_calibration_result = 0;
            for (desired_result, numbers) in equations {
                let (first_num, rest) = numbers.split_first().unwrap();
                if result_can_be_reached(desired_result, *first_num, rest) {
                    total_calibration_result += desired_result;
                }
            }
            println!("{total_calibration_result}");
        }
        Part::Two => {
            to_be_implemented();
        }
    }
}

#[allow(unused)]
fn day(part: Part) {
    let input = include_str!("day1_input.txt");

    match part {
        Part::One => {}
        Part::Two => {
            to_be_implemented();
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Part {
    One,
    Two,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Part::One),
            "2" => Ok(Part::Two),
            _ => Err("only part 1 and 2 exist".to_owned()),
        }
    }
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(parse(try_from_str = parse_day))]
    day: u8,
    part: Part,
}

fn parse_day(day: &str) -> Result<u8, Box<dyn std::error::Error>> {
    match day.parse()? {
        day @ 1..=25 => Ok(day),
        _ => Err(format!("must be in range 1-25").into()),
    }
}

fn to_be_implemented() {
    println!("not yet implemented")
}

fn main() {
    let opt = Opt::from_args();

    let day_fns = [
        day1 as fn(Part),
        day2,
        day3,
        day4,
        day5,
        day6,
        day7,
        // day8,
        // day9,
        // day10,
        // day11,
        // day12,
        // day13,
        // day14,
        // day15,
        // day16,
        // day17,
        // day18,
        // day19,
        // day20,
    ];

    let day_fn = day_fns
        .get((opt.day - 1) as usize)
        .copied()
        .unwrap_or(|_| to_be_implemented());
    day_fn(opt.part);
}
