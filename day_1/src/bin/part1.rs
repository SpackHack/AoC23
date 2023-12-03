fn main() {
    let input = include_str!("input.txt");

    let result = process(input);

    println!("Result is: {result}");
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.chars().filter_map(|char| char.to_digit(10)))
        .map(|mut num_as_str| {
            let first = num_as_str.next().unwrap();

            match num_as_str.last() {
                Some(i) => format!("{first}{i}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::process;

    #[test]
    fn test() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(process(input), 142);
    }
}
