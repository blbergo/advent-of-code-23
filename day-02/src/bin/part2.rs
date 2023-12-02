use std::collections::HashMap;

fn main() {
    // load the input data
    let input = include_str!("input1.txt");

    // create a variable to keep track of the sum of ids
    let mut sum = 0;

    // iterate through each line in the input data
    for (i, line) in input.lines().enumerate() {
        // create a hashmap to store the maximum number of each color cube
        let mut max_colors: HashMap<String, i32> = HashMap::from([
            ("red".to_string(), 1),
            ("green".to_string(), 1),
            ("blue".to_string(), 1),
        ]);

        let mut start_char = 8;

        // adjust depending on the number of digits in the id
        if i > 8 && i <= 98 {
            start_char += 1;
        } else if i == 99 {
            start_char += 2;
        }
        // remove the game id from the line
        let mut data: String = line.chars().skip(start_char).collect();

        // replace the semicolons with commas
        data = data.replace(";", ",");

        // split the data into a vector
        let mut arr: Vec<String> = data.split(",").map(|x| x.to_string()).collect();

        // trim any extra whitespace
        arr = arr.iter().map(|x| x.trim().to_string()).collect();

        // iterate through each element in data
        for color in arr.iter() {
            // split the string into an int and a color
            let data: Vec<String> = color.split(" ").map(|x| x.to_string()).collect();
            let number_of_cubes = data[0].parse::<i32>().unwrap();

            let color = data[1].clone();

            // check if the number of cubes is greater than the max for the given color
            if number_of_cubes > max_colors[&color] {
                max_colors.insert(color, number_of_cubes);
            }
        }

        // calculate the power
        let power = max_colors["red"] * max_colors["green"] * max_colors["blue"];

        // add it to the sum
        sum += power;
    }

    println!("The sum is: {}", sum);
}
