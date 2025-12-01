pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| b == &b'\n')
            .fold((50i16, 0), |(dial, sum), op| {
                let mut num = atoi::atoi::<i16>(&op[1..]).unwrap();
                if op[0] == b'L' {
                    num *= -1;
                }

                let new_dial = dial.wrapping_add(num);
                (
                    new_dial.rem_euclid(100),
                    sum + if new_dial <= 0 {
                        (dial != 0) as i16 + (-new_dial / 100)
                    } else {
                        new_dial / 100
                    },
                )
            })
            .1,
    );
}
