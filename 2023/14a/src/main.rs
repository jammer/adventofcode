fn _printmap(map: &Vec<char>, width: usize) {
    let mut i: usize = 0;
    while i < map.len() {
        for a in i..i + width {
            print!("{}", map[a]);
        }
        println!();
        i += width;
    }
}

fn calculate() -> usize {
    let contents = std::fs::read_to_string("input.txt").expect("failed to read input.txt");
    let mut map: Vec<char> = vec![];
    let width = contents.find("\n").unwrap();
    for c in contents.chars().filter(|x| !x.is_whitespace()) {
        map.push(c);
    }
    for i in 0..map.len() {
        if map[i] == '.' {
            let mut x: usize = i + width;
            while x < map.len() {
                if map[x] == 'O' {
                    map[i] = 'O';
                    map[x] = '.';
                    break;
                }
                if map[x] == '#' {
                    break;
                }
                x += width;
            }
        }
    }
    map.reverse();
    let mut load: usize = 0;
    let mut totalload: usize = 0;
    for (i, c) in map.iter().enumerate() {
        if i % width == 0 {
            load += 1;
        }
        if *c == 'O' {
            totalload += load;
        }
    }
    return totalload;
}

fn main() {
    let result = calculate();
    println!("{}", result);
}

#[test]
fn correct_result() {
    assert_eq!(calculate(), 107951);
}
