#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    Full,
    ThreeOfKind,
    TwoPairs,
    OnePair,
    High,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Cards {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Jack,
    Jack2,
    Jack3,
    Jack4,
    Jack5,
}

impl std::fmt::Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Handy {
    handtype: HandType,
    first: Cards,
    second: Cards,
    third: Cards,
    fourth: Cards,
    fifth: Cards,
    bid: u64,
}

impl std::fmt::Display for Handy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:5} {}{}{}{}{} {:4}",
            self.handtype.to_string(),
            self.first,
            self.second,
            self.third,
            self.fourth,
            self.fifth,
            self.bid
        )
    }
}

fn get_card(c: &mut std::str::Chars) -> Cards {
    use Cards::*;
    return match c.next().unwrap() {
        'A' => Ace,
        'K' => King,
        'Q' => Queen,
        'J' => Jack,
        'T' => Ten,
        '9' => Nine,
        '8' => Eight,
        '7' => Seven,
        '6' => Six,
        '5' => Five,
        '4' => Four,
        '3' => Three,
        _ => Two,
    };
}

fn gethandtype(a:Cards,mut b:Cards, mut c:Cards, mut d:Cards, mut e:Cards) -> HandType {
    use Cards::*;
    use HandType::*;
    fn upgrade(card: HandType) -> HandType {
        return match card {
            High => OnePair,
            OnePair => ThreeOfKind,
            TwoPairs => Full,
            ThreeOfKind => FourOfKind,
            Full => FourOfKind,
            FourOfKind => FiveOfKind,
            FiveOfKind => FiveOfKind,
        };
    }
    let mut handtype:HandType = High;
            if b == Jack {
                b = Jack2;
            }
            if c == Jack {
                c = Jack3;
            }
            if d == Jack {
                d = Jack4;
            }
            if e == Jack {
                e = Jack5;
            }
            if (a == b) || (b == c) || (c == d) || (d == e) {
                handtype = OnePair;
            }
            if (a == b && c == d) || (a == b && d == e) || (b == c && d == e) {
                handtype = TwoPairs;
            }
            if (a == b && a == c) || (b == c && b == d) || (c == d && c == e) {
                handtype = ThreeOfKind;
            }
            if (a == b && a == c && d == e) || (c == d && c == e && a == b) {
                handtype = Full;
            }
            if (a == b && a == c && a == d) || (e == b && e == c && e == d) {
                handtype = FourOfKind;
            }
            if a == b && a == c && a == d && a == e {
                handtype = FiveOfKind;
            }
            if b == Jack2 {
                handtype = upgrade(handtype);
            }
            if c == Jack3 {
                handtype = upgrade(handtype);
            }
            if d == Jack4 {
                handtype = upgrade(handtype);
            }
            if e == Jack5 {
                handtype = upgrade(handtype);
            }
            return handtype;
}

fn calculate() -> u64 {
    let contents = std::fs::read_to_string("input.txt").expect("unable to read input.txt");
    let mut hands: Vec<Handy> = vec![];
    for c in contents.lines() {
        if let Some((hand, bid)) = c.split_once(" ") {
            let mut c = hand.chars();
            let mut handy = Handy {
                handtype: HandType::High,
                bid: bid.parse().expect(""),
                first: get_card(&mut c),
                second: get_card(&mut c),
                third: get_card(&mut c),
                fourth: get_card(&mut c),
                fifth: get_card(&mut c),
            };
            let mut sorted: Vec<Cards> = vec![];
            sorted.push(handy.first);
            sorted.push(handy.second);
            sorted.push(handy.third);
            sorted.push(handy.fourth);
            sorted.push(handy.fifth);
            sorted.sort();
            handy.handtype = gethandtype(sorted[0], sorted[1], sorted[2], sorted[3], sorted[4]);
            hands.push(handy);
        }
    }
    hands.sort();
    let mut rank: u64 = 0;
    let mut sum: u64 = 0;
    for hand in hands.iter().rev() {
        rank = rank + 1;
        let value = hand.bid * rank;
        sum += value;
    }
    return sum;
}

fn main() {
    let sum = calculate();
    println!("{}", sum);
}

#[test]
fn correct_result() {
    assert_eq!(calculate(), 250665248);
}
