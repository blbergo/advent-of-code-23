struct Point {
    x: i32,
    y: i32,
}

struct Segment {
    point: Point,
    x2: i32,
    value: i32,
}

fn main() {
    let input = include_str!("./input.txt");

    let mut numbers: Vec<Segment> = Vec::new();
    let mut symbols: Vec<Point> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut s = String::new();
        let mut length: i32 = 0;

        for (x, c) in line.chars().enumerate() {
            if !c.is_numeric() && c != '.' {
                if length > 0 {
                    numbers.push(Segment {
                        point: Point {
                            x: x as i32 - length,
                            y: y as i32,
                        },
                        x2: x as i32 - 1,
                        value: s.parse::<i32>().unwrap(),
                    });
                    s = String::new();
                    length = 0;
                }

                symbols.push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            } else if c.is_numeric() {
                s.push(c);
                length += 1;
            }
        }

        // Handle number at the end of a line
        if length > 0 {
            numbers.push(Segment {
                point: Point {
                    x: line.len() as i32 - length,
                    y: y as i32,
                },
                x2: line.len() as i32 - 1,
                value: s.parse::<i32>().unwrap(),
            });
        }
    }

    let mut sum = 0;
    for num in &numbers {
        if is_adjacent(&num, &symbols) {
            sum += num.value;
        }
    }

    println!("Sum: {}", sum);
}

fn is_adjacent(num: &Segment, symbols: &[Point]) -> bool {
    for symbol in symbols {
        // Check for adjacency including diagonals
        if (num.point.x - 1..=num.x2 + 1).contains(&symbol.x)
            && (num.point.y - 1..=num.point.y + 1).contains(&symbol.y)
        {
            return true;
        }
    }
    false
}
