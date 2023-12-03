use std::ops::Add;

fn main() {
    let input = include_str!("input.txt");

    let result = process(input);
    println!("Result is: {}", result);
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .map(replace_spelled_numbers2)
        .map(|line| {
            let a: Vec<u32> = line.chars().filter_map(|char| char.to_digit(10)).collect();

            a
        })
        .map(|num_as_str| {
            let first = num_as_str.iter().next().unwrap();

            match num_as_str.last() {
                Some(i) => format!("{first}{i}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .unwrap()
        })
        .sum()
}

fn replace_spelled_numbers2(input: &str) -> String {
    let mut index = 0;

    let mut result = String::new();

    while index < input.len() {
        let reduce_line = &input[index..];

        let char = if reduce_line.starts_with("one") {
            Some('1')
        } else if reduce_line.starts_with("two") {
            Some('2')
        } else if reduce_line.starts_with("three") {
            Some('3')
        } else if reduce_line.starts_with("four") {
            Some('4')
        } else if reduce_line.starts_with("five") {
            Some('5')
        } else if reduce_line.starts_with("six") {
            Some('6')
        } else if reduce_line.starts_with("seven") {
            Some('7')
        } else if reduce_line.starts_with("eight") {
            Some('8')
        } else if reduce_line.starts_with("nine") {
            Some('9')
        } else {
            let result = reduce_line.chars().next();
            result
        };

        if let Some(c) = char {
            result = result.add(&c.to_string());
        }

        index += 1;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn mini() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(process(input), 281);
    }

    #[rstest]
    #[case("1abc2", 12)]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("7pqrstsixteentwone", 71)]
    fn singel_line(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(process(input), expected);
    }
}
