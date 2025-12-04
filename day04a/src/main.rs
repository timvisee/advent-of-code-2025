pub fn main() {
    const ADJACENT: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let map = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .collect::<Vec<&[u8]>>();

    let mut total = 0;

    for x in 0..map[0].len() as isize {
        for y in 0..map.len() as isize {
            let cell = map[y as usize][x as usize];

            if cell == b'.' {
                continue;
            }

            let rolls = ADJACENT
                .into_iter()
                .filter(|(ax, ay)| {
                    let xx = x + ax;
                    let yy = y + ay;

                    map.get(yy as usize)
                        .and_then(|row| row.get(xx as usize))
                        .is_some_and(|&c| c == b'@')
                })
                .take(4)
                .count();

            if rolls < 4 {
                total += 1;
            }
        }
    }

    println!("{total}");
}
