use std::collections::BTreeMap;

fn parse_card(s: &str) -> (Vec<i32>, Vec<i32>) {
    let s = s.split(':').nth(1).unwrap();
    let mut s = s.split('|');

    let winners = s
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let owned = s
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    (winners, owned)
}

fn part1(s: &str) {
    let two: i32 = 2;
    let mut total_points = 0;
    for line in s.lines() {
        let (winners, owned) = parse_card(line);
        let matched: u32 = owned
            .iter()
            .map(|number| if winners.contains(number) { 1 } else { 0 })
            .sum();
        if matched > 0 {
            total_points += two.pow(matched - 1);
        }
    }
    println!("Part one total points: {total_points}");
}

fn part2(s: &str) {
    let mut card_number = 1;
    let mut cards = BTreeMap::new();

    let mut cards_to_process = Vec::new();
    for line in s.lines() {
        let (winners, owned) = parse_card(line);
        let matched: u32 = owned
            .iter()
            .map(|number| if winners.contains(number) { 1 } else { 0 })
            .sum();
        cards.insert(card_number, matched);
        cards_to_process.push((card_number, matched));
        card_number += 1;
    }

    let mut total_processed = 0;

    while !cards_to_process.is_empty() {
        total_processed += cards_to_process.len();
        let mut won_cards = Vec::new();
        for (card_number, matched) in cards_to_process.iter() {
            for won_card_number in (card_number + 1)..(card_number + 1 + matched) {
                won_cards.push((won_card_number, *cards.get(&won_card_number).unwrap()));
            }
        }
        cards_to_process = won_cards;
    }

    println!("Part two processed {total_processed} cards");
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    part1(&s);
    part2(&s);

    Ok(())
}
