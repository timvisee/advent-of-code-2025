pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|&b| b == b'\n')
            .skip(6 * 5)
            .map(|region| {
                let (size, counts) =
                    region.split_at(region.iter().position(|b| *b == b':').unwrap());
                (
                    size.split(|&b| b == b'x')
                        .map(|n| atoi::atoi::<usize>(n).unwrap())
                        .product::<usize>(),
                    counts[3..]
                        .split(|&b| b == b' ')
                        .map(|n| atoi::atoi::<usize>(n).unwrap())
                        .collect::<arrayvec::ArrayVec<_, 6>>(),
                )
            })
            .filter(|(area, counts)| {
                *area >= counts.iter().map(|&count| count * 9).sum::<usize>()
            })
            .count()
    );
}
