pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| b == &b'\n')
            .fold((50i16, 0), |(mut dial, count), op| {
                let mut num = atoi::atoi::<i16>(&op[1..]).unwrap();
                if op[0] == b'L' {
                    num *= -1;
                }

                dial = dial.wrapping_add(num).rem_euclid(100);

                (dial, count + (dial == 0) as i16)
            })
            .1,
    );
}
