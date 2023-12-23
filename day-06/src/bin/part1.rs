fn main() {
    // load the input
    let input = include_str!("./input.txt");

    // seperate the input into time and distance arrays
    let mut lines = input.lines().collect::<Vec<&str>>();
    let mut time = lines[0].split("Time:      ").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut distance = lines[1].split("Distance:  ").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut result = 1;
    // iterate through the time array and calculate the possible distances
    for i in 0..time.len() {
        let mut waysToWin = 0;

        // precompute the minimum speed
        let mut speed = (distance[i] / time[i]) + 1;
        let mut timeLeft = time[i] - speed;

        while timeLeft > 0 {
            if speed * timeLeft > distance[i] {
                waysToWin += 1;
            }
            speed += 1;
            timeLeft -= 1;
        }

        result *= waysToWin;
    }

    println!("Result: {}", result);
}
