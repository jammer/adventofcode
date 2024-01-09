#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Map<'a> {
    map: Vec<&'a str>,
}

impl Map<'_> {
    fn get(&self, x: usize, y: usize) -> char {
        return self.map[y].chars().nth(x).unwrap();
    }

    fn travel(&self, x: usize, y: usize, dir: Direction) -> Option<(usize, usize, Direction)> {
        use Direction::*;
        return match dir {
            Up => match self.get(x, y) {
                '|' => Some((x, y - 1, Up)),
                '7' => Some((x - 1, y, Left)),
                'F' => Some((x + 1, y, Right)),
                _ => None,
            },
            Down => match self.get(x, y) {
                '|' => Some((x, y + 1, Down)),
                'J' => Some((x - 1, y, Left)),
                'L' => Some((x + 1, y, Right)),
                _ => None,
            },
            Left => match self.get(x, y) {
                '-' => Some((x - 1, y, Left)),
                'F' => Some((x, y + 1, Down)),
                'L' => Some((x, y - 1, Up)),
                _ => None,
            },
            Right => match self.get(x, y) {
                '-' => Some((x + 1, y, Right)),
                '7' => Some((x, y + 1, Down)),
                'J' => Some((x, y - 1, Up)),
                _ => None,
            },
        };
    }

    fn findstart(&self) -> (usize, usize) {
        for (y, line) in self.map.iter().enumerate() {
            for (x, c) in line.char_indices() {
                if c == 'S' {
                    return (x as usize, y as usize);
                }
            }
        }
        panic!("start not found");
    }
}

fn main() {
    use Direction::*;
    let contents = std::fs::read_to_string("input.txt").expect("unable to read input.txt");
    let map = Map {
        map: contents.lines().collect(),
    };
    let (startx, starty) = map.findstart();
    let mut starts: Vec<(usize, (usize, usize, Direction))> = vec![];
    for (x, y, dir) in [
        (startx, starty - 1, Up),
        (startx, starty + 1, Down),
        (startx - 1, starty, Left),
        (startx + 1, starty, Right),
    ] {
        if let Some(start) = map.travel(x, y, dir) {
            starts.push((2, start));
        }
    }
    loop {
        let (asteps, (ax, ay, adir)) = starts.pop().unwrap();
        let (bsteps, (bx, by, bdir)) = starts.pop().unwrap();
        if asteps > 0 {
            if ax == bx && ay == by {
                println!("{asteps}");
                break;
            }
        }
        starts.push((asteps + 1, map.travel(ax, ay, adir).unwrap()));
        starts.push((bsteps + 1, map.travel(bx, by, bdir).unwrap()));
    }
}
