type Coord = [usize; 2];

pub fn main() {
    let coords = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .map(|line| {
            let mut nums = line.split(|&b| b == b',');
            std::array::from_fn(|_| atoi::atoi::<usize>(nums.next().unwrap()).unwrap())
        })
        .collect::<Vec<Coord>>();

    println!(
        "{}",
        coords[..coords.len() - 1]
            .iter()
            .enumerate()
            .flat_map(|(i, &a)| coords[i + 1..].iter().map(move |&b| size(a, b)))
            .max()
            .unwrap(),
    );
}

#[inline]
fn size(a: Coord, b: Coord) -> usize {
    (a[0].abs_diff(b[0]) + 1) * (a[1].abs_diff(b[1]) + 1)
}
