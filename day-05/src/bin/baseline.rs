use std::collections::HashMap;

fn main() {
    // load the input data
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    // parse the input data into a usable format
    let lines = input.lines().collect::<Vec<&str>>();

    // parse the seeds
    let seeds = lines[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    // parse the maps
    let mut i = 0;
    let mut maps: Vec<HashMap<i32, i32>> = Vec::new();

    while i < lines.len() {
        if lines[i].contains(" map:") {
            let mut j = i;
            j += 1;
            while j < lines.len() && lines[j] != "" {
                j += 1;
            }

            maps.push(parse_lines(&lines[i + 1..j]));
        }
        i += 1;
    }

    let mut destinations: Vec<i32> = Vec::new();

    // run the seeds through each map
    for seed in seeds {
        let mut value = seed as i32;
        for map in &maps {
            if map.contains_key(&value) {
                value = map[&value];
            }
        }
        destinations.push(value);
    }

    // find the minimum value
    let mut min = destinations[0];

    for dest in destinations {
        if dest < min {
            min = dest;
        }
    }

    println!("The minimum value is {}", min);
}

fn parse_lines(lines: &[&str]) -> HashMap<i32, i32> {
    // cut out the first line
    let mut map: HashMap<i32, i32> = HashMap::new();

    for line in lines {
        let line = line.split_whitespace().collect::<Vec<&str>>();

        let destination = line[0].parse::<i32>().unwrap();
        let source = line[1].parse::<i32>().unwrap();
        let length = line[2].parse::<i32>().unwrap();

        for i in 0..length {
            map.insert(source + i, destination + i);
        }
    }

    return map;
}
