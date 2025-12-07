use std::collections::HashMap;

pub fn main() {
    let data = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.iter().all(|&b| b == b'.'))
        .collect::<Vec<&[u8]>>();

    let start = data[0].iter().position(|&b| b == b'S').unwrap();
    let mut paths = HashMap::from([(start, 1)]);

    for row in data[1..].iter() {
        for (i, count) in paths.clone() {
            if row[i] == b'^' {
                paths.remove(&i);
                *paths.entry(i - 1).or_default() += count;
                *paths.entry(i + 1).or_default() += count;
            }
        }
    }

    println!("{}", paths.values().sum::<usize>()); // 20571740188555
}
