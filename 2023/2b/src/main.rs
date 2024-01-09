fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("unable to read input.txt");
    let mut sum: u64 = 0;
    for line in contents.lines() {
        let (_, balls) = line.split_once(": ").expect("unable to parse");
        let balls = balls.replace(";", ",");
        let balls = balls.split(",");
        let mut blue: u64 = 0;
        let mut red: u64 = 0;
        let mut green: u64 = 0;
        for ball in balls {
            let (count, color) = ball.trim().split_once(" ").expect("unable to parse");
            let count: u64 = count.parse().expect("unable to parse");
            match color {
                "green" => {
                    if green < count {
                        green = count;
                    }
                }
                "red" => {
                    if red < count {
                        red = count;
                    }
                }
                "blue" => {
                    if blue < count {
                        blue = count;
                    }
                }
                _ => {}
            }
        }
        sum += green * red * blue;
    }
    println!("{sum}");
}
