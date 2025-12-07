pub fn main() {
    let data = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .filter(|line| !line.iter().all(|&b| b == b'.'))
        .collect::<Vec<&[u8]>>();

    let mut paths = [0; 141];
    paths[data[0].iter().position(|&b| b == b'S').unwrap()] = 1;

    for row in data[1..].iter() {
        for (i, count) in paths.into_iter().enumerate() {
            if row[i] == b'^' {
                paths[i] = 0;
                paths[i - 1] += count;
                paths[i + 1] += count;
            }
        }
    }

    println!("{}", paths.iter().sum::<usize>());
}
