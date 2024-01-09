fn calculate() -> i32 {
    let contents = std::fs::read_to_string("input.txt").expect("Unable to read input.txt");
    let mut sum: i32 = 0;
    let mut current: i32 = 0;
    for c in contents.chars().filter(|x: &char| !x.is_whitespace()) {
        if c == ',' {
            sum += current;
            current = 0;
            continue;
        }
        current += c as i32;
        current *= 17;
        current %= 256;
    }
    return sum + current;
}

fn main() {
    let result = calculate();
    println!("{result}");
}

#[test]
fn correct_result() {
    assert_eq!(calculate(), 510792);
}
