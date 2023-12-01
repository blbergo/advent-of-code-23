fn main() {
    // Load the input data
    let input = include_str!("input1.txt");

    // Create an array for storing the digits as strings
    let mut digits: Vec<Vec<char>> = Vec::new();

    // iterate through the input string
    for (i, line) in input.lines().enumerate() {
        //create a new array for this line
        digits.push(Vec::new());
        // iterate through each char in the line
       for c in line.chars() {
            if c.is_numeric() {
                if digits[i].len() < 2 {
                    digits[i].push(c);
                } else {
                    digits[i][1] = c;
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
        println!("{}", num);
        // add each integer to the sum
        sum += num;
   
    }
    
    // print out the result
    println!("The sum is: {}", sum);
}