use std::collections::HashMap;

fn main() {
    // Load the input data
    let input = include_str!("input1.txt");

    // Create an array for storing the digits as strings
    let mut digits: Vec<Vec<char>> = Vec::new();

    // create a hashmap to store the word version of numbers
    let words: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')

    ]);

    // iterate through the input string
    for (i, line) in input.lines().enumerate() {
        //create a new array for this line in digits
        digits.push(Vec::new());

        // create a new array for the non-digits
        let mut chars:Vec<char> = Vec::new();

        // iterate through each char in the line
       for c in line.chars() {
            if c.is_numeric() {
                if digits[i].len() < 2 {
                    digits[i].push(c);
                } else {
                    digits[i][1] = c;
                }

                chars.clear()
            } else {
                chars.push(c);
            }

            // iterate through the hashmap to see if the char array is one of the numeric words
            for (word, digit) in &words {
                if chars.iter().collect::<String>().contains(word) {
                    if digits[i].len() < 2 {
                        digits[i].push(*digit);
                    } else {
                        digits[i][1] = *digit;
                    }

                    // keep track of the last char since it can be the start of a new sequence
                    let last_char = chars[chars.len() - 1];
                    chars.clear();
                    chars.push(last_char);
                }
            }
       }

       // if there is only one digit, copy it to the second digit
       if digits[i].len() < 2 {
            let digit = digits[i][0];
            digits[i].push(digit);
       }
    }
 
    // sum each item together
    let mut sum = 0;

    // iterate through the digits array
    for number in digits {
        // convert each string to an integer
        let s:String = number.iter().collect();
        let num: i32 = s.parse().unwrap();
        // add each integer to the sum
        sum += num;
   
    }
    
    // print out the result
    println!("The sum is: {}", sum);
}