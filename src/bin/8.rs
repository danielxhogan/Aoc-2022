use std::collections::HashMap;
use std::{iter::Enumerate, str::Lines};

fn make_visible_key(col: usize, row: usize) -> String {
    format!("{}, {}", col, row)
}

// part 1
fn calculate_visible(mut lines: Enumerate<Lines>) {
    let mut row_tallest;
    let mut col_tallest: HashMap<usize, usize> = HashMap::new();

    let mut visible: HashMap<String, bool> = HashMap::new();
    let mut candidates_right: Vec<(usize, usize, usize)> = Vec::new();
    let mut candidates_down: HashMap<usize, Vec<(usize, usize, usize)>> = HashMap::new();

    let mut row;
    let mut col;

    while let Some(line) = lines.next() {
        row = line.0;
        row_tallest = 0;
        let mut trees = line.1.chars().enumerate();

        while let Some(tree) = trees.next() {
            col = tree.0;
            let current_height: usize = tree.1.to_string().parse().unwrap();
            let mut already_visible = false;

            let tallest_in_col = col_tallest.get_mut(&col);

            if current_height > row_tallest || col == 0 {
                row_tallest = current_height;
                visible.insert(make_visible_key(col, row), true);
                already_visible = true;
            }

            match tallest_in_col {
                Some(tallest) => {
                    if current_height > *tallest {
                        col_tallest.insert(col, current_height);

                        if !already_visible {
                            visible.insert(make_visible_key(col, row), true);
                            already_visible = true;
                        }
                    }
                }
                None => {
                    col_tallest.insert(col, current_height);
                    candidates_down.insert(col, Vec::new());

                    if !already_visible {
                        visible.insert(make_visible_key(col, row), true);
                        already_visible = true;
                    }
                }
            }

            let col_candidates = candidates_down.get_mut(&col).unwrap();

            candidates_right.retain(|candidate| candidate.2 > current_height);
            col_candidates.retain(|candidate| candidate.2 > current_height);

            if !already_visible {
                let new_candidate = (col, row, current_height);
                candidates_right.push(new_candidate);
                col_candidates.push(new_candidate);
            }
        }

        for candidate in candidates_right.iter() {
            visible.insert(make_visible_key(candidate.0, candidate.1), true);
        }

        candidates_right = Vec::new();
    }

    for candidates in candidates_down.values() {
        for candidate in candidates.iter() {
            visible.insert(make_visible_key(candidate.0, candidate.1), true);
        }
    }

    println!("visible trees: {}", visible.len());
}

// part 2
fn calculate_scenic_score(mut lines: Enumerate<Lines>) {
    let mut forest: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut row;

    while let Some(line) = lines.next() {
        row = line.0;
        forest.push(Vec::new());
        let mut trees = line.1.chars().enumerate();

        while let Some(tree) = trees.next() {
            let current_height: usize = tree.1.to_string().parse().unwrap();
            forest[row].push((current_height, 0));
        }
    }

    let mut scene_score;
    let mut highest_score = 0;
    let mut delta;
    let mut trees;

    for i in 0..forest.len() {
        for j in 0..forest[i].len() {
            scene_score = 1;

            // looking left
            delta = j;
            trees = 0;

            while delta > 0 {
                delta -= 1;
                trees += 1;

                if forest[i][delta].0 >= forest[i][j].0 {
                    break;
                }
            }

            scene_score *= trees;

            // looking right
            delta = j;
            trees = 0;

            while delta < forest[i].len() - 1 {
                delta += 1;
                trees += 1;

                if forest[i][delta].0 >= forest[i][j].0 {
                    break;
                }
            }

            scene_score *= trees;

            // looking up
            delta = i;
            trees = 0;

            while delta > 0 {
                delta -= 1;
                trees += 1;

                if forest[delta][j].0 >= forest[i][j].0 {
                    break;
                }
            }

            scene_score *= trees;

            // looking down
            delta = i;
            trees = 0;

            while delta < forest.len() - 1 {
                delta += 1;
                trees += 1;

                if forest[delta][j].0 >= forest[i][j].0 {
                    break;
                }
            }

            scene_score *= trees;

            if scene_score > highest_score {
                highest_score = scene_score;
            }
        }
    }

    println!("highest score: {}", highest_score);
}

fn main() {
    let lines = include_str!("../input/8.txt").lines().enumerate();

    // part 1
    calculate_visible(lines.clone());

    // part 2
    calculate_scenic_score(lines);
}
