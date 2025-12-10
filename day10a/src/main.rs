use itertools::Itertools;

pub fn main() {
    let machines = include_bytes!("../input.txt")
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
            let toggles = iter
                .take_while(|toggles| toggles[0] == b'(')
                .map(|toggles| {
                    toggles[1..]
                        .split(|&b| b == b',')
                        .fold(0, |a, b| a + (1 << (b[0] - b'0')))
                })
                .collect::<arrayvec::ArrayVec<_, 16>>();

            (leds, toggles)
        })
        .collect::<Vec<_>>();

    println!(
        "{}",
        machines
            .into_iter()
            .map(|(leds, toggles)| {
                (1..10)
                    .flat_map(|i| toggles.iter().copied().combinations_with_replacement(i))
                    .find(|toggles| toggles.iter().fold(0, |acc, &b| acc ^ b) == leds)
                    .unwrap()
                    .len()
            })
            .sum::<usize>(),
    );
}
