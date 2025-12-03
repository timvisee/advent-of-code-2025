pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| b == &b'\n')
            .map(|line| {
                #[rustfmt::skip]
                let f = |line: &[u8]| {
                    line.iter()
                        .enumerate()
                        .fold((0, 0), |(nn, ii), (i, &n)| { if n > nn { (n, i) } else { (nn, ii) }})
                };

                let (a, i) = f(&line[..line.len() - 1]);
                let (b, _) = f(&line[i + 1..]);
                ((a - b'0') * 10 + b - b'0') as usize
            })
            .sum::<usize>(),
    );
}
