/// Reverse version of day07b solution, though it is about 10% slower.

pub fn main() {
    let data = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.iter().all(|&b| b == b'.'))
        .collect::<Vec<&[u8]>>();

    println!(
        "{}",
        data[1..].iter().rev().fold([1usize; 141], |mut paths, p| {
            for i in 0..paths.len() {
                if p[i] == b'^' {
                    paths[i] = paths[i - 1] + paths[i + 1]
                }
            }
            paths
        })[data[0].iter().position(|&b| b == b'S').unwrap()],
    );
}
