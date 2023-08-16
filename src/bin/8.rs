use std::collections::HashMap;
use std::{cell::RefCell, iter::Enumerate, rc::Rc, str::Lines};

struct Tree {
    height: usize,
    col: usize,
    row: usize,
    distance_left: usize,
    distance_up: usize,
    sight_left: Option<usize>,
    sight_right: Option<usize>,
    sight_up: Option<usize>,
    sight_down: Option<usize>,
    sight_total: usize,
}

impl Tree {
    fn new(height: usize, col: usize, row: usize) -> Rc<RefCell<Tree>> {
        Rc::new(RefCell::new(Tree {
            height,
            col,
            row,
            distance_left: 0,
            distance_up: 0,
            sight_left: None,
            sight_right: None,
            sight_up: None,
            sight_down: None,
            sight_total: 0,
        }))
    }

    fn incr_dist_left(&mut self) {
        self.distance_left += 1;
    }

    fn incr_dist_up(&mut self) {
        self.distance_up += 1;
    }
}

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
    let mut col;
    let mut row;

    while let Some(line) = lines.next() {
        row = line.0;
        forest.push(Vec::new());
        let mut trees = line.1.chars().enumerate();

        while let Some(tree) = trees.next() {
            col = tree.0;
            let current_height: usize = tree.1.to_string().parse().unwrap();
            forest[row].push((current_height, 0));
        }
    }

    let mut delta;

    for i in 0..forest.len() {
        println!("{}", forest[i][forest[0].len() - 1].0);
        for j in 0..forest[i].len() {
            delta = j;

            while delta > 0 {
                delta -= 1;
                forest[i][j].1 += 1;

                if forest[i][delta].0 >= forest[i][j].0 {
                    break;
                }
            }

            delta = j;

            while delta < forest[i].len() - 1 {
                delta += 1;
                forest[i][j].1 += 1;

                if forest[i][delta].0 >= forest[i][j].0 {
                    break;
                }
            }

            delta = i;

            while delta > 0 {
                delta -= 1;
                forest[i][j].1 += 1;

                if forest[delta][j].0 >= forest[i][j].0 {
                    break;
                }
            }

            delta = i;

            while delta < forest.len() - 1 {
                delta += 1;
                forest[i][j].1 += 1;

                if forest[delta][j].0 >= forest[i][j].0 {
                    break;
                }
            }
        }
    }

    let mut highest_scene_score = 0;

    for i in 0..forest.len() {
        for j in 0..forest[i].len() {
            if forest[i][j].1 > highest_scene_score {
                highest_scene_score = forest[i][j].1;
            }
        }
    }

    println!("highest scene score: {}", highest_scene_score);




    // let mut all_trees: Vec<Rc<RefCell<Tree>>> = Vec::new();
    // let mut seen_row: Vec<Rc<RefCell<Tree>>> = Vec::new();
    // let mut seen_col: HashMap<usize, Vec<Rc<RefCell<Tree>>>> = HashMap::new();

    // let mut row = 0;
    // let mut col = 0;

    // while let Some(line) = lines.next() {
    //     row = line.0;
    //     let mut trees = line.1.chars().enumerate();

    //     while let Some(tree) = trees.next() {
    //         col = tree.0;
    //         let current_height: usize = tree.1.to_string().parse().unwrap();
    //         let new_tree = Tree::new(current_height, col, row);
    //         let mut new_tree_borrow = new_tree.borrow_mut();

    //         let mut i = seen_row.len();

    //         while i > 0 {
    //             i -= 1;
    //             let mut current_tree = seen_row[i].borrow_mut();
    //             current_tree.incr_dist_left();

    //             if current_tree.height >= current_height && new_tree_borrow.sight_left == None {
    //                 new_tree_borrow.sight_left = Some(current_tree.distance_left);
    //                 new_tree_borrow.sight_total += current_tree.distance_left;
    //             }

    //             if current_height >= current_tree.height && current_tree.sight_right == None {
    //                 current_tree.sight_right = Some(current_tree.distance_left);
    //                 current_tree.sight_total += current_tree.distance_left;
    //             }
    //         }

    //         if new_tree_borrow.sight_left == None {
    //             new_tree_borrow.sight_left = Some(col);
    //             new_tree_borrow.sight_total = col;
    //         }

    //         seen_row.push(new_tree.clone());

    //         let current_col_maybe = seen_col.get_mut(&col);

    //         match current_col_maybe {
    //             None => {
    //                 new_tree_borrow.sight_up = Some(0);
    //                 seen_col.insert(col, vec![new_tree.clone()]);
    //             }
    //             Some(current_col) => {
    //                 let mut i = current_col.len();

    //                 while i > 0 {
    //                     i -= 1;
    //                     let mut current_tree = current_col[i].borrow_mut();
    //                     current_tree.incr_dist_up();

    //                     if current_tree.height >= current_height && new_tree_borrow.sight_up == None {
    //                         new_tree_borrow.sight_up = Some(current_tree.distance_up);
    //                         new_tree_borrow.sight_total += current_tree.distance_up;
    //                     }

    //                     if current_height >= current_tree.height && current_tree.sight_down == None {
    //                         current_tree.sight_down = Some(current_tree.distance_up);
    //                         current_tree.sight_total += current_tree.distance_up;
    //                     }
    //                 }

    //                 if new_tree_borrow.sight_up == None {
    //                     new_tree_borrow.sight_up = Some(row);
    //                     new_tree_borrow.sight_total = row;
    //                 }

    //                 current_col.push(new_tree.clone());
    //             }
    //         }
    //     }

    //     for i in 0..seen_row.len() {
    //         let mut current_tree = seen_row[i].borrow_mut();
    //         if current_tree.sight_right == None {
    //             current_tree.sight_right = Some(col - current_tree.col);
    //             current_tree.sight_total = col - current_tree.col;
    //         }
    //     }

    //     for _ in 0..seen_row.len() {
    //         all_trees.push(seen_row.pop().unwrap());
    //     }
    // }

    // let mut highest_scene_score = 0;

    // for trees in seen_col.values() {
    //     for tree in trees {
    //         let mut tree_borrow = tree.borrow_mut();
    //         if tree_borrow.sight_down == None {
    //             tree_borrow.sight_down = Some(row - tree_borrow.row);
    //             tree_borrow.sight_total += row - tree_borrow.row;
    //         }

    //         if tree_borrow.sight_total > highest_scene_score {
    //             highest_scene_score = tree_borrow.sight_total;
    //         }
    //     }
    // }

    // println!("highest scene score: {}", highest_scene_score);
}

fn main() {
    let lines = include_str!("../input/8.txt").lines().enumerate();

    // part 1
    calculate_visible(lines.clone());

    // part 2
    calculate_scenic_score(lines);
}
