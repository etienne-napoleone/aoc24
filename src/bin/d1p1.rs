static INPUT: &str = include_str!("../../inputs/d1.txt");

fn main() {
    let (mut first, mut second): (Vec<i32>, Vec<i32>) = INPUT
        .lines()
        .map(|line| {
            let nums = line
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (*nums.first().unwrap(), *nums.last().unwrap())
        })
        .unzip();

    first.sort_unstable();
    second.sort_unstable();

    let distance = first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    println!("{distance}");
}
