fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("unable to read input.txt");
    let mut sum: u64 = 0;
    for line in contents.lines() {
        sum += line
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
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
