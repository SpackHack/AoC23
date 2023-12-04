fn main() {
    let input = include_str!("input.txt");

    let result = process(input);

    println!("Result is: {result}");
}

fn process(input: &str) -> u32 {
    let mut lines: Vec<(&str, u32)> = Vec::new();

    for line in input.lines() {
        lines.push((line, 1));
    }

    for i in 0..lines.len() {
        let mut line_points = 0;

        let line = lines.get(i).unwrap().0;

        let a: Vec<&str> = line.split(":").collect();

        let b: Vec<&str> = a.last().unwrap().split("|").collect();

        let winning_numbers: Vec<&str> = b.first().unwrap().trim().split(" ").collect();
        let numbers: Vec<&str> = b.last().unwrap().trim().split(" ").collect();

        for number in numbers {
            if number.is_empty() {
                continue;
            }

            if winning_numbers.contains(&number) {
                line_points += 1;
            }
        }

        if line_points > 0 {
            for l in 1..=line_points {
                if (i + l) >= lines.len() {
                    break;
                }

                let count = lines.get(i + l).unwrap().1;
                let new = lines.get(i).unwrap().1 + count;

                lines.get_mut(i + l).unwrap().1 = new;
            }
        }
    }

    let mut result = 0;

    for l in lines {
        result += l.1;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test() {
        let input = include_str!("test_part2.txt");

        assert_eq!(process(input), 30);
    }
}
