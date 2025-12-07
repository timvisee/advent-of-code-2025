use std::collections::HashSet;

pub fn main() {
    let data = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.iter().all(|&b| b == b'.'))
        .collect::<Vec<&[u8]>>();

    let start = data[0].iter().position(|&b| b == b'S').unwrap();
    let mut paths = HashSet::from([start]);
    let mut splits = 0;

    for row in data[1..].iter() {
        for i in paths.clone() {
            if row[i] == b'^' {
                paths.remove(&i);
                paths.insert(i - 1);
                paths.insert(i + 1);
                splits += 1;
            }
        }
    }

    println!("{splits}"); // 1590
}
