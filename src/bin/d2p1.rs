static INPUT: &str = include_str!("../../inputs/d2.txt");

fn main() {
    let safe_reports = INPUT
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|n| {
            n.windows(2)
                .scan(None, |state, window| {
                    if window[0].abs_diff(window[1]) > 3 || window[0].abs_diff(window[1]) == 0 {
                        return Some(false);
                    }

                    if state.is_none() {
                        *state = Some(window[0] < window[1]);
                    }

                    Some(state.unwrap() == (window[1] > window[0]))
                })
                .all(|x| x)
        })
        .count();

    println!("{safe_reports}");
}
