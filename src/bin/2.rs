enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose,
    Win,
    Draw,
}

fn to_choice(p1: &str) -> Result<Choice, String> {
    match p1 {
        "A" | "X" => Ok(Choice::Rock),
        "B" | "Y" => Ok(Choice::Paper),
        "C" | "Z" => Ok(Choice::Scissors),
        _ => Err("not a valid Choice character".to_string()),
    }
}

fn to_rnd_outcome(outcome: &str) -> Result<Outcome, String> {
    match outcome {
        "X" => Ok(Outcome::Lose),
        "Y" => Ok(Outcome::Draw),
        "Z" => Ok(Outcome::Win),
        _ => Err("not a valid Outcome character".to_string()),
    }
}

// part 1
// fn calc_score(p1: Choice, p2: Choice) -> u32 {
//     match (p1, p2) {
//         (Choice::Rock, Choice::Rock) => 1 + 3,
//         (Choice::Rock, Choice::Paper) => 2 + 6,
//         (Choice::Rock, Choice::Scissors) => 3 + 0,
//         (Choice::Paper, Choice::Rock) => 1 + 0,
//         (Choice::Paper, Choice::Paper) => 2 + 3,
//         (Choice::Paper, Choice::Scissors) => 3 + 6,
//         (Choice::Scissors, Choice::Rock) => 1 + 6,
//         (Choice::Scissors, Choice::Paper) => 2 + 0,
//         (Choice::Scissors, Choice::Scissors) => 3 + 3,
//     }
// }

// part 2
fn calc_score(p1: Choice, outcome: Outcome) -> u32 {
    match (p1, outcome) {
        (Choice::Rock, Outcome::Win) => 2 + 6,
        (Choice::Rock, Outcome::Lose) => 3 + 0,
        (Choice::Rock, Outcome::Draw) => 1 + 3,
        (Choice::Paper, Outcome::Win) => 3 + 6,
        (Choice::Paper, Outcome::Lose) => 1 + 0,
        (Choice::Paper, Outcome::Draw) => 2 + 3,
        (Choice::Scissors, Outcome::Win) => 1 + 6,
        (Choice::Scissors, Outcome::Lose) => 2 + 0,
        (Choice::Scissors, Outcome::Draw) => 3 + 3,
    }
}

fn main() {
    let lines = include_str!("../input/2.txt").lines();

    let sum = lines
        .map(|line| {
            let mut choices = line.split(" ");

            let c1 = choices
                .next()
                .expect("to have a character for player 1 choice");

            let c2 = choices
                .next()
                .expect("to have a character for player 2 choice");

            let op_choice = to_choice(c1).expect("character should be either A, B, or C");

            // part 1
            // let my_choice = to_choice(c2).expect("character should be either A, B, or C");
            // calc_score(op_choice, my_choice)

            // part 2
            let outcome = to_rnd_outcome(c2).expect("character should be either X, Y, or Z");

            calc_score(op_choice, outcome)
        })
        .sum::<u32>();

    println!("{}", sum);
}
