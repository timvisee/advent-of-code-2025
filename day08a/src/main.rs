type Coord = [usize; 3];

const MAX: usize = 188_000_000;

pub fn main() {
    let coords = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .map(|line| {
            let mut nums = line.split(|&b| b == b',');
            std::array::from_fn(|_| atoi::atoi::<usize>(nums.next().unwrap()).unwrap())
        })
        .collect::<Vec<_>>();

    let mut edges = Vec::with_capacity(coords.len() * coords.len() / 2);
    for i in 0..coords.len() - 1 {
        for j in i + 1..coords.len() {
            let dist = dist(coords[i], coords[j]);
            if dist < MAX {
                edges.push((i, j, dist));
            }
        }
    }
    edges.sort_unstable_by_key(|&(_, _, dist)| dist);

    let mut dsu = aph_disjoint_set::DisjointSetArrayU16::<1000>::new();
    edges.into_iter().take(1000).for_each(|(a, b, _)| {
        dsu.union(a, b);
    });

    let mut circuits = (0..coords.len())
        .fold(std::collections::HashMap::new(), |mut acc, i| {
            *acc.entry(dsu.get_root(i)).or_default() += 1;
            acc
        })
        .into_values()
        .collect::<Vec<_>>();
    circuits.sort_unstable_by_key(|&circuit| std::cmp::Reverse(circuit));

    println!("{}", circuits[0..3].iter().product::<usize>());
}

#[inline]
fn dist(a: Coord, b: Coord) -> usize {
    a[0].abs_diff(b[0]).pow(2) + a[1].abs_diff(b[1]).pow(2) + a[2].abs_diff(b[2]).pow(2)
}
