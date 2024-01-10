use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Rule {
    var: String,
    comp: char,
    amount: i32,
    out: String,
}

fn calculate() -> i32 {
    let mut sum: i32 = 0;
    let contents = std::fs::read_to_string("input.txt").expect("unable to read input file");
    let mut item: HashMap<String, i32> = HashMap::new();
    let mut tasks: HashMap<String, Vec<Rule>> = HashMap::new();
    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("{") {
            item.clear();
            let start = line.find("{").unwrap();
            let end = line.find("}").unwrap();
            let line = line.get(start + 1..end).unwrap();
            for s in line.split(",") {
                let (var, amount) = s.split_once("=").unwrap();
                item.insert(var.to_owned(), amount.parse().unwrap());
            }
            let mut current = "in".to_owned();
            loop {
                'rules: loop {
                    let task = tasks[&current].clone();
                    for rule in task {
                        match rule.comp {
                            '<' => {
                                if item[&rule.var] >= rule.amount {
                                    continue;
                                }
                            }
                            '>' => {
                                if item[&rule.var] <= rule.amount {
                                    continue;
                                }
                            }
                            _ => {}
                        }
                        current = rule.out;
                        break 'rules;
                    }
                    panic!("Ran out of rules!");
                }
                match current.as_str() {
                    "A" => {
                        for x in item.values() {
                            sum += x;
                        }
                        break;
                    }
                    "R" => break,
                    _ => continue,
                }
            }
        } else {
            let start = line.find("{").unwrap();
            let end = line.find("}").unwrap();
            let name = line.get(0..start).unwrap();
            let mut rules: Vec<Rule> = vec![];
            for rule in line.get(start + 1..end).unwrap().split(",") {
                if let Some(x) = rule.find(":") {
                    let comp_pos = rule.find(|c: char | !c.is_alphanumeric() ).unwrap();
                    let comp = rule.chars().nth(comp_pos).unwrap();
                    let var = rule.get(0..comp_pos).unwrap();
                    let amount = rule.get(comp_pos + 1..x).unwrap();
                    let out = rule.get(x + 1..).unwrap();
                    rules.push(Rule {
                        var: var.to_owned(),
                        comp: comp,
                        amount: amount.parse().unwrap(),
                        out: out.to_owned(),
                    });
                } else {
                    rules.push(Rule {
                        var: "".to_owned(),
                        comp: '=',
                        amount: 0,
                        out: rule.to_owned(),
                    });
                }
            }
            tasks.insert(name.to_owned(), rules);
        }
    }
    return sum;
}

fn main() {
    let result = calculate();
    println!("{result}");
}

#[test]
fn correct_result() {
    assert_eq!(calculate(), 346230);
}
