fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Unable to read input.txt");
    let mut lines = contents.lines();
    let time_str = lines
        .next()
        .unwrap()
        .trim_start_matches("Time:")
        .replace(" ", "");
    let distance_str = lines
        .next()
        .unwrap()
        .trim_start_matches("Distance:")
        .replace(" ", "");
    let time: i64 = time_str.parse().unwrap();
    let distance: i64 = distance_str.parse().unwrap();
    let mut sum: i64 = 0;
    let minspeed = distance / time;
    for t in minspeed..time {
        let start = time;
        let timeleft = start - t;
        if distance < timeleft * t {
            sum = timeleft - t + 1;
            break;
        }
    }
    println!("{sum}");
}
