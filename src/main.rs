use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};
use structopt::StructOpt;

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
        // day3,
        // day4,
        // day5,
        // day6,
        // day7,
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
