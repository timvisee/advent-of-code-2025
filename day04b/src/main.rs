#[rustfmt::skip]
const A: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

pub fn main() {
    let mut m = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(<[u8]>::to_vec)
        .collect::<Vec<_>>();

    let mut total = 0;
    let mut queue = Vec::from_iter((0..m[0].len()).flat_map(|x| (0..m.len()).map(move |y| (x, y))));

    while let Some((x, y)) = queue.pop() {
        if m[y][x] == b'@'
            && A.into_iter()
                .filter(|&(ax, ay)| {
                    m.get(y.wrapping_add_signed(ay))
                        .and_then(|row| row.get(x.wrapping_add_signed(ax)))
                        .is_some_and(|&c| c == b'@')
                })
                .count()
                < 4
        {
            total += 1;
            m[y][x] = b'.';
            queue.extend(
                A.into_iter()
                    .map(|(ax, ay)| (x.wrapping_add_signed(ax), y.wrapping_add_signed(ay)))
                    .filter(|&(xx, yy)| m.get(yy).and_then(|row| row.get(xx)).is_some()),
            );
        }
    }

    println!("{total}");
}
