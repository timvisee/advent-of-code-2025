type Coord = [usize; 3];

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
            edges.push((i, j, dist(coords[i], coords[j])));
        }
    }
    edges.sort_unstable_by_key(|&(_, _, dist)| dist);

    let mut dsu = aph_disjoint_set::DisjointSetArrayU16::<1000>::new();
    let (a, b, _) = edges
        .into_iter()
        .filter(|&(a, b, _)| matches!(dsu.union(a, b), aph_disjoint_set::UnionResult::Success))
        .last()
        .unwrap();

    println!("{}", coords[a][0] * coords[b][0]);
}

#[inline]
fn dist(a: Coord, b: Coord) -> usize {
    a[0].abs_diff(b[0]).pow(2) + a[1].abs_diff(b[1]).pow(2) + a[2].abs_diff(b[2]).pow(2)
}
