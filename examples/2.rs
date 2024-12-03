use anyhow::Result;
use itertools::Itertools;
use std::time::Instant;

fn main() -> Result<()> {
    let start = Instant::now();
    let input = parse_input()?;
    let elapsed = start.elapsed();
    println!("Parse Input {elapsed:?}");

    let part1_start = Instant::now();
    let part1 = part1(&input)?;
    let part1_elapsed = part1_start.elapsed();
    let part1_total = part1_elapsed + elapsed;
    println!("Part 1 {part1_elapsed:?} (total: {part1_total:?})",);

    let part2_start = Instant::now();
    let part2 = part2(input)?;
    let part2_elapsed = part2_start.elapsed();
    let part2_total = part2_elapsed + elapsed;
    println!("Part 2 {part2_elapsed:?} (total: {part2_total:?})",);

    println!("Results:");
    println!("\tPart 1: {part1}");
    println!("\tPart 2: {part2}");

    Ok(())
}

fn parse_input() -> Result<Vec<Vec<i32>>> {
    let input = include_str!("../inputs/input_2")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<i32>)
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()?;

    Ok(input)
}

fn part1(input: &[Vec<i32>]) -> Result<usize> {
    let output = input
        .iter()
        .filter(|report| is_valid_report(report))
        .count();

    Ok(output)
}

fn part2(input: Vec<Vec<i32>>) -> Result<usize> {
    let output = input
        .into_iter()
        .filter(|report| {
            if is_valid_report(report) {
                return true;
            }

            report.iter().enumerate().any(|(i, _)| {
                let variant = [&report[..i], &report[i + 1..]].concat();
                is_valid_report(&variant)
            })
        })
        .count();

    Ok(output)
}

fn is_valid_report(report: &[i32]) -> bool {
    let is_strictly_monotonic = || {
        report.iter().tuple_windows().all(|(a, b)| a < b)
            || report.iter().tuple_windows().all(|(a, b)| a > b)
    };
    let has_valid_diffs = || {
        report
            .iter()
            .tuple_windows()
            .all(|(a, b)| (1..=3).contains(&(b - a).abs()))
    };

    is_strictly_monotonic() && has_valid_diffs()
}
