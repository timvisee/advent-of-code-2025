pub fn main() {
    let mut buffer = itoa::Buffer::new();

    let count = include_bytes!("../input.txt")
        .split(|b| b == &b',')
        .map(|n| {
            let mut iter = n
                .splitn(2, |b| b == &b'-')
                .map(|n| atoi::atoi::<usize>(n).unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .flat_map(|(a, b)| a..=b)
        .filter(|n| {
            let s = buffer.format(*n).as_bytes();
            if !s.len().is_multiple_of(2) {
                return false;
            }
            let (a, b) = s.split_at(s.len() / 2);
            a == b
        })
        .sum::<usize>();

    println!("{count}");
}
