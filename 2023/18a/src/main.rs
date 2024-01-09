fn calculate() -> usize {
    let contents = std::fs::read_to_string("input.txt").expect("failed to read file");
    let mut width: isize = 0;
    let mut maxwidth: isize = 0;
    let mut maxlines: isize = 0;
    let mut lines: isize = 0;
    let mut topleftx: isize = 0;
    let mut toplefty: isize = 0;
    let mut count: usize = 0;
    let mut orders: Vec<(&str, isize, &str)> = vec![];
    for line in contents.lines() {
        let (dir, rest) = line.split_once(" ").unwrap();
        let (amount, color) = rest.split_once(" ").unwrap();
        let amount: isize = amount.parse().unwrap();
        orders.push((dir, amount, color));
        if dir == "R" {
            width += amount;
            if width > maxwidth {
                maxwidth = width;
            }
        }
        if dir == "L" {
            width -= amount;
        }
        if dir == "D" {
            lines += amount;
            if lines > maxlines {
                maxlines = lines;
            }
        }
        if dir == "U" {
            lines -= amount;
        }
        if topleftx > width {
            topleftx = width;
        }
        if toplefty > lines {
            toplefty = lines;
        }
    }
    maxwidth += 1 - topleftx;
    maxlines += 1 - toplefty;
    let mut map: Vec<char> = vec!['.'; (maxwidth * maxlines) as usize];
    let mut x = -topleftx;
    let mut y = -toplefty;
    macro_rules! getmap {
        ($x:expr,$y:expr) => {
            map[($y * maxwidth + $x) as usize]
        };
    }
    for (dir, amount, _color) in orders {
        for _ in 0..amount {
            match dir {
                "L" => x -= 1,
                "R" => x += 1,
                "U" => y -= 1,
                "D" => y += 1,
                _ => {}
            }
            getmap!(x, y) = '#';
            count += 1;
        }
    }
    let mut v: Vec<(isize, isize)> = vec![];
    v.push((100, 100));
    loop {
        if v.is_empty() {
            break;
        }
        let (x, y) = v.pop().unwrap();

        for (x, y) in [(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)] {
            if (getmap!(x, y)) == '.' {
                v.push((x, y));
                getmap!(x, y) = '*';
                count += 1;
            }
        }
    }
    return count;
}

fn main() {
    let result = calculate();
    println!("{result}");
}

#[test]
fn correct_result() {
    assert_eq!(calculate(),95356);
}