type Coord = [usize; 2];

pub fn main() {
    let poly = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .map(|line| {
            let mut nums = line.split(|&b| b == b',');
            std::array::from_fn(|_| atoi::atoi::<usize>(nums.next().unwrap()).unwrap())
        })
        .collect::<Vec<Coord>>();

    let mut rects = poly[..poly.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| poly[i + 1..].iter().map(move |&b| (a, b, size(a, b))))
        .collect::<Vec<(Coord, Coord, usize)>>();
    rects.sort_unstable_by_key(|&(_, _, size)| std::cmp::Reverse(size));

    let (_, _, size) = rects
        .into_iter()
        .find(|(a, b, _)| is_rect_inside(&poly, *a, *b))
        .unwrap();

    println!("{size}");
}

#[inline]
fn size(a: Coord, b: Coord) -> usize {
    (a[0].abs_diff(b[0]) + 1) * (a[1].abs_diff(b[1]) + 1)
}

#[inline]
fn is_inside(poly: &[Coord], [px, py]: Coord) -> bool {
    let n = poly.len();
    let mut inside = false;
    for i in 0..n {
        let ([ux, uy], [vx, vy]) = (poly[i], poly[(i + 1) % n]);
        if ux == vx && py >= uy.min(vy) && py < uy.max(vy) && px < ux {
            inside = !inside;
        }
    }
    inside
}

#[inline]
fn is_rect_inside(poly: &[Coord], [ax, ay]: Coord, [bx, by]: Coord) -> bool {
    let n = poly.len();
    let (ax, bx) = (ax.min(bx), ax.max(bx));
    let (ay, by) = (ay.min(by), ay.max(by));

    // Check corners
    if [[ax, ay], [ax, by], [bx, ay], [bx, by]]
        .into_iter()
        .any(|coord| !is_inside(poly, coord))
    {
        return false;
    }

    // Vertical edges
    for &x in &[ax, bx] {
        let mut ys = Vec::with_capacity(4);
        for i in 0..n {
            let ([ux, uy], [vx, vy]) = (poly[i], poly[(i + 1) % n]);
            if uy == vy && x >= ux.min(vx) && x <= ux.max(vx) && uy > ay && uy < by {
                ys.push(uy);
            }
        }
        ys.sort();
        ys.dedup();

        let mut prev = ay;
        for &yi in &ys {
            if yi <= prev {
                continue;
            }
            if !is_inside(poly, [x, (prev + yi) / 2]) {
                return false;
            }
            prev = yi;
        }
        if prev < by && !is_inside(poly, [x, (prev + by) / 2]) {
            return false;
        }
    }

    // Horizontal edges
    for &y in &[ay, by] {
        let mut xs = Vec::with_capacity(4);
        for i in 0..n {
            let ([ux, uy], [vx, vy]) = (poly[i], poly[(i + 1) % n]);
            if ux == vx && y >= uy.min(vy) && y <= uy.max(vy) && ux > ax && ux < bx {
                xs.push(ux);
            }
        }
        xs.sort_unstable();
        xs.dedup();

        let mut prev = ax;
        for &xi in &xs {
            if xi <= prev {
                continue;
            }
            if !is_inside(poly, [(prev + xi) / 2, y]) {
                return false;
            }
            prev = xi;
        }
        if prev < bx && !is_inside(poly, [(prev + bx) / 2, y]) {
            return false;
        }
    }

    true
}
