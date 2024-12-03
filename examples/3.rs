use anyhow::Result;
use std::time::Instant;

fn main() -> Result<()> {
    let start = Instant::now();
    let input = parse_input()?;
    let elapsed = start.elapsed();
    println!("Parse Input {elapsed:?}");

    let part1_start = Instant::now();
    let part1 = part1(input)?;
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

fn parse_input() -> Result<&'static str> {
    let input = include_str!("../inputs/input_3");

    Ok(input)
}

fn part1(input: &str) -> Result<u32> {
    let output = input
        .split("mul(")
        .map(|s| {
            s.split(")")
                .next()
                .expect("Could not find closing parenthesis")
                .split(",")
                // this will also filter out cases like `mul(737,853,^from()`
                .map(|s| s.parse::<u32>().unwrap_or(0))
                .product::<u32>() // why
        })
        .sum();

    Ok(output)
}
fn part2(input: &str) -> Result<u32> {
    let result = input
        .split("do()")
        .map(|s| s.split("don't()").next().unwrap_or(""))
        .collect::<String>();

    let result = part1(&result)?;

    Ok(result)
}
