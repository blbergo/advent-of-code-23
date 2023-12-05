fn main() {
    let input = include_str!("./input.txt");

    let mut sum = 0;

    for line in input.lines() {
        // cut off the "Card X: " part
        let line = line.split(": ").collect::<Vec<&str>>()[1];

        // split up the cards
        let cards = line.split(" | ").collect::<Vec<&str>>();

        // convert each one to an array of ints
        let winning_nums = cards[0]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let my_nums = cards[1]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut winnings = 0;

        // iterate throuh my_nums to see if any of them are in winning_nums
        for num in my_nums {
            if winning_nums.contains(&num) {
                if winnings == 0 {
                    winnings = 1;
                } else {
                    winnings *= 2;
                }
            }
        }

        sum += winnings;
    }

    println!("{}", sum);
}
