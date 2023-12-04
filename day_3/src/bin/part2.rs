fn main() {
    let input = include_str!("input.txt");

    let result = process(input);

    println!("Result is: {result}");
}

fn process(input: &str) -> u64 {
    let x = input.lines().collect::<Vec<&str>>().len();
    let y = input.lines().collect::<Vec<&str>>().first().unwrap().len();

    let mut mask: Vec<Vec<(u64, i64, C)>> = vec![vec![(0, -1, C::A); y]; x];

    let mut result: u64 = 0;

    let mut num = String::new();
    let mut num_id = 0;

    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            if char.is_numeric() {
                num.push(char);
            } else {
                if let Ok(i) = num.parse::<u64>() {
                    result += i;
                };
            }

            if result > 0 {
                for i in 0..num.len() {
                    mark_mask(&mut mask, x, y - 1 - i, result, num_id);
                }
                num_id += 1;
                num = String::new();
                result = 0;
            }
        }
        if !num.is_empty() {
            if let Ok(i) = num.parse::<u64>() {
                result += i;
            };
            for i in 0..num.len() {
                mark_mask(&mut mask, x, y - 1 - i, result, num_id);
            }
            num_id += 1;
            num = String::new();
            result = 0;
        }
    }

    let mut result = 0;

    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            match char {
                '*' => {
                    if let C::B = mask[x][y].2 {
                        result += mask[x][y].0
                    }
                }
                _ => {}
            }
        }
    }

    result
}

fn mark_mask(
    mask: &mut Vec<Vec<(u64, i64, C)>>,
    x: usize,
    y: usize,
    input: u64,
    num_id: i64,
) -> i64 {
    insert(mask, x, y, input, num_id);

    if x > 0 && y > 0 {
        insert(mask, x - 1, y - 1, input, num_id);
    }

    if x > 0 {
        insert(mask, x - 1, y, input, num_id);
    }

    if x > 0 && y < mask.len() - 1 {
        insert(mask, x - 1, y + 1, input, num_id);
    }

    if y < mask.len() - 1 {
        insert(mask, x, y + 1, input, num_id);
    }

    if x < mask[0].len() - 1 && y < mask.len() - 1 {
        insert(mask, x + 1, y + 1, input, num_id);
    }

    if x < mask[0].len() - 1 {
        insert(mask, x + 1, y, input, num_id);
    }

    if x < mask[0].len() - 1 && y > 0 {
        insert(mask, x + 1, y - 1, input, num_id);
    }

    if y > 0 {
        insert(mask, x, y - 1, input, num_id);
    }

    num_id
}

fn insert(mask: &mut Vec<Vec<(u64, i64, C)>>, x: usize, y: usize, input: u64, num_id: i64) {
    if mask[x][y].1 != num_id {
        mask[x][y].1 = num_id;
        if mask[x][y].0 == 0 {
            mask[x][y].0 = input;
        } else {
            mask[x][y].0 = mask[x][y].0 * input;

            if let C::B = mask[x][y].2 {
                mask[x][y].2 = C::C;
            } else {
                mask[x][y].2 = C::B;
            }
        }
    }
}

#[derive(Clone)]
enum C {
    A,
    B,
    C,
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test() {
        let input = include_str!("test_part2.txt");

        assert_eq!(process(input), 467835);
    }

    #[rstest]
    fn test2() {
        let input = include_str!("test_part2_2.txt");

        assert_eq!(process(input), 100);
    }
}
