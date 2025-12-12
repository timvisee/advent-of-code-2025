pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|&b| b == b'\n')
            .skip(6 * 5)
            .filter(|region| {
                let (size, counts) =
                    region.split_at(region.iter().position(|b| *b == b':').unwrap());
                size.split(|&b| b == b'x')
                    .map(|n| atoi::atoi::<usize>(n).unwrap())
                    .product::<usize>()
                    >= counts[3..]
                        .split(|&b| b == b' ')
                        .map(|n| atoi::atoi::<usize>(n).unwrap() * 9)
                        .sum()
            })
            .count()
    );
}
