#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn c(x: isize, y: isize) -> usize {
    (y * 110 + x) as usize
}

fn light(
    mut dir: Direction,
    mut x: isize,
    mut y: isize,
    map: &mut Vec<char>,
    lightmap: &mut Vec<bool>,
    splitmap: &mut Vec<bool>,
) {
    use Direction::*;
    loop {
        match dir {
            Left => x = x - 1,
            Right => x = x + 1,
            Up => y = y - 1,
            Down => y = y + 1,
        }
        if x < 0 || y < 0 || x >= 110 || y >= 110 {
            return;
        }
        lightmap[c(x, y)] = true;
        match map[c(x, y)] {
            '.' => continue,
            '\\' => match dir {
                Left => dir = Up,
                Right => dir = Down,
                Up => dir = Left,
                Down => dir = Right,
            },
            '/' => match dir {
                Left => dir = Down,
                Right => dir = Up,
                Up => dir = Right,
                Down => dir = Left,
            },
            '-' => match dir {
                Left => continue,
                Right => continue,
                Up => {
                    if splitmap[c(x, y)] {
                        break;
                    }
                    splitmap[c(x, y)] = true;
                    light(Left, x, y, map, lightmap, splitmap);
                    dir = Right;
                }
                Down => {
                    if splitmap[c(x, y)] {
                        break;
                    }
                    splitmap[c(x, y)] = true;
                    light(Left, x, y, map, lightmap, splitmap);
                    dir = Right;
                }
            },
            '|' => match dir {
                Up => continue,
                Down => continue,
                Left => {
                    if splitmap[c(x, y)] {
                        break;
                    }
                    splitmap[c(x, y)] = true;
                    light(Up, x, y, map, lightmap, splitmap);
                    dir = Down;
                }
                Right => {
                    if splitmap[c(x, y)] {
                        break;
                    }
                    splitmap[c(x, y)] = true;
                    light(Up, x, y, map, lightmap, splitmap);
                    dir = Down;
                }
            },
            _ => {}
        }
    }
}

fn calculate() -> usize {
    let contents = std::fs::read_to_string("input.txt").expect("failed to read input.txt");
    let mut map: Vec<char> = vec![' '; contents.len()];
    for (y, lines) in contents.lines().enumerate() {
        for (x, char) in lines.char_indices() {
            map[c(x as isize, y as isize)] = char;
        }
    }
    let mut lightmap: Vec<bool> = vec![false; contents.len()];
    let mut splitmap: Vec<bool> = vec![false; contents.len()];
    light(
        Direction::Right,
        -1,
        0,
        &mut map,
        &mut lightmap,
        &mut splitmap,
    );
    return lightmap.iter().filter(|x| **x).count();
}

fn main() {
    let result = calculate();
    println!("{result}");
}

#[test]
fn correct_result() {
    assert_eq!(calculate(), 8125);
}
