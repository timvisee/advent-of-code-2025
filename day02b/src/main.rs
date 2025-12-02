// Using modulo trick from <https://github.com/MizardX/AdventOfCode_2025/blob/main/src/day_02.rs>

pub fn main() {
    let count = include_bytes!("../input.txt")
        .split(|b| b == &b',')
        .map(|n| {
            let mut iter = n
                .splitn(2, |b| b == &b'-')
                .map(|n| atoi::atoi::<usize>(n).unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .flat_map(|(a, b)| a..=b)
        .filter(|n| match n {
            10..=99 => n % 11 == 0,
            100..=999 => n % 111 == 0,
            1_000..=9_999 => n % 101 == 0 || n % 1_111 == 0,
            10_000..=99_999 => n % 11_111 == 0,
            100_000..=999_999 => n % 1_001 == 0 || n % 10_101 == 0 || n % 111_111 == 0,
            1_000_000..=9_999_999 => n % 1_111_111 == 0,
            10_000_000..=99_999_999 => n % 10_001 == 0 || n % 1_010_101 == 0 || n % 11_111_111 == 0,
            100_000_000..=999_999_999 => n % 1_001_001 == 0 || n % 111_111_111 == 0,
            1_000_000_000..=9_999_999_999 => {
                n % 100_001 == 0 || n % 101_010_101 == 0 || n % 1_111_111_111 == 0
            }
            _ => false,
        })
        .sum::<usize>();

    println!("{count}");
}
