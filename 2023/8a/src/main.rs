use std::collections::HashMap;

fn calculate() -> u64 {
    let contents = std::fs::read_to_string("input.txt").expect("unable to read input.txt");
    let mut lines = contents.lines();
    let directions = lines.next().expect("");
    lines.next();
    let mut lefts: HashMap<String, String> = HashMap::new();
    let mut rights: HashMap<String, String> = HashMap::new();
    while let Some(line) = lines.next() {
        let line = line.replace("(", "").replace(")", "");
        let (id, edges) = line.trim().split_once(" =").expect("");
        let (left, right) = edges.trim().split_once(", ").expect("");
        lefts.insert(id.to_string(), left.to_string());
        rights.insert(id.to_string(), right.to_string());
    }
    let mut step: u64 = 0;
    let mut current = "AAA".to_string();
    for d in directions.chars().cycle() {
        if current == "ZZZ" {
            return step;
        }
        step += 1;
        if d == 'L' {
            current = lefts.get(&current).unwrap().to_string();
        } else {
            current = rights.get(&current).unwrap().to_string();
        }
    }
    return 0;
}

fn main() {
    let result = calculate();
    println!("{result}");
}

#[test]
fn correct_result() {
    assert_eq!(calculate(),17287);
}