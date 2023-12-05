// create a struct to represent a point
struct Point {
    x: i32,
    y: i32,
}

// create a struct to represent a line segment
struct Segment {
    point: Point,
    // only need x2 because numbers read left to right
    x2: i32,
    value: i32,
}
fn main() {
    // load the input data
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let mut numbers: Vec<Segment> = Vec::new();
    let mut symbols: Vec<Point> = Vec::new();

    // iterate over the input data
    for (y, line) in input.lines().enumerate() {
        let mut s = String::new();
        let mut length: i32 = 0;
        // iterate over each char in a line
        for (x, c) in line.chars().enumerate() {
            // check if the char is a symbol
            if !c.is_numeric() && c != '.' {
                // check if there is a number before the symbol
                if length > 0 {
                    numbers.push(Segment {
                        point: Point {
                            x: x as i32 - length - 1,
                            y: y as i32,
                        },
                        x2: x as i32 - 1,
                        value: s.parse::<i32>().unwrap(),
                    });
                }

                // add the symbol to the list
                symbols.push(Point {
                    x: x as i32,
                    y: y as i32,
                });

                // clear the current num string
                s = String::new();
                length = 0;
            } else {
                // add the number to the current num string
                if c.is_numeric() {
                    s.push(c);
                    length += 1;
                } else {
                    if length > 0 {
                        numbers.push(Segment {
                            point: Point {
                                x: x as i32 - length,
                                y: y as i32,
                            },
                            x2: x as i32 - 1,
                            value: s.parse::<i32>().unwrap(),
                        });
                    }

                    // clear the current num string
                    s = String::new();
                    length = 0;
                }
            }
        }
    }

    let mut sum = 0;

    for num in numbers {
        if is_adjacent(&num, &symbols) {
            sum += num.value;
        }
    }

    println!("Sum: {}", sum);
}

fn is_adjacent(num: &Segment, symbols: &Vec<Point>) -> bool {
    for symbol in symbols {
        // check the top and bottom and corners
        if symbol.x >= num.point.x - 1 && symbol.x <= num.x2 + 1 && (symbol.y == num.point.y + 1 || symbol.y == num.point.y -1) {
            return true;
        }

        //check the left and right
        if symbol.y == num.point.y && (symbol.x == num.point.x - 1 || symbol.x == num.x2 + 1) {
            return true;
        }
    }

    println!("No adjacent symbol found for {:?}", num.value);

    return false;
}
