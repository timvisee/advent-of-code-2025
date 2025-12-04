pub fn main() {
    #[rustfmt::skip]
    const ADJACENT: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    let mut m = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(<[u8]>::to_vec)
        .collect::<Vec<_>>();

    let mut total = 0;
    let mut queue = Vec::from_iter((0..m[0].len()).flat_map(|x| (0..m.len()).map(move |y| (x, y))));

    while let Some((x, y)) = queue.pop() {
        if m[y][x] == b'.' {
            continue;
        }

        if ADJACENT
            .into_iter()
            .filter(|(ax, ay)| {
                m.get((y as isize + ay) as usize)
                    .and_then(|row| row.get((x as isize + ax) as usize))
                    .is_some_and(|&c| c == b'@')
            })
            .count()
            >= 4
        {
            continue;
        }

        total += 1;
        m[y][x] = b'.';

        queue.extend(
            ADJACENT
                .into_iter()
                .map(|(ax, ay)| ((ax + x as isize) as usize, (ay + y as isize) as usize))
                .filter(|&(xx, yy)| {
                    m.get(yy)
                        .and_then(|row| row.get(xx))
                        .is_some_and(|&c| c == b'@')
                }),
        );
    }

    println!("{total}"); // 8317
}
