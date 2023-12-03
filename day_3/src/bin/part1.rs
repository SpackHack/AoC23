fn main() {
    let input = include_str!("input.txt");

    let result = process(input);

    println!("Result is: {result}");
}

fn process(input: &str) -> u32 {
    let x = input.lines().collect::<Vec<&str>>().len();
    let y = input.lines().collect::<Vec<&str>>().first().unwrap().len();

    let mut mask = vec![vec![false; y]; x];

    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            match char {
                '*' | '@' | '/' | '#' | '%' | '&' | '$' | '-' | '=' | '+' => {
                    mark_mask(&mut mask, x, y);
                }
                _ => {}
            }
        }
    }

    let mut result = 0;

    for (x, line) in input.lines().enumerate() {
        let mut num = String::new();
        let mut is_adjacent = false;

        for (y, char) in line.chars().enumerate() {
            if char.is_numeric() {
                num.push(char);

                if mask[x][y] {
                    is_adjacent = true;
                }
            } else {
                if is_adjacent {
                    result += num.parse::<u32>().unwrap();
                }
                num = String::new();
                is_adjacent = false;
            }
        }

        if is_adjacent {
            result += num.parse::<u32>().unwrap();
        }
    }

    result
}

fn mark_mask(mask: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    mask[x][y] = true;

    if x > 0 && y > 0 {
        mask[x - 1][y - 1] = true;
    }

    if x > 0 {
        mask[x - 1][y] = true;
    }

    if x > 0 && y < mask.len() - 1 {
        mask[x - 1][y + 1] = true;
    }

    if y < mask.len() - 1 {
        mask[x][y + 1] = true;
    }

    if x < mask[0].len() - 1 && y < mask.len() - 1 {
        mask[x + 1][y + 1] = true;
    }

    if x < mask[0].len() - 1 {
        mask[x + 1][y] = true;
    }

    if x < mask[0].len() - 1 && y > 0 {
        mask[x + 1][y - 1] = true;
    }

    if y > 0 {
        mask[x][y - 1] = true;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test() {
        let input = include_str!("test_part1.txt");

        assert_eq!(process(input), 4361);
    }

    #[rstest]
    fn test1() {
        let input = include_str!("test_part1_1.txt");

        assert_eq!(process(input), 2);
    }
}
