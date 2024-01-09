use std::fmt;

#[allow(dead_code)]
struct Stone {
    px: i64,
    py: i64,
    pz: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl<'a> Stone {
    pub fn new(
        mut points: impl Iterator<Item = &'a str>,
        mut velocities: impl Iterator<Item = &'a str>,
    ) -> Self {
        Self {
            px: points.next().unwrap().trim().parse().unwrap(),
            py: points.next().unwrap().trim().parse().unwrap(),
            pz: points.next().unwrap().trim().parse().unwrap(),
            vx: velocities.next().unwrap().trim().parse().unwrap(),
            vy: velocities.next().unwrap().trim().parse().unwrap(),
            vz: velocities.next().unwrap().trim().parse().unwrap(),
        }
    }
}

impl fmt::Debug for Stone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Stone")
            .field("px", &self.px)
            .field("py", &self.py)
            .field("vx", &self.vx)
            .field("vy", &self.vy)
            .finish()
    }
}

fn slope(px: i64, vx: i64, py: i64, vy: i64) -> Option<f64> {
    let x1 = px;
    let y1 = py;
    let x2 = px + vx;
    let y2 = py + vy;
    let divider: f64 = x2 as f64 - x1 as f64;
    if divider == 0.0 {
        return None;
    };
    return Some((y2 - y1) as f64 / divider);
}

fn calculate() -> i64 {
    let contents = std::fs::read_to_string("input.txt").expect("unable to read input file");
    let mut stones: Vec<Stone> = vec![];
    for line in contents.lines() {
        let (points, velocities) = line.split_once("@").unwrap();
        let points = points.split(",");
        let velocities = velocities.split(",");
        stones.push(Stone::new(points, velocities));
    }
    let mut i = stones.iter();
    let mut a = i.next().unwrap();
    let mut amount: i64 = 0;
    loop {
        for b in i.clone() {
            let aslope = slope(a.px, a.vx, a.py, a.vy).unwrap();
            let bslope = slope(b.px, b.vx, b.py, b.vy).unwrap();
            if aslope == bslope {
                assert!(a.px != b.px);
            } else {
                let ac = a.py as f64 - aslope * a.px as f64;
                let bc = b.py as f64 - bslope * b.px as f64;
                const MIN: f64 = 200000000000000.0;
                const MAX: f64 = 400000000000000.0;
                let x = (bc - ac) / (aslope - bslope);
                let y = aslope * x + ac;
                let timea = (x - a.px as f64) / a.vx as f64;
                let timeb = (x - b.px as f64) / b.vx as f64;
                if timea > 0.0 && timeb > 0.0 {
                    if x >= MIN && y >= MIN && x <= MAX && y <= MAX {
                        amount += 1;
                    }
                }
            }
        }
        if let Some(next) = i.next() {
            a = next;
            continue;
        }
        break;
    }
    return amount;
}

fn main() {
    let result = calculate();
    println!("{}", result);
}

#[test]
fn correct_result() {
    assert_eq!(calculate(),23760);
}