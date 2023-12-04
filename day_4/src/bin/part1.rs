fn main() {
    let input = include_str!("input.txt");

    let result = process(input);

    println!("Result is: {result}");
}

fn process(input: &str) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        let mut line_points = 0;

        let a: Vec<&str> = line.split(":").collect();

        let b: Vec<&str> = a.last().unwrap().split("|").collect();

        let winning_numbers: Vec<&str> = b.first().unwrap().trim().split(" ").collect();
        let numbers: Vec<&str> = b.last().unwrap().trim().split(" ").collect();

        for number in numbers {
            if number.is_empty() {
                continue;
            }

            if winning_numbers.contains(&number) {
                if line_points == 0 {
                    line_points = 1;
                } else {
                    line_points = line_points * 2;
                }
            }
        }

        result += line_points;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test() {
        let input = include_str!("test_part1.txt");

        assert_eq!(process(input), 13);
    }
}
