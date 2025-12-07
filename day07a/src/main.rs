pub fn main() {
    let data = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.iter().all(|&b| b == b'.'))
        .collect::<Vec<&[u8]>>();

    let (mut paths, mut splits) = ([false; 141], 0);
    paths[data[0].iter().position(|&b| b == b'S').unwrap()] = true;

    for row in data[1..].iter() {
        for (i, used) in paths.into_iter().enumerate() {
            if row[i] == b'^' {
                paths[i] = false;
                paths[i - 1] = true;
                paths[i + 1] = true;
                splits += used as usize;
            }
        }
    }

    println!("{splits}"); // 1590
}
