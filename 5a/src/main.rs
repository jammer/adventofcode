#[derive(Debug, Copy, Clone, PartialOrd, Ord, Eq, PartialEq)]
struct Map {
    source: u64,
    destination: u64,
    range: u64,
}

fn usemap(srcs: Vec<u64>, mut dsts: Vec<Map>) -> Vec<u64> {
    dsts.sort();
    let mut map: Vec<u64> = vec![];
    'seed: for src in srcs {
        for dst in &dsts {
            if src < dst.source {
                map.push(src);
                continue 'seed;
            }
            if src < dst.source + dst.range {
                map.push(src - dst.source + dst.destination);
                continue 'seed;
            }
        }
        map.push(src);
    }
    return map;
}

fn loadmap(lines: &mut std::str::Lines<'_>) -> Vec<Map> {
    lines.next();
    let mut line = lines.next().expect("Map missing");
    let mut maps: Vec<Map> = vec![];
    while line != "" {
        let split: Vec<u64> = line
            .split(" ")
            .map(|x| x.parse().expect("Cannot parse u64"))
            .collect();
        maps.push(Map {
            destination: split[0],
            source: split[1],
            range: split[2],
        });

        line = lines.next().unwrap_or("");
    }
    return maps;
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Unable to read input.txt");
    let mut lines = contents.lines();
    let seeds = lines.next().expect("Input is empty");
    let mut srcs: Vec<u64> = seeds
        .split(" ")
        .skip(1)
        .map(|x| x.parse().expect("Cannot parse u64"))
        .collect();
    lines.next();

    for _ in 0..7 {
        srcs = usemap(srcs, loadmap(&mut lines));
    }

    srcs.sort();
    println!("{}", srcs.first().expect("Result is empty"));
}
