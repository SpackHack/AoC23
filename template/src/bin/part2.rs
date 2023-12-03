fn main() {
    let input = include_str!("input.txt");

    let result = process(input);

    println!("Result is: {result}");
}

fn process(_input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test() {
        let input = "";

        assert_eq!(process(input), 0);
    }
}
