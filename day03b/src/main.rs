pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| b == &b'\n')
            .flat_map(|mut line| {
                #[rustfmt::skip]
                let f = |line: &[u8]| {
                    line.iter()
                        .enumerate()
                        .fold((0, 0), |(nn, ii), (i, &n)| { if n > nn { (n, i) } else { (nn, ii) }})
                };

                (0..12).rev().map(move |p| {
                    let (n, i) = f(&line[..line.len() - p]);
                    line = &line[i + 1..];
                    (n - b'0') as usize * 10usize.pow(p as u32)
                })
            })
            .sum::<usize>(),
    );
}
