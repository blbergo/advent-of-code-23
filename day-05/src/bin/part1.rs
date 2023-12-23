use std::collections::HashMap;
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

fn main() {
    // load the input data
    let input = include_str!("./input.txt");

// parse the input data into a usable format
let lines: Vec<&str> = input.split("\n").collect();

// parse the seeds
let seeds = lines[0].split(": ").nth(1).unwrap().split(" ").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();

// parse the maps
let mut maps:Vec<HashMap<i32, i32>> = Vec::new();

let mut i = 0;

while i < lines.len() {
    
    if lines[i].contains(" map:") {
        i += 1;
        let j = i;
        while i < lines.len() && lines[i] != "" {
            i += 1;
        }

    println!("Parsing started for lines {} to {}", j, i);
       maps.push(parse_lines(lines[j..i].to_vec()));
    }
    i += 1;
}



let mut destinations:Vec<i32> = Vec::new();

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

fn parse_lines(lines: Vec<&str>)-> HashMap<i32, i32> {
    // cut out the first line
    let mut map:HashMap<i32, i32> = HashMap::new();


    for line in lines {
        let line = line.split_whitespace().collect::<Vec<&str>>();

        let destination = line[0].parse::<i32>().unwrap();
        let source = line[1].parse::<i32>().unwrap();
        let length = line[2].parse::<i32>().unwrap();

        for i in 0..length {
            print!("\r{:.5}%", i as f32 / length as f32 * 100.0);

            stdout().flush().unwrap();
            sleep(Duration::from_millis(20));
            
            map.insert(source + i, destination + i, );
        }
    }

    return map;
}