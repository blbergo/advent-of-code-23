fn main() {
    // load the input
    let input = include_str!("./input.txt");

    // seperate the input into a time and distance int
    let mut lines = input.lines().collect::<Vec<&str>>();
    let mut time = lines[0].split("Time:      ").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    let mut distance = lines[1].split("Distance:  ").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("").parse::<i64>().unwrap();

    println!("Time: {}", time);
    println!("Distance: {}", distance);

   let mut waysToWin = 0;

    // precompute the minimum speed
    let mut speed = (distance / time) + 1;
    let mut timeLeft = time - speed;

    while timeLeft > 0 {
        if speed * timeLeft > distance {
            waysToWin += 1;
        }
        speed += 1;
        timeLeft -= 1;
    }

    println!("Result: {}", waysToWin); 
}
