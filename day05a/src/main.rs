pub fn main() {
    let mut iter = include_bytes!("../input.txt").split(|b| b == &b'\n');

    let mut ranges = iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|n| {
            let (a, b) = n.split_at(n.iter().position(|b| b == &b'-').unwrap());
            atoi::atoi::<usize>(a).unwrap()..=atoi::atoi::<usize>(&b[1..]).unwrap()
        })
        .collect::<Vec<_>>();
    ranges.sort_unstable_by_key(|r| *r.start());

    println!(
        "{}",
        iter.map(|n| { atoi::atoi::<usize>(n).unwrap() })
            .filter(|n| ranges
                .iter()
                .skip_while(|r| r.end() < n)
                .take_while(|r| r.start() <= n)
                .any(|r| r.contains(n)))
            .count(),
    );
}
