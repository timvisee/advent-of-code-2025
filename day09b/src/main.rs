type Coord = [usize; 2];

pub fn main() {
    let poly = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .map(|line| {
            let mut nums = line.split(|&b| b == b',');
            std::array::from_fn(|_| atoi::atoi::<usize>(nums.next().unwrap()).unwrap())
        })
        .collect::<Vec<_>>();

    let (mut h, mut v) = (vec![], vec![]);
    for (i, &[ax, ay]) in poly.iter().enumerate() {
        let [bx, by] = poly[(i + 1) % poly.len()];
        if ax == bx {
            v.push((ax, ay.min(by), ay.max(by)));
        } else {
            h.push((ay, ax.min(bx), ax.max(bx)));
        }
    }

    h.sort_unstable_by_key(|e| e.0);
    v.sort_unstable_by_key(|e| e.0);
    let h_ys = h.iter().map(|&h| h.0).collect::<Vec<_>>();
    let v_xs = v.iter().map(|&e| e.0).collect::<Vec<_>>();
    let h_max_len = *h.iter().max_by_key(|e| e.2 - e.1).unwrap();
    let v_max_len = *v.iter().max_by_key(|e| e.2 - e.1).unwrap();

    let mut size = 0;
    for ([ax, ay], [bx, by], area) in poly[..poly.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| poly[i + 1..].iter().map(move |&b| (a, b, area(a, b))))
    {
        if area > size
            && check_rect(
                (ax.min(bx), ay.min(by), ax.max(bx), ay.max(by)),
                &h,
                &v,
                h_max_len,
                v_max_len,
                &h_ys,
                &v_xs,
            )
        {
            size = area;
        }
    }

    println!("{size}");
}

#[inline]
fn area(a: Coord, b: Coord) -> usize {
    (a[0].abs_diff(b[0]) + 1) * (a[1].abs_diff(b[1]) + 1)
}

#[inline]
fn check_rect(
    (ax, ay, bx, by): (usize, usize, usize, usize),
    h: &[(usize, usize, usize)],
    v: &[(usize, usize, usize)],
    (y, x_min, x_max): (usize, usize, usize),
    (x, y_min, y_max): (usize, usize, usize),
    h_ys: &[usize],
    v_xs: &[usize],
) -> bool {
    if (ay < y && y < by && ((x_min < bx && bx <= x_max) || (x_min <= ax && ax < x_max)))
        || (ax < x && x < bx && ((y_min < by && by <= y_max) || (y_min <= ay && ay < y_max)))
    {
        return false;
    }

    let start = h_ys.iter().position(|&y| y > ay).unwrap_or(0);
    let end = h_ys[start..]
        .iter()
        .position(|&y| y > by)
        .map(|i| start + i)
        .unwrap_or(h.len());
    for &(_, x_min, x_max) in &h[start..end] {
        if x_min <= ax && ax < x_max || x_min < bx && bx <= x_max {
            return false;
        }
    }

    let start = v_xs.iter().position(|&x| x > ax).unwrap_or(0);
    let end = v_xs[start..]
        .iter()
        .position(|&x| x > bx)
        .map(|i| start + i)
        .unwrap_or(v.len());
    for &(_, y_min, y_max) in &v[start..end] {
        if y_min <= ay && ay < y_max || y_min < by && by <= y_max {
            return false;
        }
    }

    true
}
