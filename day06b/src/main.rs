pub fn main() {
    let data = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .collect::<Vec<_>>();

    let (mut nums, mut current, mut total, mut op) = (Vec::with_capacity(4), 0, 0, 0);

    for i in 0..data[0].len() {
        nums.clear();
        nums.extend(
            data[..data.len() - 1]
                .iter()
                .map(|line| line[i])
                .filter(|&b| b != b' '),
        );
        if nums.is_empty() {
            total += current;
            continue;
        }

        let value = nums
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &b)| (b - b'0') as usize * 10_usize.pow(i as u32))
            .sum::<usize>();

        let new = data.last().unwrap()[i];
        if new != b' ' {
            op = new;
            current = (op == b'*') as _;
        }

        if op == b'+' {
            current += value;
        } else {
            current *= value;
        }
    }
    total += current;

    println!("{total}");
}
