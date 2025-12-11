use std::collections::{HashMap, HashSet};

type Dev = [u8; 3];

pub fn main() {
    let (fwd, bwd) = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .map(|line| {
            (
                line[0..3].try_into().unwrap(),
                line[5..]
                    .chunks(4)
                    .map(|c| c[0..3].try_into().unwrap())
                    .collect::<Vec<Dev>>(),
            )
        })
        .fold(
            (HashMap::new(), HashMap::<_, HashSet<_>>::new()),
            |(mut fwd, mut bwd), (dev, outs)| {
                for &out in &outs {
                    bwd.entry(out).or_default().insert(dev);
                }
                fwd.insert(dev, outs);
                (fwd, bwd)
            },
        );

    println!("{}", paths(&fwd, bwd.clone(), *b"you", *b"out"));
}

#[inline]
fn paths(
    fwd: &HashMap<Dev, Vec<Dev>>,
    mut bwd: HashMap<Dev, HashSet<Dev>>,
    from: Dev,
    to: Dev,
) -> usize {
    let mut count = HashMap::new();
    count.insert(from, 1);

    let mut pending = fwd
        .keys()
        .copied()
        .filter(|dev| !bwd.contains_key(dev))
        .collect::<Vec<_>>();

    while let Some(dev) = pending.pop() {
        for dep in fwd.get(&dev).into_iter().flat_map(|i| i.iter().copied()) {
            let c = count.get(&dev).copied().unwrap_or_default();
            *count.entry(dep).or_default() += c;

            let revs = bwd.get_mut(&dep).unwrap();
            revs.remove(&dev);
            if revs.is_empty() {
                pending.push(dep);
            }
        }
    }

    count[&to]
}
