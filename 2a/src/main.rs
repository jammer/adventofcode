fn main() {
    const RED: u64 = 12;
    const GREEN: u64 = 13;
    const BLUE: u64 = 14;

    let contents = std::fs::read_to_string("input.txt").expect("unable to read input.txt");
    let mut sum: u64 = 0;
    'outer: for line in contents.lines() {
        let (id, balls) = line.trim_start_matches("Game ").split_once(": ").expect("unable to parse");
        let balls = balls.replace(";", ",");
        let balls = balls.split(",");
        for ball in balls {
            let (count, color) = ball.trim().split_once(" ").expect("unable to parse");
            let count: u64 = count.parse().expect("unable to parse");
            if count
                > match color {
                    "green" => GREEN,
                    "red" => RED,
                    "blue" => BLUE,
                    _ => 0,
                }
            {
                continue 'outer;
            }
        }
        sum += id.parse::<u64>().unwrap();
    }
    println!("{sum}");
}
