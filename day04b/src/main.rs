pub fn main() {
    #[rustfmt::skip]
    const ADJACENT: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    let mut map = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(<[u8]>::to_vec)
        .collect::<Vec<Vec<u8>>>();

    let mut queue: Vec<(isize, isize)> = Vec::from_iter(
        (0..map[0].len() as isize).flat_map(|x| (0..map.len() as isize).map(move |y| (x, y))),
    );

    let mut total = 0;
    while let Some((x, y)) = queue.pop() {
        if map[y as usize][x as usize] == b'.' {
            continue;
        }

        if ADJACENT
            .into_iter()
            .filter(|(ax, ay)| {
                map.get((y + ay) as usize)
                    .and_then(|row| row.get((x + ax) as usize))
                    .is_some_and(|&c| c == b'@')
            })
            .take(4)
            .count()
            >= 4
        {
            continue;
        }

        total += 1;
        map[y as usize][x as usize] = b'.';

        queue.extend(
            ADJACENT
                .into_iter()
                .map(|(ax, ay)| (ax + x, ay + y))
                .filter(|&(xx, yy)| {
                    map.get(yy as usize)
                        .and_then(|row| row.get(xx as usize))
                        .is_some_and(|&c| c == b'@')
                }),
        );
    }

    println!("{total}"); // 8317
}
