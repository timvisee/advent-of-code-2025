use itertools::Itertools;

pub fn main() {
    let mut lines = include_bytes!("../input.txt").split(|&b| b == b'\n');

    let shape_sizes = lines
        .by_ref()
        .take(6 * 5)
        .tuples()
        .map(|(_, a, b, c, _)| {
            a.into_iter().filter(|&&b| b != b'#').count()
                + b.into_iter().filter(|&&b| b != b'#').count()
                + c.into_iter().filter(|&&b| b != b'#').count()
        })
        .collect::<Vec<usize>>();

    println!(
        "{}",
        lines
            .into_iter()
            .map(|region| {
                let (size, counts) =
                    region.split_at(region.iter().position(|b| *b == b':').unwrap());
                let area = size
                    .split(|&b| b == b'x')
                    .map(|n| atoi::atoi::<usize>(n).unwrap())
                    .product::<usize>();
                let counts = counts[2..]
                    .split(|&b| b == b' ')
                    .map(|n| atoi::atoi::<usize>(n).unwrap())
                    .collect::<Vec<_>>();
                (area, counts)
            })
            .filter(|(area, counts)| {
                let min_cells = counts
                    .iter()
                    .zip(&shape_sizes)
                    .map(|(count, shape_size)| count * shape_size)
                    .sum::<usize>();
                if *area < min_cells {
                    return false;
                }

                let max_cells = counts.iter().map(|&count| count * 9).sum::<usize>();
                *area >= max_cells
            })
            .count()
    );
}
