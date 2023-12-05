fn main() {
    let input = include_str!("./input.txt");

    // keeps track of how many times a card has been seen
    let mut cards: Vec<i32> = Vec::new();

    for line in input.lines() {
        // cut off the "Card X: " part
        let line = line.split(": ").collect::<Vec<&str>>()[1];

        // split up the cards
        let _cards = line.split(" | ").collect::<Vec<&str>>();

        // convert each one to an array of ints
        let winning_nums = _cards[0]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let my_nums = _cards[1]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut count = 0;

        // iterate throuh my_nums to see if any of them are in winning_nums
        for num in my_nums {
            if winning_nums.contains(&num) {
                count += 1;
            }
        }

        let mut n = count;
        while count > 0 {
            while n > 0 {
                cards.push(n);
                n -= 1;
            }
            count -= 1;
        }
    }

    println!("{}", cards.len());
}
