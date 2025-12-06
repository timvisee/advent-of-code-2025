pub fn main() {
    let data = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .map(|line| {
            line.split(|&b| b == b' ')
                .filter(|num| !num.is_empty())
                .collect()
        })
        .collect::<Vec<Vec<&[u8]>>>();

    println!(
        "{}",
        (0..data[0].len())
            .map(|i| {
                let nums = data[..data.len() - 1]
                    .iter()
                    .map(|row| atoi::atoi::<usize>(row[i]).unwrap());
                if data.last().unwrap()[i][0] == b'+' {
                    nums.sum::<usize>()
                } else {
                    nums.product::<usize>()
                }
            })
            .sum::<usize>(),
    );
}
