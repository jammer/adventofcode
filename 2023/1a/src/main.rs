fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("unable to read input.txt");
    let mut sum: u64 = 0;
    for line in contents.lines() {
        sum += line
            .chars()
            .filter(|x| x.is_numeric())
            .fold("".to_owned(), |mut acc, c| {
                if acc.is_empty() {
                    acc.push(c);
                    return acc.repeat(2);
                }
                acc.pop();
                acc.push(c);
                acc
            })
            .parse::<u64>()
            .expect("not valid number");
    }
    println!("{}", sum);
}
