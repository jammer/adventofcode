fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("unable to read input.txt");
    let mut points: u64 = 0;
    for line in contents.lines() {
        let (winning, card_numbers) = line.split_once(" | ").expect("failed to split");
        let (_, win_numbers) = winning.split_once(": ").expect("failed to split");
        let split: Vec<&str> = win_numbers.split(" ").collect();
        let win_numbers: Vec<u64> = split
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().expect("failed to parse"))
            .collect();
        points += card_numbers
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().expect("failed to parse"))
            .fold(0, |a, x| {
                if win_numbers.contains(&x) {
                    if a == 0 {
                        1
                    } else {
                        a * 2
                    }
                } else {
                    a
                }
            });
    }
    println!("{}", points);
}
