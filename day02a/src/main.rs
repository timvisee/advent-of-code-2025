// Using modulo trick from <https://github.com/MizardX/AdventOfCode_2025/blob/main/src/day_02.rs>

pub fn main() {
    let count = include_bytes!("../input.txt")
        .split(|b| b == &b',')
        .map(|n| {
            let (a, b) = n.split_at(n.iter().position(|b| b == &b'-').unwrap());
            (
                atoi::atoi::<usize>(a).unwrap(),
                atoi::atoi::<usize>(&b[1..]).unwrap(),
            )
        })
        .flat_map(|(a, b)| a..=b)
        .filter(|n| match n {
            10..=99 => n % 11 == 0,
            1_000..=9_999 => n % 101 == 0,
            100_000..=999_999 => n % 1_001 == 0,
            10_000_000..=99_999_999 => n % 10_001 == 0,
            1_000_000_000..=9_999_999_999 => n % 100_001 == 0,
            _ => false,
        })
        .sum::<usize>();

    println!("{count}");
}
