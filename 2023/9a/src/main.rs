fn calc(mut nums: Vec<i64>) -> Vec<i64> {
    nums.reverse();
    let mut second: i64 = nums.pop().unwrap();
    let mut newnums: Vec<i64> = vec![];
    let mut allzeroes = true;
    while !nums.is_empty() {
        let first = second;
        second = nums.pop().unwrap();
        newnums.push(second - first);
        if second - first != 0 {
            allzeroes = false;
        }
    }
    if !allzeroes {
        let mut vec = calc(newnums);
        vec.push(vec.last().unwrap() + second);
        return vec;
    }
    vec![second]
}

fn calculate() -> i64 {
    let contents = std::fs::read_to_string("input.txt").expect("unable to read input.txt");
    let mut sum: i64 = 0;
    for line in contents.lines() {
        let nums: Vec<i64> = line
            .split(" ")
            .filter(|x| x.len() > 0)
            .map(|x| x.parse().unwrap())
            .collect();
        let vec = calc(nums);
        sum += vec.last().unwrap();
    }
    return sum;
}

fn main() {
    let result = calculate();
    println!("{result}");
}

#[test]
fn correct_result() {
    assert_eq!(calculate(), 1938800261);
}
