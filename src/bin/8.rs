use std::collections::HashMap;

fn make_visible_key(col: usize, row: usize) -> String {
    format!("{}, {}", col, row)
}

fn main() {
    let mut lines = include_str!("../input/8.txt").lines().enumerate();

    let mut row_tallest;
    let mut col_tallest: HashMap<usize, usize> = HashMap::new();

    let mut visible: HashMap<String, (usize, usize)> = HashMap::new();
    let mut candidates_right: Vec<(usize, usize, usize)> = Vec::new();
    let mut candidates_down: HashMap<usize, Vec<(usize, usize, usize)>> = HashMap::new();

    let mut row = 0;
    let mut col = 0;

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
                visible.insert(make_visible_key(col, row), (col, row));
                already_visible = true;
            }

            match tallest_in_col {
                Some(tallest) => {
                    if current_height > *tallest {
                        col_tallest.insert(col, current_height);

                        if !already_visible {
                            visible.insert(make_visible_key(col, row), (col, row));
                        }
                    }
                }
                None => {
                    col_tallest.insert(col, current_height);
                    candidates_down.insert(col, Vec::new());

                    if !already_visible {
                        visible.insert(make_visible_key(col, row), (col, row));
                    }
                }
            }

            let col_candidates = candidates_down.get_mut(&col).unwrap();

            candidates_right.retain(|candidate| candidate.2 <= current_height);
            col_candidates.retain(|candidate| candidate.2 <= current_height);

            if !already_visible {
                let new_candidate = (col, row, current_height);
                candidates_right.push(new_candidate);
                col_candidates.push((col, row, current_height));
            }
        }

        for candidate in candidates_right.iter() {
            visible.insert(make_visible_key(col, row), (candidate.0, candidate.1));
        }

        candidates_right = Vec::new();
    }

    for candidates in candidates_down.values() {
        for candidate in candidates.iter() {
            visible.insert(
                make_visible_key(candidate.0, candidate.1),
                (candidate.0, candidate.1),
            );
        }
    }

    for r in 0..row {
        visible.insert(make_visible_key(col, r), (col, r));
    }

    for c in 0..col {
        visible.insert(make_visible_key(c, row), (c, row));
    }

    println!("visible trees: {}", visible.len());
}
