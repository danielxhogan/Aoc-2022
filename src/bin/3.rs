use std::collections::HashMap;

type Scores = HashMap<char, u32>;
type ElfGroup = (String, String, String);

fn make_scores() -> Scores {
    let mut scores = HashMap::new();
    let mut c: char;

    let mut i = 97;
    while i <= 122 {
        c = char::from_u32(i).unwrap();
        scores.insert(c, i - 96);
        i += 1;
    }

    i = 65;
    while i <= 90 {
        c = char::from_u32(i).unwrap();
        scores.insert(c, i - 38);
        i += 1;
    }

    scores
}

fn find_duplicate(rucksack: &str, scores: Scores) -> u32 {
    let mid_point = rucksack.len() / 2;
    let mut first_half = rucksack[0..mid_point].chars();
    let mut second_half = rucksack[mid_point..].chars();

    let mut seen: HashMap<char, u32> = HashMap::new();
    let mut check_prev: Option<&u32>;

    let mut duplicates: Vec<u32> = Vec::new();

    while let Some(next_char) = first_half.next() {
        check_prev = seen.get(&next_char);

        match check_prev {
            Some(value) => {
                let new_val = value + 1;
                seen.insert(next_char, new_val);
            }
            None => {
                seen.insert(next_char, 1);
            }
        };
    }

    // println!();

    while let Some(next_char) = second_half.next() {
        check_prev = seen.get(&next_char);

        match check_prev {
            Some(_) => {
                duplicates.push(*scores.get(&next_char).unwrap());
                break;
            }
            None => (),
        }
    }

    duplicates.iter().sum()
}

fn get_badge_score(group: &ElfGroup, scores: &Scores) -> u32 {
    let mut elf_1_chars = group.0.chars();
    let mut elf_2_chars = group.1.chars();
    let mut elf_3_chars = group.2.chars();

    let mut seen: HashMap<char, bool> = HashMap::new();

    let mut in_common: HashMap<char, bool> = HashMap::new();
    let mut check_in_common: Option<&bool>;

    let mut badge: char = '!';

    while let Some(next_char) = elf_1_chars.next() {
        seen.insert(next_char, true);
    }

    while let Some(next_char) = elf_2_chars.next() {
        check_in_common = seen.get(&next_char);

        match check_in_common {
            Some(_) => {
                in_common.insert(next_char, true);
            }
            None => (),
        }
    }

    while let Some(next_char) = elf_3_chars.next() {
        check_in_common = in_common.get(&next_char);

        match check_in_common {
            Some(_) => {
                badge = next_char;
                break;
            }
            None => (),
        }
    }

    *scores.get(&badge).unwrap()
}

fn main() {
    let scores = make_scores();

    // part 1
    let total_score: u32 = include_str!("../input/3.txt")
        .lines()
        .map(|rucksack| find_duplicate(rucksack, scores.clone()))
        .sum();

    println!("{total_score:?}");

    // part 2
    let mut elf_group: Vec<&str> = Vec::new();
    let mut elf_groups: Vec<ElfGroup> = Vec::new();

    let mut lines = include_str!("../input/3.txt").lines();

    while let Some(line) = lines.next() {
        elf_group.push(line);

        if elf_group.len() == 3 {
            elf_groups.push((
                elf_group[0].to_string(),
                elf_group[1].to_string(),
                elf_group[2].to_string(),
            ));

            elf_group = Vec::new();
        }
    }

    let total_score: u32 = elf_groups
        .iter()
        .map(|group| get_badge_score(group, &scores))
        .sum();

    println!("{}", total_score);
}
