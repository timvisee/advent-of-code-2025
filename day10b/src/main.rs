use arrayvec::ArrayVec;
use microlp::{LinearExpr, OptimizationDirection, Problem};

const BTNS_SIZE: usize = 13;
const JOLT_SIZE: usize = 10;

pub fn main() {
    let presses = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .map(|line| {
            let (first, last) = line.split_at(line.iter().position(|&b| b == b'{').unwrap());

            let btns = first[1..]
                .split(|&b| b == b' ')
                .skip(1)
                .filter(|btns| !btns.is_empty())
                .map(|btns| {
                    btns[1..]
                        .split(|&b| b == b',')
                        .map(|n| 1 << (n[0] - b'0'))
                        .sum()
                })
                .collect::<ArrayVec<u16, BTNS_SIZE>>();
            let jolts = last[1..]
                .split(|&b| b == b',')
                .map(|b| atoi::atoi::<u16>(b).unwrap())
                .collect::<ArrayVec<u16, JOLT_SIZE>>();

            let mut problem = Problem::new(OptimizationDirection::Minimize);
            let max = jolts.iter().copied().max().unwrap();
            let vars = (0..btns.len())
                .map(|_| problem.add_integer_var(1.0, (0, max as i32)))
                .collect::<ArrayVec<_, BTNS_SIZE>>();
            for (i, &n) in jolts.iter().enumerate() {
                problem.add_constraint(
                    btns.iter()
                        .zip(&vars)
                        .filter(|&(mask, _)| mask & (1 << i) != 0)
                        .fold(LinearExpr::empty(), |mut ex, (_, &var)| {
                            ex.add(var, 1.0);
                            ex
                        }),
                    microlp::ComparisonOp::Eq,
                    n as f64,
                );
            }
            problem.solve().unwrap().objective().round() as usize
        })
        .sum::<usize>();

    println!("{presses}");
}
