use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string("input.txt")?;
    let length: usize = contents.lines().next().expect("empty input").len();
    let mut location: Vec<i32> = vec![-1; contents.len()];
    let mut numbers: Vec<i32> = vec![];
    let mut symbols: Vec<(usize, usize)> = vec![];
    for (y, line) in contents.lines().enumerate() {
        let mut reading = false;
        let mut number = "".to_owned();
        for (x, char) in line.char_indices() {
            if char.is_digit(10) {
                location[y * length + x] = numbers.len() as i32;
                reading = true;
                number.push(char);
                continue;
            }
            if reading {
                numbers.push(number.parse()?);
                number.clear();
            }
            reading = false;
            if char == '*' {
                symbols.push((x, y));
            }
        }
        if reading {
            numbers.push(number.parse()?);
            number.clear();
        }
    }
    let mut sum: i32 = 0;
    for (x, y) in symbols {
        let mut mynumbers: Vec<usize> = vec![];
        for (x, y) in [
            (x - 1, y - 1),
            (x, y - 1),
            (x - 1, y),
            (x + 1, y - 1),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ] {
            let loc = location[y * length + x];
            if loc != -1 {
                mynumbers.push(loc as usize);
            }
        }
        mynumbers.sort();
        mynumbers.dedup();
        if mynumbers.len() == 2 {
            sum += mynumbers.iter().fold(1, |a, x| a * numbers[*x]);
        }
    }
    println!("{sum}");
    Ok(())
}
