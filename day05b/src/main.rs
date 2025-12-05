pub fn main() {
    let mut ranges = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .take_while(|line| !line.is_empty())
        .map(|n| {
            let (a, b) = n.split_at(n.iter().position(|b| b == &b'-').unwrap());
            atoi::atoi::<usize>(a).unwrap()..=atoi::atoi::<usize>(&b[1..]).unwrap()
        })
        .collect::<Vec<_>>();
    ranges.sort_unstable_by_key(|r| std::cmp::Reverse(*r.start()));

    let (mut merged, mut current) = (Vec::with_capacity(ranges.len() / 2), ranges.pop().unwrap());
    while let Some(range) = ranges.pop() {
        if current.contains(range.start()) {
            current = *current.start()..=*current.end().max(range.end());
        } else {
            merged.push(current);
            current = range;
        }
    }
    merged.push(current);

    #[rustfmt::skip]
    println!("{}", merged.iter().map(|r| r.end() - r.start() + 1).sum::<usize>());
}
