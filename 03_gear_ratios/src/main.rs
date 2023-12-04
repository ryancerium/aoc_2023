use std::{
    collections::{BTreeMap, HashSet},
    fmt::Write,
};

struct Cell {
    c: char,
    adjacent: bool,
    id: i32,
}

impl Cell {
    fn new(c: char) -> Cell {
        Cell {
            c: if c == '.' { ' ' } else { c },
            adjacent: false,
            id: 0,
        }
    }

    fn is_part(&self) -> bool {
        !self.c.is_ascii_digit() && self.c != ' '
    }

    fn mark_as_adjacent(&mut self) {
        if self.c.is_ascii_digit() {
            self.adjacent = true
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.c)?;
        f.write_char(if self.adjacent { 'a' } else { ' ' })?;
        Ok(())
    }
}

type Schematic = Vec<Vec<Cell>>;

fn row_range(schematic: &Schematic, row: usize) -> std::ops::Range<usize> {
    let height = schematic.len();
    let start = if row == 0 { 0 } else { row - 1 };
    let end = if row == height - 1 { row + 1 } else { row + 2 };
    start..end
}

fn col_range(schematic: &Schematic, col: usize) -> std::ops::Range<usize> {
    let width = schematic[0].len();
    let start = if col == 0 { 0 } else { col - 1 };
    let end = if col == width - 1 { col + 1 } else { col + 2 };
    start..end
}

fn _print_schematic(schematic: &Schematic) {
    for row in schematic.iter() {
        for cell in row.iter() {
            print!("{cell}");
        }
        println!();
    }
}

fn part1(s: &str) -> Option<()> {
    let mut schematic: Schematic = s
        .lines()
        .map(|line| line.chars().map(Cell::new).collect())
        .collect();

    let h = schematic.len();
    let w = schematic[0].len();

    for row in 0..h {
        for col in 0..w {
            if !schematic[row][col].is_part() {
                continue;
            }

            for r in row_range(&schematic, row) {
                for c in col_range(&schematic, col) {
                    //println!("Marking s[{r:2}][{c:2}]");
                    schematic[r][c].mark_as_adjacent();
                }
            }
        }
    }

    let mut part_numbers = Vec::new();
    for row in schematic.iter() {
        let mut part_number = String::new();
        let mut adjacent = false;
        for cell in row.iter() {
            if cell.c.is_ascii_digit() {
                part_number.push(cell.c);
                adjacent |= cell.adjacent;
                continue;
            }

            if adjacent && !part_number.is_empty() {
                part_numbers.push(part_number.parse::<i32>().unwrap());
            }
            part_number.clear();
            adjacent = false;
        }
        if adjacent && !part_number.is_empty() {
            part_numbers.push(part_number.parse::<i32>().unwrap());
        }
    }

    //println!("{part_numbers:?}");
    let sum: i32 = part_numbers.iter().sum();
    println!("Part one answer: {sum}");

    Some(())
}

fn part2(s: &str) -> Option<()> {
    let mut schematic: Schematic = s
        .lines()
        .map(|line| line.chars().map(Cell::new).collect())
        .collect();

    let h = schematic.len();
    let w = schematic[0].len();

    let mut id = 1;
    for row in schematic.iter_mut() {
        for cell in row.iter_mut() {
            if cell.c.is_ascii_digit() {
                cell.id = id;
            } else {
                id += 1;
            }
        }
        id += 1;
    }

    // for row in schematic.iter() {
    //     for cell in row.iter() {
    //         let id = cell.id;
    //         print!("{id:3}");
    //     }
    //     println!("");
    // }

    let mut id_pairs = HashSet::new();
    for row in 0..h {
        for col in 0..w {
            if schematic[row][col].c != '*' {
                continue;
            }

            let mut surrounding_ids = HashSet::new();
            for r in row_range(&schematic, row) {
                for c in col_range(&schematic, col) {
                    if schematic[r][c].c.is_ascii_digit() {
                        surrounding_ids.insert(schematic[r][c].id);
                    }
                }
            }

            if surrounding_ids.len() == 2 {
                let mut ids = surrounding_ids.iter();
                id_pairs.insert((*ids.next().unwrap(), *ids.next().unwrap()));
            }
        }
    }

    // id_pairs
    //     .iter()
    //     .for_each(|(id0, id1)| println!("{id0:3} * {id1:3}"));

    let mut part_numbers = BTreeMap::new();
    for row in schematic.iter() {
        let mut part_id = 0;
        let mut part_number = String::new();

        for cell in row.iter() {
            if cell.c.is_ascii_digit() {
                part_id = cell.id;
                part_number.push(cell.c);
                continue;
            }

            if !part_number.is_empty() {
                part_numbers.insert(part_id, part_number.parse::<i32>().unwrap());
                part_number.clear();
            }
        }

        if !part_number.is_empty() {
            part_numbers.insert(part_id, part_number.parse::<i32>().unwrap());
        }
    }

    // part_numbers
    //     .iter()
    //     .for_each(|(id, value)| println!("{id:3} -> {value}"));

    let gear_ratio_sum: i32 = id_pairs
        .iter()
        .map(|(id0, id1)| part_numbers.get(id0).unwrap() * part_numbers.get(id1).unwrap())
        .sum();

    println!("Part two answer: {gear_ratio_sum}");
    Some(())
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    part1(&s);
    part2(&s);

    Ok(())
}
