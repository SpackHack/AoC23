fn main() {
    let input = include_str!("input.txt");
    let config = Config {
        red: 12,
        green: 13,
        blue: 14,
    };

    let result = process(input, config);

    println!("Result is: {result}");
}

fn process(input: &str, config: Config) -> u32 {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let a: Vec<&str> = line.split(":").collect();

            let rounds: Vec<&str> = a.last().unwrap().split(";").collect();

            let mut is_possible = true;

            for round in rounds {
                let colures: Vec<&str> = round.split(",").collect();

                for color in colures {
                    let c: Vec<&str> = color.trim().split(" ").collect();

                    let count = c.first().unwrap().parse::<u32>().unwrap();

                    match *c.last().unwrap() {
                        "red" => {
                            if count > config.red {
                                is_possible = false;
                                break;
                            }
                        }
                        "green" => {
                            if count > config.green {
                                is_possible = false;
                                break;
                            }
                        }
                        "blue" => {
                            if count > config.blue {
                                is_possible = false;
                                break;
                            }
                        }
                        _ => {
                            panic!("should not happen")
                        }
                    }
                }
            }

            if is_possible {
                return i as u32 + 1;
            }

            0
        })
        .sum()
}

struct Config {
    red: u32,
    green: u32,
    blue: u32,
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test() {
        let input = include_str!("test_part1.txt");
        let config = Config {
            red: 12,
            green: 13,
            blue: 14,
        };

        assert_eq!(process(input, config), 8);
    }
}
