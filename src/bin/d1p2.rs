static INPUT: &str = include_str!("../../inputs/d1.txt");

fn main() {
    let (first, second): (Vec<i32>, Vec<i32>) = INPUT
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

    let score = first
        .iter()
        .map(|n1| n1 * second.iter().filter(|&n2| n1 == n2).count() as i32)
        .sum::<i32>();

    println!("{score}");
}
