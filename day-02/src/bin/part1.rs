use std::collections::HashMap;

fn main() {
    // load the input data
    let input = include_str!("input1.txt");

    // create a hashmap to store the maximum number of each color cube
    let max_colors: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    // create a variable to keep track of the sum of ids
    let mut sum = 0;

    // iterate through each line in the input data
    for (i, line) in input.lines().enumerate() {
        let mut is_valid = true;

        let mut start_char = 8;

        // adjust depending on the number of digits in the id
        if i > 8 && i <= 98 {
            start_char += 1;
        } else if i == 99 {
            start_char += 2;
        }
        // remove the game id from the line
        let data: String = line.chars().skip(start_char).collect();

        // replace the semicolons with commas
        let data = data.replace(";", ",");

        // split the data into a vector
        let data: Vec<&str> = data.split(",").collect();

        // trim any extra whitespace
        let data: Vec<&str> = data.iter().map(|x| x.trim()).collect();

        // iterate through each element in data
        for (_j, color) in data.iter().enumerate() {
            // split the string into an int and a color
            let _data: Vec<&str> = color.split(" ").collect();
            let number_of_cubes = _data[0].parse::<i32>().unwrap();
            let color = _data[1];

            // check if the number of cubes is greater than the max for the given color
            if number_of_cubes > max_colors[color] {
                is_valid = false;
            }
        }

        if is_valid {
            // add the id to the sum
            sum += i + 1;
        }
    }

    println!("The sum is: {}", sum);
}
