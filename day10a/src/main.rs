use itertools::Itertools;

pub fn main() {
    let machines = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .map(|line| {
            let mut iter = line.split(|&b| b == b' ');

            let lights = iter
                .next()
                .map(|lights| &lights[1..lights.len() - 1])
                .map(|lights| lights.into_iter().map(|&b| b == b'#').collect::<Vec<_>>())
                .unwrap();
            let toggles = iter
                .take_while(|toggles| toggles[0] == b'(')
                .map(|toggles| &toggles[1..toggles.len() - 1])
                .map(|toggles| {
                    toggles
                        .split(|&b| b == b',')
                        .map(|n| atoi::atoi::<usize>(n).unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            (lights, toggles)
        })
        .collect::<Vec<_>>();

    let mut total = 0;
    for (lights, toggles) in machines {
        let mut found = false;
        'outer: for i in 1..10 {
            for toggles in toggles.iter().cloned().combinations_with_replacement(i) {
                let mut state = vec![false; lights.len()];

                for toggle in toggles {
                    for &index in &toggle {
                        state[index] = !state[index];
                    }
                }

                if state == lights {
                    found = true;
                    total += i;
                    break 'outer;
                }
            }
        }

        if !found {
            unreachable!("No solution found");
        }
    }

    println!("{total}");
}
