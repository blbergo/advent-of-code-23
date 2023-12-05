fn main() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

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
