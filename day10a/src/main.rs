pub fn main() {
    let presses = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .map(|line| {
            let mut iter = line.split(|&b| b == b' ');

            let leds = iter
                .next()
                .map(|leds| {
                    leds[1..]
                        .iter()
                        .rev()
                        .fold(0, |acc, &b| (acc << 1) + (b == b'#') as u16)
                })
                .unwrap();
            let btns = iter
                .take_while(|toggles| toggles[0] == b'(')
                .map(|toggles| {
                    toggles[1..]
                        .split(|&b| b == b',')
                        .fold(0, |a, b| a + (1 << (b[0] - b'0')))
                })
                .collect::<arrayvec::ArrayVec<_, 16>>();

            (0..2usize.pow(btns.len() as _)).fold(usize::MAX, |min, mask| {
                let n = mask.count_ones() as usize;
                if n < min
                    && btns
                        .iter()
                        .enumerate()
                        .filter(|(ix, _)| mask & (1 << ix) != 0)
                        .fold(leds, |acc, (_, &b)| acc ^ b)
                        == 0
                {
                    return min.min(n);
                }
                min
            })
        })
        .sum::<usize>();

    println!("{presses}");
}
