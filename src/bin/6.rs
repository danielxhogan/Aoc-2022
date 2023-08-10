fn main() {
    let input: Vec<char> = include_str!("../input/6.txt")
        .lines()
        .next()
        .unwrap()
        .chars()
        .collect();

    let mut marker: usize = 0;
    let mut skip = 0;

    // part 1
    // for i in 3..input.len() {
    //     if input[i] == input[i - 1] {
    //         skip = 3;
    //     } else if input[i] == input[i - 2] && skip < 2 {
    //         skip = 2;
    //     } else if input[i] == input[i - 3]  && skip < 1{
    //         skip = 1;
    //     }

    //     if skip == 0 {
    //         marker = i;
    //         break;
    //     }

    //     skip -= 1;
    // }

    // part 2
    for i in 13..input.len() {
        for j in 1..=13 {
            if input[i] == input[i - j] && skip < (14 - j) {
                skip = 14 - j;
                break;
            }
        }

        if skip == 0 {
            marker = i;
            break;
        }

        skip -= 1;
    }

    println!("{marker:?}")
}
