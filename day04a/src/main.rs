#[rustfmt::skip]
const A: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

pub fn main() {
    let m = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .collect::<Vec<_>>();

    println!(
        "{}",
        (0..m[0].len())
            .flat_map(|x| (0..m.len()).map(move |y| (x, y)))
            .filter(|&(x, y)| {
                m[y][x] == b'@'
                    && A.into_iter()
                        .filter(|&(ax, ay)| {
                            m.get(y.wrapping_add_signed(ay))
                                .and_then(|row| row.get(x.wrapping_add_signed(ax)))
                                .is_some_and(|&c| c == b'@')
                        })
                        .count()
                        < 4
            })
            .count(),
    );
}
