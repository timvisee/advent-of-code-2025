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
            let s = s.as_bytes();

            for len in 1..=s.len() / 2 {
                if !s.len().is_multiple_of(len) {
                    continue;
                }

                let (needle, haystack) = s.split_at(len);

                if haystack.chunks(len).all(|chunk| chunk == needle) {
                    return true;
                }
            }

            false
        })
        .sum::<usize>();

    println!("{count}"); // 28915664389
}
