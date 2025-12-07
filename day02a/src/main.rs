// Using modulo trick from <https://github.com/MizardX/AdventOfCode_2025/blob/main/src/day_02.rs>
// Using arithmetic approach from <https://github.com/erik-adelbert/aoc/blob/main/2025/2/aoc2.go>

pub fn main() {
    let count = include_bytes!("../input.txt")
        .split(|b| b == &b',')
        .map(|n| {
            let mut iter = n
                .splitn(2, |b| b == &b'-')
                .map(|n| atoi::atoi::<usize>(n).unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .flat_map(|(a, b)| {
            // Split ranges by powers of 10
            std::iter::successors(Some(10), move |x| Some(x * 10).filter(|&x| x < b * 10))
                .filter(move |&x| x > a)
                .scan(a, move |start, x| {
                    let span = (*start, (x - 1).min(b));
                    *start = x;
                    Some(span)
                })
        })
        .map(|(a, b)| match a {
            a if a >= 1000000000 => m(a, b, 100001),
            a if a >= 100000000 => 0,
            a if a >= 10000000 => m(a, b, 10001),
            a if a >= 1000000 => 0,
            a if a >= 100000 => m(a, b, 1001),
            a if a >= 10000 => 0,
            a if a >= 1000 => m(a, b, 101),
            a if a >= 100 => 0,
            a if a >= 10 => m(a, b, 11),
            _ => 0,
        })
        .sum::<usize>();

    println!("{count}");
}

/// Compute sum of all multiples of x in the range [a, b]
/// From <https://github.com/erik-adelbert/aoc/blob/3f55ce44ba249eb5b47c0edc439bc462a9c67bd7/2025/2/aoc2.go#L124-L141>
fn m(a: usize, b: usize, x: usize) -> usize {
    let first = a.div_ceil(x) * x;
    if first > b {
        return 0;
    }
    (first + b / x * x) * (b / x + 1 - first / x) / 2
}
