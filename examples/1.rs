use anyhow::Result;
use std::collections::HashMap;
use std::time::Instant;

fn main() -> Result<()> {
    let start = Instant::now();
    let input = parse_input()?;
    let elapsed = start.elapsed();
    println!("Parse Input {elapsed:?}");

    let input_1 = input.clone();
    let input_2 = input;

    let part1_start = Instant::now();
    let part1 = part1(input_1)?;
    let part1_elapsed = part1_start.elapsed();
    let part1_total = part1_elapsed + elapsed;
    println!("Part 1 {part1_elapsed:?} (total: {part1_total:?})",);

    let part2_start = Instant::now();
    let part2 = part2(input_2)?;
    let part2_elapsed = part2_start.elapsed();
    let part2_total = part2_elapsed + elapsed;
    println!("Part 2 {part2_elapsed:?} (total: {part2_total:?})",);

    println!("Results:");
    println!("\tPart 1: {part1}");
    println!("\tPart 2: {part2}");

    Ok(())
}

fn parse_input() -> Result<(Vec<u32>, Vec<u32>)> {
    let input = include_str!("../inputs/input_1");

    let a = Vec::with_capacity(1000);
    let b = Vec::with_capacity(1000);

    let (a, b) = input
        .split_whitespace()
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .fold((a, b), |(mut a, mut b), chunk| {
            a.push(chunk[0].parse().expect("Could not parse input"));
            b.push(chunk[1].parse().expect("Could not parse input"));
            (a, b)
        });

    Ok((a, b))
}

fn part1((mut a, mut b): (Vec<u32>, Vec<u32>)) -> Result<u32> {
    a.sort_unstable();
    b.sort_unstable();

    let output: u32 = a.iter().zip(b.iter()).map(|(x, y)| x.abs_diff(*y)).sum();

    Ok(output)
}

fn part2((a, b): (Vec<u32>, Vec<u32>)) -> Result<u32> {
    let occurences = b
        .iter()
        .fold(HashMap::with_capacity(b.len()), |mut map, n| {
            *map.entry(n).or_insert(0) += 1;
            map
        });

    let output: u32 = a.iter().map(|n| *n * occurences.get(n).unwrap_or(&0)).sum();

    Ok(output)
}
