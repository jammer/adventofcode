fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("unable to read input.txt");
    let mut cards: Vec<u64> = vec![1; contents.lines().count()];
    for (linenumber, line) in contents.lines().enumerate() {
        let (winning, card_numbers) = line.split_once(" | ").expect("failed to split");
        let (_, win_numbers) = winning.split_once(": ").expect("failed to split");
        let split: Vec<&str> = win_numbers.split(" ").collect();
        let win_numbers: Vec<u64> = split
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().expect("failed to parse"))
            .collect();
        let split: Vec<&str> = card_numbers.split(" ").collect();
        let wins: usize = split
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().expect("failed to parse"))
            .fold(0, |a, x| a + if win_numbers.contains(&x) { 1 } else { 0 });
        let current_count = cards[linenumber];
        let next = linenumber + 1;
        for card in cards[next..next + wins].iter_mut() {
            *card = *card + current_count;
        }
    }
    println!("{}", cards.iter().sum::<u64>());
}
