fn part1(s: &str) {
    let mut sum = 0;
    for line in s.lines() {
        let lhs = line.chars().find(char::is_ascii_digit).unwrap().to_string();
        let rhs = line
            .chars()
            .rev()
            .find(char::is_ascii_digit)
            .unwrap()
            .to_string();
        sum += (lhs + rhs.as_str()).parse::<i32>().unwrap();
    }
    println!("sum: {sum}");
}

fn find_first_string_index(mut haystack: String) -> usize {
    while !haystack.is_empty() {
        for (s, i) in [
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]
        .into_iter()
        {
            if haystack.starts_with(s) {
                return i;
            }
        }
        haystack = haystack[1..].to_owned();
    }
    panic!("Couldn't find needle!");
}

fn find_last_string_index(mut haystack: String) -> usize {
    let mut most_recent = 0;
    while !haystack.is_empty() {
        for (s, i) in [
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]
        .into_iter()
        {
            if haystack.starts_with(s) {
                most_recent = i;
            }
        }
        haystack = haystack[1..].to_owned();
    }
    most_recent
}

fn part2(s: &str) {
    let mut sum = 0;

    for line in s.lines() {
        let lhs = find_first_string_index(line.to_owned());
        let rhs = find_last_string_index(line.to_owned());
        //println!("{lhs}{rhs} {line}");
        sum += lhs * 10 + rhs;
    }
    println!("sum: {sum}");
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;
    part1(&s);
    part2(&s);

    Ok(())
}
