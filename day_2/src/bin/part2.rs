fn main() {
    let input = include_str!("input.txt");

    let result = process(input);

    println!("Result is: {result}");
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let a: Vec<&str> = line.split(":").collect();

            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            let rounds: Vec<&str> = a.last().unwrap().split(";").collect();

            for round in rounds {
                let colures: Vec<&str> = round.split(",").collect();

                for color in colures {
                    let c: Vec<&str> = color.trim().split(" ").collect();

                    let count = c.first().unwrap().parse::<u32>().unwrap();

                    match *c.last().unwrap() {
                        "red" => {
                            if count > min_red {
                                min_red = count;
                            }
                        }
                        "green" => {
                            if count > min_green {
                                min_green = count;
                            }
                        }
                        "blue" => {
                            if count > min_blue {
                                min_blue = count;
                            }
                        }
                        _ => {
                            panic!("should not happen")
                        }
                    }
                }
            }

            let result = match (min_red, min_green, min_blue) {
                (0, 0, 0) => 0,
                (r, 0, 0) => r,
                (0, g, 0) => g,
                (0, 0, b) => b,
                (r, g, 0) => r * g,
                (r, 0, b) => r * b,
                (0, g, b) => g * b,
                (r, g, b) => r * g * b,
            };

            return result;
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test() {
        let input = include_str!("test_part2.txt");

        assert_eq!(process(input), 2286);
    }
}
