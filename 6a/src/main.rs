fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Unable to read input.txt");
    let mut lines = contents.lines();
    let times_str = lines.next().unwrap();
    let distances_str = lines.next().unwrap();
    let mut times: Vec<i32> = vec![];
    let mut distances: Vec<i32> = vec![];
    times_str
        .trim()
        .trim_start_matches("Time:")
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .for_each(|x| times.push(x));
    distances_str
        .trim_start_matches("Distance:")
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .for_each(|x| distances.push(x));
    let mut all: i32 = 1;
    for i in 0..times.len() {
        let mut sum: i32 = 0;
        let minspeed = distances[i] / times[i];
        for t in minspeed..times[i] {
            let start = times[i];
            let timeleft = start - t;
            if distances[i] < timeleft * t {
                sum += timeleft - t + 1;
                break;
            }
        }
        all *= sum;
    }
    println!("{all}");
}
