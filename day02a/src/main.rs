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
        .filter(|n| {
            let s = format!("{n}");

            // Exclude numbers with odd length
            if s.len() % 2 != 0 {
                return false;
            }

            let (a, b) = s.split_at(s.len() / 2);
            assert_eq!(a.len(), b.len());

            a == b
        })
        .sum::<usize>();

    println!("{count}",);
}
