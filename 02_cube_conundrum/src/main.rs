use std::cmp::max;

fn is_possible_game(s: &str) -> Option<i32> {
    let mut parts = s.split(':');

    let id = parts.next()?.split(' ').nth(1)?.parse::<i32>().ok()?;

    println!("Game {id} was... ");
    let max_r = 12;
    let max_g = 13;
    let max_b = 14;

    for reveal in parts.next()?.split(';') {
        println!("  reveal: {reveal}");
        for revealed_cubes in reveal.split(',') {
            let count = revealed_cubes
                .trim()
                .split(' ')
                .next()?
                .trim()
                .parse::<i32>()
                .ok()?;

            if revealed_cubes.contains("red") && count > max_r
                || revealed_cubes.contains("green") && count > max_g
                || revealed_cubes.contains("blue") && count > max_b
            {
                println!("  ❌");
                return None;
            }
        }
    }

    println!("  ✅");
    Some(id)
}

fn part1(s: &str) {
    let valid_id_sum: i32 = s
        .lines()
        .map(is_possible_game)
        .map(|id| id.unwrap_or(0))
        .sum();
    println!("Sum of Valid IDs: {valid_id_sum}");
}

fn power_of_game(s: &str) -> Option<i32> {
    let mut parts = s.split(':');

    let id = parts.next()?.split(' ').nth(1)?.parse::<i32>().ok()?;

    let mut max_r = 0;
    let mut max_g = 0;
    let mut max_b = 0;

    for reveal in parts.next()?.split(';') {
        for revealed_cubes in reveal.split(',') {
            let count = revealed_cubes
                .trim()
                .split(' ')
                .next()?
                .trim()
                .parse::<i32>()
                .ok()?;

            if revealed_cubes.contains("red") {
                max_r = max(count, max_r);
            } else if revealed_cubes.contains("green") {
                max_g = max(count, max_g);
            } else if revealed_cubes.contains("blue") {
                max_b = max(count, max_b);
            }
        }
    }

    let power = max_r * max_g * max_b;
    println!("Game {id:3} power is {max_r:3} * {max_g:3} * {max_b:3} = {power}");
    Some(power)
}

fn part2(s: &str) {
    let game_power_sums: i32 = s.lines().map(power_of_game).map(|id| id.unwrap_or(0)).sum();
    println!("Sum of Game Powers: {game_power_sums}");
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    part1(&s);
    part2(&s);

    Ok(())
}
