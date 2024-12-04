use anyhow::Result;
use grid::{grid, Grid};
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

fn parse_input() -> Result<Grid<char>> {
    let input = include_str!("../inputs/input_4")
        .lines()
        .fold(grid![], |mut grid, line| {
            grid.push_row(line.chars().collect());
            grid
        });
    Ok(input)
}

fn part1(mut input: Grid<char>) -> Result<u32> {
    let output = (0..4).fold(0, |acc, rotation_count| {
        let grid = rotate_grid(&mut input, rotation_count);
        grid.indexed_iter().fold(acc, |acc, ((x, y), &c)| {
            if c != 'X' {
                return acc;
            }

            [(1, 0), (1, 1)].iter().fold(acc, |acc, &(dx, dy)| {
                if grid.get(x + dx, y + dy) == Some(&'M')
                    && grid.get(x + 2 * dx, y + 2 * dy) == Some(&'A')
                    && grid.get(x + 3 * dx, y + 3 * dy) == Some(&'S')
                {
                    return acc + 1;
                }

                acc
            })
        })
    });

    Ok(output)
}

fn part2(mut input: Grid<char>) -> Result<u32> {
    let output = (0..4).fold(0, |acc, rotation_count| {
        let grid = rotate_grid(&mut input, rotation_count);
        grid.indexed_iter().fold(acc, |acc, ((x, y), &c)| {
            if c != 'A' {
                return acc;
            }

            if grid.get(x - 1, y - 1) == Some(&'M')
                && grid.get(x + 1, y - 1) == Some(&'M')
                && grid.get(x - 1, y + 1) == Some(&'S')
                && grid.get(x + 1, y + 1) == Some(&'S')
            {
                return acc + 1;
            }

            acc
        })
    });

    Ok(output)
}

fn rotate_grid(grid: &mut Grid<char>, times: usize) -> Grid<char> {
    (0..times).fold(grid.clone(), |mut grid, _| {
        grid.rotate_right();
        grid
    })
}
